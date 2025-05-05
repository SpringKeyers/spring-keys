use std::collections::VecDeque;
use std::time::{Duration, Instant, SystemTime};
use crossterm::event::{KeyCode, KeyModifiers};
use crate::core::TypingError;

const KEY_REPEAT_DELAY: Duration = Duration::from_millis(500);
const KEY_REPEAT_RATE: Duration = Duration::from_millis(33);

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
    pub timestamp: Instant,
    pub is_repeat: bool,
    pub last_repeat: Option<Instant>,
}

#[derive(Debug)]
pub struct InputProcessor {
    pub current_text: String,
    pub cursor_position: usize,
    pub input_buffer: VecDeque<KeyboardEvent>,
    pub last_error: Option<TypingError>,
    pub caps_lock_enabled: bool,
    pub last_key_time: Option<Instant>,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error: Option<TypingError>,
    pub suggestions: Vec<String>,
}

impl KeyboardEvent {
    pub fn new(key: KeyCode, modifiers: KeyModifiers) -> Self {
        Self {
            key,
            modifiers,
            timestamp: Instant::now(),
            is_repeat: false,
            last_repeat: None,
        }
    }

    pub fn is_repeat(&self) -> bool {
        self.is_repeat
    }

    pub fn should_repeat(&self, now: Instant) -> bool {
        if let Some(last) = self.last_repeat {
            now.duration_since(last) >= KEY_REPEAT_RATE
        } else {
            now.duration_since(self.timestamp) >= KEY_REPEAT_DELAY
        }
    }
}

impl InputProcessor {
    pub fn new() -> Self {
        Self {
            current_text: String::new(),
            cursor_position: 0,
            input_buffer: VecDeque::new(),
            last_error: None,
            caps_lock_enabled: false,
            last_key_time: None,
        }
    }

    pub fn validate_input(&self, expected: &str) -> ValidationResult {
        let current = self.current_text.as_str();
        let is_valid = current.starts_with(expected);
        
        let error = if !is_valid && !current.is_empty() {
            let pos = current.len() - 1;
            Some(TypingError {
                expected: expected.chars().nth(pos).unwrap_or(' '),
                received: current.chars().last().unwrap(),
                position: pos,
                timestamp: SystemTime::now(),
            })
        } else {
            None
        };

        ValidationResult {
            is_valid,
            error,
            suggestions: Vec::new(), // TODO: Implement suggestions
        }
    }

    pub fn process_modifiers(&mut self, event: &KeyboardEvent) {
        if event.key == KeyCode::CapsLock {
            self.caps_lock_enabled = !self.caps_lock_enabled;
        }
    }

    pub fn handle_caps_lock(&mut self, c: char) -> char {
        if self.caps_lock_enabled {
            c.to_ascii_uppercase()
        } else {
            c.to_ascii_lowercase()
        }
    }

    pub fn update_error_state(&mut self, result: ValidationResult) {
        self.last_error = result.error;
    }
} 