use std::time::{Duration, Instant, SystemTime};
use crossterm::event::{KeyCode, KeyModifiers};
use crate::core::TypingError;

mod event_queue;
pub use event_queue::{EventQueue, KeyboardEvent};

const KEY_REPEAT_DELAY: Duration = Duration::from_millis(500);
const KEY_REPEAT_RATE: Duration = Duration::from_millis(33);

#[derive(Debug)]
pub struct InputProcessor {
    pub current_text: String,
    pub cursor_position: usize,
    pub event_queue: EventQueue,
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

    pub fn process_key_event(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        let event = KeyboardEvent::new(key, modifiers);
        self.event_queue.push(event);
        self.process_modifiers(key, modifiers);
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
            suggestions: Vec::new(),
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