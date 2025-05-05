use std::collections::HashMap;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use crate::core::histogram::HistogramStats;

/// Represents keyboard rows for metrics tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyboardRow {
    /// The top row (numbers and qwertyuiop)
    Top,
    /// The home row (asdfghjkl)
    Home,
    /// The bottom row (zxcvbnm)
    Bottom,
}

/// Represents finger used for typing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    Thumb,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
}

/// Represents a typing error
#[derive(Debug, Clone)]
pub struct TypingError {
    /// The expected character
    pub expected: char,
    /// The actual character typed
    pub actual: char,
    /// The position in the text
    pub position: usize,
    /// The time when the error occurred
    pub timestamp: Instant,
}

/// Extended statistics for tracking performance over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedStats {
    pub current: f64,
    pub avg_10s: f64,
    pub avg_60s: f64,
    pub avg_quote: f64,
    pub slowest: f64,
    pub fastest: f64,
    #[serde(skip)]
    pub times_10s: Vec<(Instant, f64)>,
    #[serde(skip)]
    pub times_60s: Vec<(Instant, f64)>,
    pub quote_times: Vec<f64>,
}

impl ExtendedStats {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            avg_10s: 0.0,
            avg_60s: 0.0,
            avg_quote: 0.0,
            slowest: 0.0,
            fastest: 0.0,
            times_10s: Vec::new(),
            times_60s: Vec::new(),
            quote_times: Vec::new(),
        }
    }

    pub fn update(&mut self, value: f64, now: Instant) {
        self.current = value;
        
        // Update slowest/fastest
        self.slowest = self.slowest.max(value);
        self.fastest = if self.fastest == 0.0 {
            value
        } else {
            self.fastest.min(value)
        };
        
        // Update 10s average
        self.times_10s.retain(|(time, _)| now.duration_since(*time).as_secs() <= 10);
        self.times_10s.push((now, value));
        self.avg_10s = self.times_10s.iter().map(|(_, v)| v).sum::<f64>() / self.times_10s.len() as f64;
        
        // Update 60s average
        self.times_60s.retain(|(time, _)| now.duration_since(*time).as_secs() <= 60);
        self.times_60s.push((now, value));
        self.avg_60s = self.times_60s.iter().map(|(_, v)| v).sum::<f64>() / self.times_60s.len() as f64;
    }

    pub fn update_quote_stats(&mut self, value: f64) {
        self.quote_times.push(value);
        self.avg_quote = self.quote_times.iter().sum::<f64>() / self.quote_times.len() as f64;
    }
}

/// Per-character typing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMetrics {
    /// Total number of times this character was typed
    pub count: usize,
    /// Number of times this character was typed correctly
    pub correct_count: usize,
    /// Total time spent typing this character (in milliseconds)
    pub total_time_ms: u64,
    /// Minimum time to type this character (in milliseconds)
    pub min_time_ms: u64,
    /// Maximum time to type this character (in milliseconds)
    pub max_time_ms: u64,
    /// Running average of time to type this character (in milliseconds)
    pub avg_time_ms: f64,
    /// Short-term average (last 10 keystrokes) time to type this character (in milliseconds)
    pub short_term_avg_ms: f64,
    /// Recent typing times for this character (for short-term average calculation)
    pub recent_times_ms: Vec<u64>,
    /// Last recorded delay for this character (in milliseconds)
    pub last_delay_ms: u64,
    /// Keyboard row this character belongs to
    pub row: KeyboardRow,
    /// Finger typically used to type this character
    pub finger: Finger,
    pub extended_stats: ExtendedStats,
}

impl CharacterMetrics {
    /// Create new metrics for a character
    pub fn new(row: KeyboardRow, finger: Finger) -> Self {
        Self {
            count: 0,
            correct_count: 0,
            total_time_ms: 0,
            min_time_ms: u64::MAX,
            max_time_ms: 0,
            avg_time_ms: 0.0,
            short_term_avg_ms: 0.0,
            recent_times_ms: Vec::with_capacity(10),
            last_delay_ms: 0,
            row,
            finger,
            extended_stats: ExtendedStats::new(),
        }
    }
    
    /// Update metrics with a new keystroke
    pub fn update(&mut self, time_ms: u64, correct: bool) {
        self.count += 1;
        if correct {
            self.correct_count += 1;
            self.last_delay_ms = time_ms;
            
            self.total_time_ms += time_ms;
            self.min_time_ms = self.min_time_ms.min(time_ms);
            self.max_time_ms = self.max_time_ms.max(time_ms);
            
            // Update running average
            self.avg_time_ms = self.total_time_ms as f64 / self.correct_count as f64;
            
            // Update recent times for short-term average
            if self.recent_times_ms.len() >= 10 {
                self.recent_times_ms.remove(0);
            }
            self.recent_times_ms.push(time_ms);
            
            // Calculate short-term average
            if !self.recent_times_ms.is_empty() {
                let sum: u64 = self.recent_times_ms.iter().sum();
                self.short_term_avg_ms = sum as f64 / self.recent_times_ms.len() as f64;
            }
            
            self.extended_stats.update(time_ms as f64, Instant::now());
        }
    }
    
    /// Get accuracy for this character
    pub fn accuracy(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            (self.correct_count as f64 / self.count as f64) * 100.0
        }
    }

    pub fn update_quote_stats(&mut self) {
        if !self.recent_times_ms.is_empty() {
            let avg = self.recent_times_ms.iter().sum::<u64>() as f64 / self.recent_times_ms.len() as f64;
            self.extended_stats.update_quote_stats(avg);
        }
    }
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

/// Stats collected for a completed quote
#[derive(Serialize, Deserialize)]
pub struct QuoteStats {
    pub quote_text: String,
    pub wpm: f64,
    pub accuracy: f64,
    pub total_keystrokes: usize,
    pub correct_keystrokes: usize,
    pub error_count: usize,
    pub number_row_avg: f64,
    pub top_row_avg: f64,
    pub home_row_avg: f64,
    pub bottom_row_avg: f64,
    pub finger_stats: HashMap<Finger, FingerStats>,
    pub key_geometric_averages: HashMap<char, f64>,
    pub timestamp: String,
}

/// Individual finger statistics for serialization
#[derive(Serialize, Deserialize)]
pub struct FingerStats {
    pub current: f64,
    pub avg_10s: f64,
    pub avg_60s: f64,
    pub fastest: f64,
    pub slowest: f64,
}

/// Detailed typing metrics with per-character statistics
#[derive(Debug, Clone)]
pub struct TypingMetrics {
    /// Words per minute
    pub wpm: f64,
    /// Overall accuracy percentage
    pub accuracy: f64,
    /// Total keystrokes
    pub keystrokes: usize,
    /// Correct keystrokes
    pub correct_keystrokes: usize,
    /// Typing errors
    pub errors: Vec<TypingError>,
    /// Start time of the current session
    pub start_time: Instant,
    /// Current time (updated regularly)
    pub current_time: Instant,
    /// Per-character metrics
    pub char_metrics: HashMap<char, CharacterMetrics>,
    /// Metrics for number keys
    pub number_metrics: CategoryMetrics,
    /// Metrics for letter keys
    pub letter_metrics: CategoryMetrics,
    /// Metrics for top row keys
    pub top_row_metrics: CategoryMetrics,
    /// Metrics for home row keys
    pub home_row_metrics: CategoryMetrics,
    /// Metrics for bottom row keys
    pub bottom_row_metrics: CategoryMetrics,
    /// Metrics per finger
    pub finger_metrics: HashMap<Finger, ExtendedStats>,
    /// Last keystroke time
    pub last_keystroke_time: Option<Instant>,
    /// Key speed histogram
    pub key_histogram: HistogramStats,
    /// WPM histogram
    pub wpm_histogram: HistogramStats,
}

impl TypingMetrics {
    /// Create new typing metrics
    pub fn new() -> Self {
        let mut metrics = Self {
            wpm: 0.0,
            accuracy: 0.0,
            keystrokes: 0,
            correct_keystrokes: 0,
            errors: Vec::new(),
            start_time: Instant::now(),
            current_time: Instant::now(),
            char_metrics: HashMap::new(),
            number_metrics: CategoryMetrics::new(),
            letter_metrics: CategoryMetrics::new(),
            top_row_metrics: CategoryMetrics::new(),
            home_row_metrics: CategoryMetrics::new(),
            bottom_row_metrics: CategoryMetrics::new(),
            finger_metrics: HashMap::new(),
            last_keystroke_time: None,
            key_histogram: HistogramStats::new_key_speed(),
            wpm_histogram: HistogramStats::new_wpm(),
        };
        
        // Initialize finger metrics for all fingers
        metrics.finger_metrics.insert(Finger::LeftPinky, ExtendedStats::new());
        metrics.finger_metrics.insert(Finger::LeftRing, ExtendedStats::new());
        metrics.finger_metrics.insert(Finger::LeftMiddle, ExtendedStats::new());
        metrics.finger_metrics.insert(Finger::LeftIndex, ExtendedStats::new());
        metrics.finger_metrics.insert(Finger::Thumb, ExtendedStats::new());
        metrics.finger_metrics.insert(Finger::RightIndex, ExtendedStats::new());
        metrics.finger_metrics.insert(Finger::RightMiddle, ExtendedStats::new());
        metrics.finger_metrics.insert(Finger::RightRing, ExtendedStats::new());
        metrics.finger_metrics.insert(Finger::RightPinky, ExtendedStats::new());
        
        // Initialize character metrics for standard keyboard
        metrics.initialize_keyboard_mapping();
        
        metrics
    }
    
    /// Initialize character metrics for standard keyboard layout
    fn initialize_keyboard_mapping(&mut self) {
        // Top row - numbers
        for c in "1234567890-=".chars() {
            let finger = match c {
                '1' | '2' => Finger::LeftPinky,
                '3' => Finger::LeftRing,
                '4' => Finger::LeftMiddle,
                '5' | '6' => Finger::LeftIndex,
                '7' | '8' => Finger::RightIndex,
                '9' => Finger::RightMiddle,
                '0' => Finger::RightRing,
                '-' | '=' => Finger::RightPinky,
                _ => Finger::RightPinky,
            };
            self.char_metrics.insert(c, CharacterMetrics::new(KeyboardRow::Top, finger));
        }
        
        // Top row - letters
        for c in "qwertyuiop[]\\".chars() {
            let finger = match c {
                'q' | 'a' | 'z' => Finger::LeftPinky,
                'w' | 's' | 'x' => Finger::LeftRing,
                'e' | 'd' | 'c' => Finger::LeftMiddle,
                'r' | 'f' | 'v' | 't' | 'g' | 'b' => Finger::LeftIndex,
                'y' | 'h' | 'n' | 'u' | 'j' | 'm' => Finger::RightIndex,
                'i' | 'k' | ',' => Finger::RightMiddle,
                'o' | 'l' | '.' => Finger::RightRing,
                'p' | ';' | '\'' | '[' | ']' | '\\' => Finger::RightPinky,
                _ => Finger::RightPinky,
            };
            self.char_metrics.insert(c, CharacterMetrics::new(KeyboardRow::Top, finger));
        }
        
        // Home row
        for c in "asdfghjkl;'".chars() {
            let finger = match c {
                'a' => Finger::LeftPinky,
                's' => Finger::LeftRing,
                'd' => Finger::LeftMiddle,
                'f' | 'g' => Finger::LeftIndex,
                'h' | 'j' => Finger::RightIndex,
                'k' => Finger::RightMiddle,
                'l' => Finger::RightRing,
                ';' | '\'' => Finger::RightPinky,
                _ => Finger::RightPinky,
            };
            self.char_metrics.insert(c, CharacterMetrics::new(KeyboardRow::Home, finger));
        }
        
        // Bottom row
        for c in "zxcvbnm,./".chars() {
            let finger = match c {
                'z' => Finger::LeftPinky,
                'x' => Finger::LeftRing,
                'c' => Finger::LeftMiddle,
                'v' | 'b' => Finger::LeftIndex,
                'n' | 'm' => Finger::RightIndex,
                ',' => Finger::RightMiddle,
                '.' => Finger::RightRing,
                '/' => Finger::RightPinky,
                _ => Finger::RightPinky,
            };
            self.char_metrics.insert(c, CharacterMetrics::new(KeyboardRow::Bottom, finger));
        }
        
        // Space bar
        self.char_metrics.insert(' ', CharacterMetrics::new(KeyboardRow::Bottom, Finger::Thumb));
    }
    
    /// Record a keystroke
    pub fn record_keystroke(&mut self, c: char, expected_c: char, position: usize) {
        self.current_time = Instant::now();
        self.keystrokes += 1;
        
        let is_correct = c == expected_c;
        if is_correct {
            self.correct_keystrokes += 1;
        } else {
            self.errors.push(TypingError {
                expected: expected_c,
                actual: c,
                position,
                timestamp: self.current_time,
            });
        }
        
        // Calculate time since last keystroke
        let time_ms = if let Some(last_time) = self.last_keystroke_time {
            self.current_time.duration_since(last_time).as_millis() as u64
        } else {
            // First keystroke, use a reasonable default (300ms)
            300
        };
        
        // Update per-character metrics
        if !self.char_metrics.contains_key(&c) {
            // Default to right pinky for unknown characters
            self.char_metrics.insert(c, CharacterMetrics::new(KeyboardRow::Top, Finger::RightPinky));
        }
        
        if let Some(metrics) = self.char_metrics.get_mut(&c) {
            metrics.update(time_ms, is_correct);
            
            // Only update category and finger metrics for correct keystrokes
            if is_correct {
                // Update category metrics
                if c.is_ascii_digit() {
                    self.number_metrics.update(time_ms, is_correct);
                }
                
                if c.is_alphabetic() {
                    self.letter_metrics.update(time_ms, is_correct);
                }
                
                // Update row metrics
                match metrics.row {
                    KeyboardRow::Top => self.top_row_metrics.update(time_ms, is_correct),
                    KeyboardRow::Home => self.home_row_metrics.update(time_ms, is_correct),
                    KeyboardRow::Bottom => self.bottom_row_metrics.update(time_ms, is_correct),
                }
                
                // Update finger metrics with extended stats
                if let Some(finger_stats) = self.finger_metrics.get_mut(&metrics.finger) {
                    finger_stats.update(time_ms as f64, self.current_time);
                }

                // Record timing in histogram for any correct keystroke
                let ms = time_ms as f64;
                self.key_histogram.record_value(ms);
                
                // Convert to WPM and record
                let wpm = HistogramStats::ms_to_wpm(ms);
                self.wpm_histogram.record_value(wpm);
            }
        }
        
        // Update last keystroke time
        self.last_keystroke_time = Some(self.current_time);
        
        // Update overall metrics
        self.calculate_overall_metrics();
    }
    
    /// Calculate overall metrics like WPM and accuracy
    pub fn calculate_overall_metrics(&mut self) {
        // Calculate words per minute
        let elapsed = self.current_time.duration_since(self.start_time).as_secs_f64();
        let words = self.keystrokes as f64 / 5.0; // Standard: 5 keystrokes = 1 word
        self.wpm = if elapsed > 0.0 { (words * 60.0) / elapsed } else { 0.0 };
        
        // Calculate accuracy
        if self.keystrokes > 0 {
            self.accuracy = (self.correct_keystrokes as f64 / self.keystrokes as f64) * 100.0;
        }
    }
    
    /// Generate a heat map for key speed
    pub fn generate_heat_map(&self) -> HashMap<char, (f64, u64)> {
        let mut heat_map = HashMap::new();
        
        for (c, metrics) in &self.char_metrics {
            if metrics.count > 0 {
                // Normalize to a value between 0.0 and 1.0
                // Lower is better (faster typing)
                let avg_normalized = if metrics.avg_time_ms <= 100.0 {
                    0.0 // Very fast (100ms or less)
                } else if metrics.avg_time_ms >= 500.0 {
                    1.0 // Very slow (500ms or more)
                } else {
                    (metrics.avg_time_ms - 100.0) / 400.0 // Linear scaling between 100-500ms
                };
                
                heat_map.insert(*c, (avg_normalized, metrics.last_delay_ms));
            }
        }
        
        heat_map
    }
    
    /// Get finger performance summary with extended stats
    pub fn finger_performance(&self) -> &HashMap<Finger, ExtendedStats> {
        &self.finger_metrics
    }

    /// Calculate geometric average for a key (combining upper and lowercase stats)
    fn get_key_geometric_avg(&self, key: char) -> f64 {
        let lowercase = key.to_ascii_lowercase();
        let uppercase = key.to_ascii_uppercase();
        
        let mut values = Vec::new();
        
        // Collect stats for both cases if they exist
        if let Some(metrics) = self.char_metrics.get(&lowercase) {
            if metrics.count > 0 {
                values.push(metrics.avg_time_ms);
            }
        }
        if let Some(metrics) = self.char_metrics.get(&uppercase) {
            if metrics.count > 0 {
                values.push(metrics.avg_time_ms);
            }
        }
        
        // Calculate geometric mean if we have values
        if values.is_empty() {
            0.0
        } else {
            let product: f64 = values.iter().product();
            product.powf(1.0 / values.len() as f64)
        }
    }

    /// Get geometric averages for all typable keys
    pub fn get_key_geometric_averages(&self) -> HashMap<char, f64> {
        let mut averages = HashMap::new();
        
        // Letters (a-z, store as lowercase)
        for c in 'a'..='z' {
            let avg = self.get_key_geometric_avg(c);
            if avg > 0.0 {
                averages.insert(c, avg);
            }
        }
        
        // Numbers
        for c in '0'..='9' {
            let avg = self.get_key_geometric_avg(c);
            if avg > 0.0 {
                averages.insert(c, avg);
            }
        }
        
        // Special characters
        for c in ['-', '=', '[', ']', '\\', ';', '\'', ',', '.', '/', ' '] {
            let avg = self.get_key_geometric_avg(c);
            if avg > 0.0 {
                averages.insert(c, avg);
            }
        }
        
        averages
    }

    /// Save the current stats to a JSON file
    pub fn save_to_json(&self, quote_text: &str) -> std::io::Result<()> {
        // Get key geometric averages
        let key_geometric_averages = self.get_key_geometric_averages();

        // Create stats directory if it doesn't exist
        let stats_dir = PathBuf::from("stats");
        fs::create_dir_all(&stats_dir)?;

        // Generate timestamp-based filename
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let filename = stats_dir.join(format!("{}.json", timestamp));

        // Create and save the stats object
        let stats = QuoteStats {
            quote_text: quote_text.to_string(),
            wpm: self.wpm,
            accuracy: self.accuracy,
            total_keystrokes: self.keystrokes,
            correct_keystrokes: self.correct_keystrokes,
            error_count: self.errors.len(),
            number_row_avg: self.number_metrics.avg_time_ms,
            top_row_avg: self.top_row_metrics.avg_time_ms,
            home_row_avg: self.home_row_metrics.avg_time_ms,
            bottom_row_avg: self.bottom_row_metrics.avg_time_ms,
            finger_stats: self.finger_metrics
                .iter()
                .map(|(finger, stats)| {
                    (*finger, FingerStats {
                        current: stats.current,
                        avg_10s: stats.avg_10s,
                        avg_60s: stats.avg_60s,
                        fastest: stats.fastest,
                        slowest: stats.slowest,
                    })
                })
                .collect(),
            key_geometric_averages,
            timestamp: timestamp.clone(),
        };

        // Serialize and save to file
        let json = serde_json::to_string_pretty(&stats)?;
        fs::write(filename, json)?;

        Ok(())
    }

    /// Reset quote-specific stats while maintaining overall metrics
    pub fn prepare_for_new_quote(&mut self) {
        // Clear errors for new quote
        self.errors.clear();
        
        // Update quote stats for all characters before resetting
        for metrics in self.char_metrics.values_mut() {
            metrics.update_quote_stats();
        }

        // Reset last keystroke time
        self.last_keystroke_time = None;

        // Reset start time for WPM calculation
        self.start_time = Instant::now();
        self.current_time = self.start_time;

        // Reset current quote histograms
        self.key_histogram.reset_current();
        self.wpm_histogram.reset_current();
    }
} 