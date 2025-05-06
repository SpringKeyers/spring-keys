use std::time::{SystemTime, Instant};

pub mod state;
pub mod metrics;
pub mod histogram;

use metrics::TypingMetrics;

#[derive(Debug)]
pub struct TypingSession {
    pub text: String,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub metrics: TypingMetrics,
    pub current_position: usize,
    pub completed_quotes: usize,
    pub total_wpm: f64,
    pub total_accuracy: f64,
    pub current_sentence_end: usize,
    pub show_up_to: usize,
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
        let first_sentence_end = find_next_sentence_end(&text, 0);
        Self {
            text: text.clone(),
            start_time: Instant::now(),
            end_time: None,
            metrics: TypingMetrics::new(),
            current_position: 0,
            completed_quotes: 0,
            total_wpm: 0.0,
            total_accuracy: 0.0,
            current_sentence_end: first_sentence_end,
            show_up_to: first_sentence_end,
        }
    }

    pub fn load_new_quote(&mut self, text: String) {
        // Save current stats before loading new quote
        if let Err(e) = self.metrics.save_to_json(&self.text) {
            eprintln!("Failed to save stats: {}", e);
        }

        // Update text and reset timing/positions
        self.text = text;
        self.reset();
    }
    
    pub fn record_keystroke(&mut self, c: char) {
        let expected_char = self.text.chars().nth(self.current_position).unwrap_or(' ');
        self.metrics.record_keystroke(c, expected_char, self.current_position);
        
        // Increment position if the character matches and we're not past the end
        if c == expected_char && self.current_position < self.text.len() {
            self.current_position += 1;
        }
    }

    pub fn get_averages(&self) -> (f64, f64) {
        if self.completed_quotes == 0 {
            (0.0, 0.0)
        } else {
            (
                self.total_wpm / self.completed_quotes as f64,
                self.total_accuracy / self.completed_quotes as f64
            )
        }
    }

    pub fn calculate_metrics(&mut self) {
        // The basic metrics (WPM, accuracy) are calculated by the TypingMetrics struct
        self.metrics.calculate_overall_metrics();
    }
    
    pub fn reset(&mut self) {
        // Reset timing
        self.start_time = Instant::now();
        self.end_time = None;
        
        // Reset position
        self.current_position = 0;
        
        // Reset sentence positions
        self.current_sentence_end = find_next_sentence_end(&self.text, 0);
        self.show_up_to = self.current_sentence_end;
    }

    pub fn advance_sentence(&mut self) {
        if self.current_position >= self.current_sentence_end {
            // Find next sentence end
            self.current_sentence_end = find_next_sentence_end(&self.text, self.current_sentence_end + 1);
            self.show_up_to = self.current_sentence_end;
        }
    }
}

// Helper function to find the end of the next sentence
pub fn find_next_sentence_end(text: &str, start_pos: usize) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let mut in_ellipsis = false;
    
    for (i, &c) in chars.iter().enumerate().skip(start_pos) {
        match c {
            '.' => {
                // Check for ellipsis
                if i + 2 < chars.len() && chars[i + 1] == '.' && chars[i + 2] == '.' {
                    in_ellipsis = true;
                    continue;
                }
                if in_ellipsis {
                    in_ellipsis = false;
                    continue;
                }
                return i + 1; // Include the period
            }
            '!' | '?' => return i + 1, // Include the punctuation mark
            _ => continue,
        }
    }
    text.len() // If no sentence end found, return end of text
} 