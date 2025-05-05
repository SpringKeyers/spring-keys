use std::time::Instant;
use super::metrics::TypingMetrics;
use crate::quotes::Quote;

/// Represents a typing session with metrics
pub struct TypingSession {
    /// The text to type
    pub text: String,
    /// Typing metrics for this session
    pub metrics: TypingMetrics,
    /// Start time of the session
    pub start_time: Instant,
    /// Number of completed quotes in this session
    pub completed_quotes: usize,
}

impl TypingSession {
    /// Create a new typing session
    pub fn new(quote: &Quote) -> Self {
        Self {
            text: quote.text.clone(),
            metrics: TypingMetrics::new(),
            start_time: Instant::now(),
            completed_quotes: 0,
        }
    }

    /// Get the average WPM and accuracy across all quotes
    pub fn get_averages(&self) -> (f64, f64) {
        // For now, just return current values since we don't track historical data
        (self.metrics.wpm, self.metrics.accuracy)
    }

    /// Start a new quote
    pub fn load_new_quote(&mut self, quote: &Quote) {
        // Save current stats if we have any text
        if !self.text.is_empty() {
            if let Err(e) = self.metrics.save_to_json(&self.text) {
                eprintln!("Failed to save stats: {}", e);
            }
        }

        // Reset for new quote
        self.text = quote.text.clone();
        self.metrics = TypingMetrics::new();
        self.start_time = Instant::now();
        self.completed_quotes += 1;
    }
} 