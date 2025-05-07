use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use log::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccumulatedStats {
    pub total_quotes: usize,
    pub total_keystrokes: usize,
    pub total_errors: usize,
    pub session_errors: usize,  // Track errors for the current session
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
}

impl AccumulatedStats {
    pub fn new() -> Self {
        Self {
            total_quotes: 0,
            total_keystrokes: 0,
            total_errors: 0,
            session_errors: 0,
            avg_wpm: 0.0,
            avg_accuracy: 0.0,
        }
    }

    pub fn load_from_directory() -> Self {
        // Create a new instance with default values
        let mut stats = Self::new();

        // Create stats directory if it doesn't exist
        let stats_dir = PathBuf::from("stats");
        if !stats_dir.exists() {
            if let Err(e) = fs::create_dir_all(&stats_dir) {
                info!("Failed to create stats directory: {}", e);
                return stats;
            }
        }

        // Return default stats if directory is empty
        stats
    }

    pub fn update_from_session(&mut self, session: &crate::core::TypingSession) {
        self.total_quotes += 1;
        self.total_keystrokes += session.metrics.keystrokes;
        
        // Don't update error counts here since we're tracking them in real-time
        // during input processing

        // Update running averages
        let (wpm, accuracy) = session.get_averages();
        if self.avg_wpm == 0.0 {
            self.avg_wpm = wpm;
        } else {
            self.avg_wpm = 0.95 * self.avg_wpm + 0.05 * wpm;
        }

        if self.avg_accuracy == 0.0 {
            self.avg_accuracy = accuracy;
        } else {
            self.avg_accuracy = 0.95 * self.avg_accuracy + 0.05 * accuracy;
        }
    }
} 