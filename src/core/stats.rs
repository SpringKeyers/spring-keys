use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core::metrics::{Finger, FingerStats};
use log::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccumulatedStats {
    pub total_quotes: usize,
    pub total_keystrokes: usize,
    pub total_correct_keystrokes: usize,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub total_errors: usize,
    pub row_averages: RowAverages,
    pub finger_stats: HashMap<Finger, AccumulatedFingerStats>,
    pub key_averages: HashMap<char, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowAverages {
    pub number_row: f64,
    pub top_row: f64,
    pub home_row: f64,
    pub bottom_row: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccumulatedFingerStats {
    pub avg_speed: f64,
    pub fastest: f64,
    pub slowest: f64,
    pub total_keystrokes: usize,
}

impl AccumulatedStats {
    pub fn new() -> Self {
        Self {
            total_quotes: 0,
            total_keystrokes: 0,
            total_correct_keystrokes: 0,
            avg_wpm: 0.0,
            avg_accuracy: 0.0,
            total_errors: 0,
            row_averages: RowAverages {
                number_row: 0.0,
                top_row: 0.0,
                home_row: 0.0,
                bottom_row: 0.0,
            },
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

        // Accumulate basic stats
        self.total_quotes += 1;
        self.total_keystrokes += quote_stats.total_keystrokes;
        self.total_correct_keystrokes += quote_stats.correct_keystrokes;
        self.total_errors += quote_stats.error_count;

        // Update WPM and accuracy averages
        let prev_total = (self.avg_wpm * (self.total_quotes - 1) as f64) as f64;
        self.avg_wpm = (prev_total + quote_stats.wpm) / self.total_quotes as f64;

        let prev_acc = (self.avg_accuracy * (self.total_quotes - 1) as f64) as f64;
        self.avg_accuracy = (prev_acc + quote_stats.accuracy) / self.total_quotes as f64;

        // Update row averages
        self.row_averages.number_row = weighted_average(
            self.row_averages.number_row,
            quote_stats.number_row_avg,
            self.total_quotes,
        );
        self.row_averages.top_row = weighted_average(
            self.row_averages.top_row,
            quote_stats.top_row_avg,
            self.total_quotes,
        );
        self.row_averages.home_row = weighted_average(
            self.row_averages.home_row,
            quote_stats.home_row_avg,
            self.total_quotes,
        );
        self.row_averages.bottom_row = weighted_average(
            self.row_averages.bottom_row,
            quote_stats.bottom_row_avg,
            self.total_quotes,
        );

        // Update finger stats
        for (finger, stats) in quote_stats.finger_stats {
            let acc_stats = self.finger_stats.entry(finger).or_insert(AccumulatedFingerStats {
                avg_speed: 0.0,
                fastest: f64::INFINITY,
                slowest: 0.0,
                total_keystrokes: 0,
            });

            acc_stats.avg_speed = weighted_average(
                acc_stats.avg_speed,
                stats.current,
                self.total_quotes,
            );
            acc_stats.fastest = acc_stats.fastest.min(stats.fastest);
            acc_stats.slowest = acc_stats.slowest.max(stats.slowest);
            acc_stats.total_keystrokes += 1;
        }

        // Update key averages
        for (key, avg) in quote_stats.key_geometric_averages {
            let current_avg = self.key_averages.entry(key).or_insert(0.0);
            *current_avg = weighted_average(*current_avg, avg, self.total_quotes);
        }

        Ok(())
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

#[derive(Debug, Serialize, Deserialize)]
struct QuoteStats {
    wpm: f64,
    accuracy: f64,
    total_keystrokes: usize,
    correct_keystrokes: usize,
    error_count: usize,
    number_row_avg: f64,
    top_row_avg: f64,
    home_row_avg: f64,
    bottom_row_avg: f64,
    finger_stats: HashMap<Finger, FingerStats>,
    key_geometric_averages: HashMap<char, f64>,
    timestamp: String,
} 