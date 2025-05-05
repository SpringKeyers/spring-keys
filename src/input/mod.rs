use std::time::{Duration, Instant};
use crossterm::event::{KeyCode, KeyModifiers};
use crate::core::TypingSession;

mod event_queue;
pub use event_queue::{EventQueue, KeyboardEvent};

#[derive(Debug)]
pub struct InputProcessor {
    pub current_text: String,
    pub cursor_position: usize,
    pub event_queue: EventQueue,
    pub last_error: Option<bool>,
    pub caps_lock_enabled: bool,
    pub last_key_time: Option<Instant>,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error: Option<bool>,
    pub position: usize,
}

impl InputProcessor {
    pub fn new() -> Self {
        Self {
            current_text: String::new(),
            cursor_position: 0,
            event_queue: EventQueue::new(),
            last_error: None,
            caps_lock_enabled: false,
            last_key_time: None,
        }
    }

    pub fn process_key_event(&mut self, key: KeyCode, modifiers: KeyModifiers, typing_session: Option<&mut TypingSession>) {
        let event = KeyboardEvent::new(key, modifiers);
        self.event_queue.push(event);
        self.process_modifiers(key, modifiers);
        
        // Record the keystroke in the typing session metrics
        if let Some(session) = typing_session {
            match key {
                KeyCode::Char(c) => {
                    let processed_char = self.handle_caps_lock(c);
                    session.record_keystroke(processed_char);
                },
                KeyCode::Backspace => {
                    // For backspace, we could add a special tracking method
                    // but for now we'll just record it as a special character
                    session.record_keystroke('\u{232B}'); // Unicode backspace symbol
                },
                _ => {} // Ignore other keys for metrics tracking
            }
        }
        
        self.last_key_time = Some(Instant::now());
    }

    pub fn process_queued_events(&mut self) {
        self.event_queue.cleanup_old_events();
        
        while let Some(event) = self.event_queue.pop() {
            match event.key {
                KeyCode::Char(c) => {
                    let processed_char = self.handle_caps_lock(c);
                    self.insert_char(processed_char);
                }
                KeyCode::Backspace => self.handle_backspace(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                _ => {}
            }
        }
    }

    fn process_modifiers(&mut self, key: KeyCode, _modifiers: KeyModifiers) {
        if key == KeyCode::CapsLock {
            self.caps_lock_enabled = !self.caps_lock_enabled;
        }
    }

    pub fn handle_caps_lock(&self, c: char) -> char {
        if self.caps_lock_enabled {
            c.to_ascii_uppercase()
        } else {
            c.to_ascii_lowercase()
        }
    }

    fn insert_char(&mut self, c: char) {
        self.current_text.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    fn handle_backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.current_text.remove(self.cursor_position);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.current_text.len() {
            self.cursor_position += 1;
        }
    }

    pub fn validate_input(&self, expected: &str) -> ValidationResult {
        let current = self.current_text.as_str();
        let mut is_valid = true;
        let mut error = None;
        let mut error_position = 0;
        
        // Check character by character
        for (i, (actual, expected)) in current.chars().zip(expected.chars()).enumerate() {
            if actual != expected {
                is_valid = false;
                error = Some(true);
                error_position = i;
                break;
            }
        }
        
        // Check if the input is longer than expected
        if current.len() > expected.len() {
            is_valid = false;
            error = Some(true);
            error_position = expected.len();
        }

        ValidationResult {
            is_valid,
            error,
            position: error_position,
        }
    }

    pub fn update_error_state(&mut self, result: ValidationResult) {
        self.last_error = result.error;
    }

    pub fn clear(&mut self) {
        self.current_text.clear();
        self.cursor_position = 0;
        self.event_queue.clear();
        self.last_error = None;
        self.last_key_time = None;
    }
} 