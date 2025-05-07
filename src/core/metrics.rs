use std::collections::HashMap;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use crate::core::histogram::HistogramStats;
use crate::core::stats::QuoteStats;

/// Represents keyboard rows for metrics tracking
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum KeyboardRow {
    Number,
    Top,
    Home,
    Bottom,
}

/// Represents finger used for typing
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    LeftThumb,
    RightThumb,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

/// Represents a typing error
#[derive(Debug, Clone, Serialize)]
pub struct TypingError {
    pub expected: char,
    pub actual: char,
    pub position: usize,
    #[serde(skip)]
    pub timestamp: Instant,
}

impl<'de> Deserialize<'de> for TypingError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            expected: char,
            actual: char,
            position: usize,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(TypingError {
            expected: helper.expected,
            actual: helper.actual,
            position: helper.position,
            timestamp: Instant::now(),
        })
    }
}

/// Extended statistics for tracking performance over time
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExtendedStats {
    pub current: f64,
    pub avg_10s: f64,
    pub avg_60s: f64,
    pub fastest: f64,
    pub slowest: f64,
}

/// Per-character typing statistics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CharacterMetrics {
    pub row: KeyboardRow,
    pub finger: Finger,
    pub avg_time_ms: f64,
    pub total_time_ms: f64,
    pub count: usize,
    pub errors: usize,
}

/// Category metrics for grouping characters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryMetrics {
    /// Total number of keystrokes in this category
    pub count: usize,
    /// Number of correct keystrokes in this category
    pub correct_count: usize,
    /// Total time spent typing characters in this category (in milliseconds)
    pub total_time_ms: u64,
    /// Average time per keystroke in this category (in milliseconds)
    pub avg_time_ms: f64,
    /// Short-term average (last 20 keystrokes) time in this category (in milliseconds)
    pub short_term_avg_ms: f64,
    /// Recent typing times for this category (for short-term average calculation)
    pub recent_times_ms: Vec<u64>,
}

/// Detailed typing metrics with per-character statistics
#[derive(Debug, Clone, Serialize)]
pub struct TypingMetrics {
    #[serde(skip)]
    pub start_time: Instant,
    #[serde(skip)]
    pub current_time: Instant,
    pub keystrokes: usize,
    pub correct_keystrokes: usize,
    #[serde(skip)]
    pub errors: Vec<TypingError>,
    pub wpm: f64,
    pub accuracy: f64,
    pub key_timings: HashMap<char, Vec<f64>>,
    pub key_errors: HashMap<char, usize>,
    pub finger_stats: HashMap<Finger, ExtendedStats>,
    pub row_stats: HashMap<KeyboardRow, ExtendedStats>,
    pub char_metrics: HashMap<char, CharacterMetrics>,
    pub number_metrics: CategoryMetrics,
    pub letter_metrics: CategoryMetrics,
    pub top_row_metrics: CategoryMetrics,
    pub home_row_metrics: CategoryMetrics,
    pub bottom_row_metrics: CategoryMetrics,
    #[serde(skip)]
    pub last_keystroke_time: Option<Instant>,
    pub key_histogram: HistogramStats,
    pub wpm_histogram: HistogramStats,
}

impl<'de> Deserialize<'de> for TypingMetrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            keystrokes: usize,
            correct_keystrokes: usize,
            wpm: f64,
            accuracy: f64,
            key_timings: HashMap<char, Vec<f64>>,
            key_errors: HashMap<char, usize>,
            finger_stats: HashMap<Finger, ExtendedStats>,
            row_stats: HashMap<KeyboardRow, ExtendedStats>,
            char_metrics: HashMap<char, CharacterMetrics>,
            number_metrics: CategoryMetrics,
            letter_metrics: CategoryMetrics,
            top_row_metrics: CategoryMetrics,
            home_row_metrics: CategoryMetrics,
            bottom_row_metrics: CategoryMetrics,
            key_histogram: HistogramStats,
            wpm_histogram: HistogramStats,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(TypingMetrics {
            start_time: Instant::now(),
            current_time: Instant::now(),
            keystrokes: helper.keystrokes,
            correct_keystrokes: helper.correct_keystrokes,
            errors: Vec::new(),
            wpm: helper.wpm,
            accuracy: helper.accuracy,
            key_timings: helper.key_timings,
            key_errors: helper.key_errors,
            finger_stats: helper.finger_stats,
            row_stats: helper.row_stats,
            char_metrics: helper.char_metrics,
            number_metrics: helper.number_metrics,
            letter_metrics: helper.letter_metrics,
            top_row_metrics: helper.top_row_metrics,
            home_row_metrics: helper.home_row_metrics,
            bottom_row_metrics: helper.bottom_row_metrics,
            last_keystroke_time: None,
            key_histogram: helper.key_histogram,
            wpm_histogram: helper.wpm_histogram,
        })
    }
}

impl TypingMetrics {
    pub fn new() -> Self {
        let mut metrics = Self {
            start_time: Instant::now(),
            current_time: Instant::now(),
            keystrokes: 0,
            correct_keystrokes: 0,
            errors: Vec::new(),
            wpm: 0.0,
            accuracy: 0.0,
            key_timings: HashMap::new(),
            key_errors: HashMap::new(),
            finger_stats: HashMap::new(),
            row_stats: HashMap::new(),
            char_metrics: HashMap::new(),
            number_metrics: CategoryMetrics::new(),
            letter_metrics: CategoryMetrics::new(),
            top_row_metrics: CategoryMetrics::new(),
            home_row_metrics: CategoryMetrics::new(),
            bottom_row_metrics: CategoryMetrics::new(),
            last_keystroke_time: None,
            key_histogram: HistogramStats::new(),
            wpm_histogram: HistogramStats::new(),
        };

        // Initialize finger stats
        for finger in [
            Finger::LeftPinky, Finger::LeftRing, Finger::LeftMiddle, Finger::LeftIndex,
            Finger::LeftThumb, Finger::RightThumb,
            Finger::RightIndex, Finger::RightMiddle, Finger::RightRing, Finger::RightPinky,
        ] {
            metrics.finger_stats.insert(finger, ExtendedStats::new());
        }

        // Initialize row stats
        for row in [
            KeyboardRow::Number,
            KeyboardRow::Top,
            KeyboardRow::Home,
            KeyboardRow::Bottom,
        ] {
            metrics.row_stats.insert(row, ExtendedStats::new());
        }

        metrics
    }

    pub fn record_keystroke(&mut self, c: char, expected: char, position: usize) {
        self.keystrokes += 1;
        if c == expected {
            self.correct_keystrokes += 1;
        } else {
            self.errors.push(TypingError {
                expected,
                actual: c,
                position,
                timestamp: Instant::now(),
            });
            *self.key_errors.entry(c).or_insert(0) += 1;
        }

        let now = Instant::now();
        if let Some(last_time) = self.last_keystroke_time {
            let time_ms = now.duration_since(last_time).as_millis() as f64;
            self.key_timings.entry(c).or_insert_with(Vec::new).push(time_ms);
            self.key_histogram.add_value(time_ms);
        }
        self.last_keystroke_time = Some(now);
    }

    pub fn calculate_overall_metrics(&mut self) {
        let elapsed = self.current_time.duration_since(self.start_time).as_secs_f64();
        let minutes = elapsed / 60.0;
        let words = self.correct_keystrokes as f64 / 5.0;
        self.wpm = if minutes > 0.0 { words / minutes } else { 0.0 };
        self.accuracy = if self.keystrokes > 0 {
            (self.correct_keystrokes as f64 / self.keystrokes as f64) * 100.0
        } else {
            0.0
        };
        self.wpm_histogram.add_value(self.wpm);
    }

    pub fn save_to_json(&self, quote: &str) -> Result<(), Box<dyn std::error::Error>> {
        let stats_dir = PathBuf::from("stats");
        fs::create_dir_all(&stats_dir)?;

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("typing_stats_{}.json", timestamp);
        let file_path = stats_dir.join(filename);

        let quote_stats = QuoteStats {
            quote: quote.to_string(),
            metrics: self.clone(),
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string_pretty(&quote_stats)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    pub fn get_heat_map(&self) -> HashMap<char, f64> {
        let mut heat_map = HashMap::new();
        for (key, timings) in &self.key_timings {
            if !timings.is_empty() {
                let avg = timings.iter().sum::<f64>() / timings.len() as f64;
                heat_map.insert(*key, avg);
            }
        }
        heat_map
    }

    pub fn get_key_geometric_averages(&self) -> HashMap<char, f64> {
        let mut averages = HashMap::new();
        for (key, timings) in &self.key_timings {
            if !timings.is_empty() {
                let product: f64 = timings.iter().product();
                let avg = product.powf(1.0 / timings.len() as f64);
                averages.insert(*key, avg);
            }
        }
        averages
    }

    pub fn finger_performance(&self) -> &HashMap<Finger, ExtendedStats> {
        &self.finger_stats
    }

    pub fn simulate_demo_data(&mut self) {
        // Simulate some typing data for visualization
        for finger in self.finger_stats.values_mut() {
            finger.current = 180.0;
            finger.avg_10s = 200.0;
            finger.avg_60s = 220.0;
            finger.fastest = 150.0;
            finger.slowest = 250.0;
        }

        // Simulate some key timings
        for c in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
            self.key_timings.insert(c, vec![180.0, 200.0, 220.0]);
        }
    }
}

impl ExtendedStats {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            avg_10s: 0.0,
            avg_60s: 0.0,
            fastest: f64::INFINITY,
            slowest: 0.0,
        }
    }

    pub fn update(&mut self, value: f64, _now: Instant) {
        self.current = value;
        self.fastest = self.fastest.min(value);
        self.slowest = self.slowest.max(value);
        
        // Simple moving averages
        if self.avg_10s == 0.0 {
            self.avg_10s = value;
        } else {
            self.avg_10s = 0.9 * self.avg_10s + 0.1 * value;
        }
        
        if self.avg_60s == 0.0 {
            self.avg_60s = value;
        } else {
            self.avg_60s = 0.98 * self.avg_60s + 0.02 * value;
        }
    }
}

impl CharacterMetrics {
    pub fn new(row: KeyboardRow, finger: Finger) -> Self {
        Self {
            row,
            finger,
            avg_time_ms: 0.0,
            total_time_ms: 0.0,
            count: 0,
            errors: 0,
        }
    }

    pub fn update(&mut self, time_ms: f64, is_correct: bool) {
        self.count += 1;
        if !is_correct {
            self.errors += 1;
        }
        self.total_time_ms += time_ms;
        self.avg_time_ms = self.total_time_ms / self.count as f64;
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    pub fn update_quote_stats(&mut self) {
        // Reset stats for the new quote
        self.total_time_ms = 0.0;
        self.count = 0;
        self.errors = 0;
        self.avg_time_ms = 0.0;
    }
}

impl CategoryMetrics {
    /// Create new category metrics
    pub fn new() -> Self {
        Self {
            count: 0,
            correct_count: 0,
            total_time_ms: 0,
            avg_time_ms: 0.0,
            short_term_avg_ms: 0.0,
            recent_times_ms: Vec::with_capacity(20),
        }
    }
    
    /// Update metrics with a new keystroke
    pub fn update(&mut self, time_ms: u64, correct: bool) {
        self.count += 1;
        if correct {
            self.correct_count += 1;
            
            self.total_time_ms += time_ms;
            
            // Update running average
            self.avg_time_ms = self.total_time_ms as f64 / self.correct_count as f64;
            
            // Update recent times for short-term average
            if self.recent_times_ms.len() >= 20 {
                self.recent_times_ms.remove(0);
            }
            self.recent_times_ms.push(time_ms);
            
            // Calculate short-term average
            if !self.recent_times_ms.is_empty() {
                let sum: u64 = self.recent_times_ms.iter().sum();
                self.short_term_avg_ms = sum as f64 / self.recent_times_ms.len() as f64;
            }
        }
    }
    
    /// Get accuracy for this category
    pub fn accuracy(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            (self.correct_count as f64 / self.count as f64) * 100.0
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FingerStats {
    pub current: f64,
    pub fastest: f64,
    pub slowest: f64,
}

impl FingerStats {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            fastest: f64::INFINITY,
            slowest: 0.0,
        }
    }

    pub fn update(&mut self, value: f64) {
        self.current = value;
        self.fastest = self.fastest.min(value);
        self.slowest = self.slowest.max(value);
    }
} 