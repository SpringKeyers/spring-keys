use std::time::SystemTime;

pub mod state;

#[derive(Debug)]
pub struct TypingSession {
    pub text: String,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub metrics: TypingMetrics,
}

#[derive(Debug, Default)]
pub struct TypingMetrics {
    pub wpm: f32,
    pub accuracy: f32,
    pub errors: Vec<TypingError>,
    pub total_keystrokes: usize,
    pub correct_keystrokes: usize,
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
            start_time: SystemTime::now(),
            end_time: None,
            metrics: TypingMetrics::default(),
        }
    }

    pub fn calculate_metrics(&mut self) {
        if let Some(end_time) = self.end_time {
            if let Ok(duration) = end_time.duration_since(self.start_time) {
                let minutes = duration.as_secs_f32() / 60.0;
                let words = self.text.split_whitespace().count() as f32;
                self.metrics.wpm = words / minutes;
            }
        }

        if self.metrics.total_keystrokes > 0 {
            self.metrics.accuracy = self.metrics.correct_keystrokes as f32 
                / self.metrics.total_keystrokes as f32 
                * 100.0;
        }
    }
} 