use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core::metrics::{Finger, FingerStats, TypingMetrics, KeyboardRow, ExtendedStats};
use log::{info, warn};
use chrono::{DateTime, Utc};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccumulatedStats {
    pub total_quotes: usize,
    pub total_keystrokes: usize,
    pub total_correct_keystrokes: usize,
    pub total_errors: usize,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub number_row_stats: ExtendedStats,
    pub top_row_stats: ExtendedStats,
    pub home_row_stats: ExtendedStats,
    pub bottom_row_stats: ExtendedStats,
    pub finger_stats: HashMap<Finger, ExtendedStats>,
    pub key_averages: HashMap<char, ExtendedStats>,
}

impl AccumulatedStats {
    pub fn new() -> Self {
        Self {
            total_quotes: 0,
            total_keystrokes: 0,
            total_correct_keystrokes: 0,
            total_errors: 0,
            avg_wpm: 0.0,
            avg_accuracy: 0.0,
            number_row_stats: ExtendedStats::new(),
            top_row_stats: ExtendedStats::new(),
            home_row_stats: ExtendedStats::new(),
            bottom_row_stats: ExtendedStats::new(),
            finger_stats: HashMap::new(),
            key_averages: HashMap::new(),
        }
    }

    pub fn load_from_directory() -> Self {
        let mut stats = AccumulatedStats::new();
        let stats_dir = PathBuf::from("stats");

        // Create stats directory if it doesn't exist
        if !stats_dir.exists() {
            if let Err(e) = fs::create_dir_all(&stats_dir) {
                warn!("Failed to create stats directory: {}", e);
                return stats;
            }
        }

        // Read all JSON files in the stats directory
        if let Ok(entries) = fs::read_dir(&stats_dir) {
            let mut valid_files = 0;
            let mut total_files = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "json") {
                        total_files += 1;
                        if let Err(e) = stats.accumulate_file(&path) {
                            warn!("Failed to process stats file {:?}: {}", path, e);
                        } else {
                            valid_files += 1;
                        }
                    }
                }
            }

            info!("Processed {}/{} stats files successfully", valid_files, total_files);
        }

        stats
    }

    fn accumulate_file(&mut self, path: &Path) -> std::io::Result<()> {
        let content = fs::read_to_string(path)?;
        let quote_stats: QuoteStats = serde_json::from_str(&content)?;

        self.total_quotes += 1;
        self.total_keystrokes += quote_stats.metrics.keystrokes;
        self.total_correct_keystrokes += quote_stats.metrics.correct_keystrokes;
        self.total_errors += quote_stats.metrics.errors.len();

        // Update running averages
        let prev_total = self.avg_wpm * (self.total_quotes - 1) as f64;
        self.avg_wpm = (prev_total + quote_stats.metrics.wpm) / self.total_quotes as f64;

        let prev_acc = self.avg_accuracy * (self.total_quotes - 1) as f64;
        self.avg_accuracy = (prev_acc + quote_stats.metrics.accuracy) / self.total_quotes as f64;

        // Update row stats
        if let Some(stats) = quote_stats.metrics.row_stats.get(&KeyboardRow::Number) {
            self.number_row_stats.update(stats.current, Instant::now());
        }

        if let Some(stats) = quote_stats.metrics.row_stats.get(&KeyboardRow::Top) {
            self.top_row_stats.update(stats.current, Instant::now());
        }

        if let Some(stats) = quote_stats.metrics.row_stats.get(&KeyboardRow::Home) {
            self.home_row_stats.update(stats.current, Instant::now());
        }

        if let Some(stats) = quote_stats.metrics.row_stats.get(&KeyboardRow::Bottom) {
            self.bottom_row_stats.update(stats.current, Instant::now());
        }

        // Update finger stats
        for (finger, stats) in &quote_stats.metrics.finger_stats {
            let finger_stats = self.finger_stats.entry(*finger).or_insert_with(ExtendedStats::new);
            finger_stats.update(stats.current, Instant::now());
        }

        // Update key averages
        for (key, avg) in quote_stats.metrics.get_key_geometric_averages() {
            let key_stats = self.key_averages.entry(key).or_insert_with(ExtendedStats::new);
            key_stats.update(avg, Instant::now());
        }

        Ok(())
    }

    pub fn add_quote_stats(&mut self, quote_stats: &QuoteStats) {
        self.total_quotes += 1;
        self.total_keystrokes += quote_stats.metrics.keystrokes;
        self.total_correct_keystrokes += quote_stats.metrics.correct_keystrokes;
        self.total_errors += quote_stats.metrics.errors.len();

        // Update running averages
        let prev_total = self.avg_wpm * (self.total_quotes - 1) as f64;
        self.avg_wpm = (prev_total + quote_stats.metrics.wpm) / self.total_quotes as f64;

        let prev_acc = self.avg_accuracy * (self.total_quotes - 1) as f64;
        self.avg_accuracy = (prev_acc + quote_stats.metrics.accuracy) / self.total_quotes as f64;

        // Update row stats
        if let Some(stats) = quote_stats.metrics.row_stats.get(&KeyboardRow::Number) {
            self.number_row_stats.update(stats.current, Instant::now());
        }

        if let Some(stats) = quote_stats.metrics.row_stats.get(&KeyboardRow::Top) {
            self.top_row_stats.update(stats.current, Instant::now());
        }

        if let Some(stats) = quote_stats.metrics.row_stats.get(&KeyboardRow::Home) {
            self.home_row_stats.update(stats.current, Instant::now());
        }

        if let Some(stats) = quote_stats.metrics.row_stats.get(&KeyboardRow::Bottom) {
            self.bottom_row_stats.update(stats.current, Instant::now());
        }

        // Update finger stats
        for (finger, stats) in &quote_stats.metrics.finger_stats {
            let finger_stats = self.finger_stats.entry(*finger).or_insert_with(ExtendedStats::new);
            finger_stats.update(stats.current, Instant::now());
        }

        // Update key averages
        for (key, avg) in quote_stats.metrics.get_key_geometric_averages() {
            let key_stats = self.key_averages.entry(key).or_insert_with(ExtendedStats::new);
            key_stats.update(avg, Instant::now());
        }
    }
}

fn weighted_average(current: f64, new_value: f64, total_samples: usize) -> f64 {
    if total_samples <= 1 {
        new_value
    } else {
        let weight = 1.0 / total_samples as f64;
        current * (1.0 - weight) + new_value * weight
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteStats {
    pub quote: String,
    pub metrics: TypingMetrics,
    pub timestamp: DateTime<Utc>,
} 