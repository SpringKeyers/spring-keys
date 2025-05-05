use std::collections::HashMap;
use std::time::Instant;
use serde::{Serialize, Deserialize};

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
    /// Keyboard row this character belongs to
    pub row: KeyboardRow,
    /// Finger typically used to type this character
    pub finger: Finger,
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
            row,
            finger,
        }
    }
    
    /// Update metrics with a new keystroke
    pub fn update(&mut self, time_ms: u64, correct: bool) {
        self.count += 1;
        if correct {
            self.correct_count += 1;
        }
        
        self.total_time_ms += time_ms;
        self.min_time_ms = self.min_time_ms.min(time_ms);
        self.max_time_ms = self.max_time_ms.max(time_ms);
        
        // Update running average
        self.avg_time_ms = self.total_time_ms as f64 / self.count as f64;
        
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
    }
    
    /// Get accuracy for this character
    pub fn accuracy(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            (self.correct_count as f64 / self.count as f64) * 100.0
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
        }
        
        self.total_time_ms += time_ms;
        
        // Update running average
        self.avg_time_ms = self.total_time_ms as f64 / self.count as f64;
        
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
    
    /// Get accuracy for this category
    pub fn accuracy(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            (self.correct_count as f64 / self.count as f64) * 100.0
        }
    }
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
    pub finger_metrics: HashMap<Finger, CategoryMetrics>,
    /// Last keystroke time
    pub last_keystroke_time: Option<Instant>,
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
        };
        
        // Initialize finger metrics for all fingers
        metrics.finger_metrics.insert(Finger::LeftPinky, CategoryMetrics::new());
        metrics.finger_metrics.insert(Finger::LeftRing, CategoryMetrics::new());
        metrics.finger_metrics.insert(Finger::LeftMiddle, CategoryMetrics::new());
        metrics.finger_metrics.insert(Finger::LeftIndex, CategoryMetrics::new());
        metrics.finger_metrics.insert(Finger::RightIndex, CategoryMetrics::new());
        metrics.finger_metrics.insert(Finger::RightMiddle, CategoryMetrics::new());
        metrics.finger_metrics.insert(Finger::RightRing, CategoryMetrics::new());
        metrics.finger_metrics.insert(Finger::RightPinky, CategoryMetrics::new());
        
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
        self.char_metrics.insert(' ', CharacterMetrics::new(KeyboardRow::Bottom, Finger::RightIndex));
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
            
            // Update finger metrics
            if let Some(finger_metrics) = self.finger_metrics.get_mut(&metrics.finger) {
                finger_metrics.update(time_ms, is_correct);
            }
        }
        
        // Update last keystroke time
        self.last_keystroke_time = Some(self.current_time);
        
        // Update overall metrics
        self.calculate_overall_metrics();
    }
    
    /// Calculate overall metrics like WPM and accuracy
    fn calculate_overall_metrics(&mut self) {
        // Calculate elapsed time in minutes
        let elapsed = self.current_time.duration_since(self.start_time);
        let minutes = elapsed.as_secs_f64() / 60.0;
        
        // Calculate WPM (standard 5 chars per word)
        if minutes > 0.0 {
            self.wpm = (self.keystrokes as f64 / 5.0) / minutes;
        }
        
        // Calculate accuracy
        if self.keystrokes > 0 {
            self.accuracy = (self.correct_keystrokes as f64 / self.keystrokes as f64) * 100.0;
        }
    }
    
    /// Generate a heat map for key speed
    pub fn generate_heat_map(&self) -> HashMap<char, f64> {
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
                
                heat_map.insert(*c, avg_normalized);
            }
        }
        
        heat_map
    }
    
    /// Get finger performance summary
    pub fn finger_performance(&self) -> HashMap<Finger, f64> {
        let mut performance = HashMap::new();
        
        for (finger, metrics) in &self.finger_metrics {
            if metrics.count > 0 {
                performance.insert(*finger, metrics.avg_time_ms);
            }
        }
        
        performance
    }
} 