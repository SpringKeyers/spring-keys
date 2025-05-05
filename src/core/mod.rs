use std::time::{SystemTime, Instant};

pub mod state;
pub mod metrics;

use metrics::TypingMetrics;

#[derive(Debug)]
pub struct TypingSession {
    pub text: String,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub metrics: TypingMetrics,
    pub current_position: usize,
}

#[derive(Debug, Clone)]
pub struct TypingError {
    pub expected: char,
    pub received: char,
    pub position: usize,
    pub timestamp: SystemTime,
}

impl TypingSession {
    pub fn new(text: String) -> Self {
        Self {
            text,
            start_time: Instant::now(),
            end_time: None,
            metrics: TypingMetrics::new(),
            current_position: 0,
        }
    }
    
    pub fn record_keystroke(&mut self, c: char) {
        let expected_char = self.text.chars().nth(self.current_position).unwrap_or(' ');
        self.metrics.record_keystroke(c, expected_char, self.current_position);
        
        if c == expected_char {
            self.current_position += 1;
        }
        
        // Check if we're at the end of the text
        if self.current_position >= self.text.len() {
            self.end_time = Some(Instant::now());
        }
    }

    pub fn calculate_metrics(&mut self) {
        // The basic metrics (WPM, accuracy) are calculated by the TypingMetrics struct
        // This is now just a convenience function that can be extended later
    }
    
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.end_time = None;
        self.metrics = TypingMetrics::new();
        self.current_position = 0;
    }
} 