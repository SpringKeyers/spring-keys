use std::time::{SystemTime, Instant};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

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
    log_file: Option<std::fs::File>,
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
        // Open log file with append mode
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("typing_stats.log")
            .ok();
        
        Self {
            text,
            start_time: Instant::now(),
            end_time: None,
            metrics: TypingMetrics::new(),
            current_position: 0,
            completed_quotes: 0,
            total_wpm: 0.0,
            total_accuracy: 0.0,
            log_file,
        }
    }

    pub fn load_new_quote(&mut self, text: String) {
        // Save current stats before loading new quote
        if let Err(e) = self.metrics.save_to_json(&self.text) {
            eprintln!("Failed to save stats: {}", e);
        }

        // Update session stats
        self.completed_quotes += 1;
        self.total_wpm += self.metrics.wpm;
        self.total_accuracy += self.metrics.accuracy;

        // Prepare metrics for new quote while maintaining overall stats
        self.metrics.prepare_for_new_quote();

        // Update text and reset timing
        self.text = text;
        self.start_time = Instant::now();
        self.end_time = None;
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

    pub fn check_completion(&mut self) -> bool {
        if self.current_position >= self.text.len() && self.text.ends_with('.') {
            // Update averages
            self.completed_quotes += 1;
            self.total_wpm += self.metrics.wpm;
            self.total_accuracy += self.metrics.accuracy;

            // Log completion stats
            if let Some(log_file) = &mut self.log_file {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
                let stats = format!(
                    "\n[{}] Quote completed:\n{}\nWPM: {:.1}\nAccuracy: {:.1}%\nAvg WPM: {:.1}\nAvg Accuracy: {:.1}%\n",
                    timestamp,
                    self.text,
                    self.metrics.wpm,
                    self.metrics.accuracy,
                    self.total_wpm / self.completed_quotes as f64,
                    self.total_accuracy / self.completed_quotes as f64
                );

                // Log row performance
                let row_stats = format!(
                    "Row Performance (ms):\n  Top: {:.1} (10s: {:.1}, 60s: {:.1}, Quote: {:.1})\n  Home: {:.1} (10s: {:.1}, 60s: {:.1}, Quote: {:.1})\n  Bottom: {:.1} (10s: {:.1}, 60s: {:.1}, Quote: {:.1})\n",
                    self.metrics.top_row_metrics.avg_time_ms,
                    self.metrics.top_row_metrics.short_term_avg_ms,
                    self.metrics.top_row_metrics.avg_time_ms,
                    self.metrics.top_row_metrics.avg_time_ms,
                    self.metrics.home_row_metrics.avg_time_ms,
                    self.metrics.home_row_metrics.short_term_avg_ms,
                    self.metrics.home_row_metrics.avg_time_ms,
                    self.metrics.home_row_metrics.avg_time_ms,
                    self.metrics.bottom_row_metrics.avg_time_ms,
                    self.metrics.bottom_row_metrics.short_term_avg_ms,
                    self.metrics.bottom_row_metrics.avg_time_ms,
                    self.metrics.bottom_row_metrics.avg_time_ms,
                );

                // Log finger performance with extended stats
                let mut finger_stats = String::from("Finger Performance (ms):\n");
                for (finger, stats) in &self.metrics.finger_metrics {
                    finger_stats.push_str(&format!(
                        "  {:?}:\n    Current: {:.1}\n    10s Avg: {:.1}\n    60s Avg: {:.1}\n    Quote Avg: {:.1}\n    Fastest: {:.1}\n    Slowest: {:.1}\n",
                        finger,
                        stats.current,
                        stats.avg_10s,
                        stats.avg_60s,
                        stats.avg_quote,
                        stats.fastest,
                        stats.slowest
                    ));
                }

                let separator = "\n----------------------------------------\n";
                
                if let Err(e) = writeln!(log_file, "{}{}{}{}", stats, row_stats, finger_stats, separator) {
                    eprintln!("Failed to write to log file: {}", e);
                }
            }

            true
        } else {
            false
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
        self.start_time = Instant::now();
        self.end_time = None;
        self.metrics = TypingMetrics::new();
        self.current_position = 0;
    }
} 