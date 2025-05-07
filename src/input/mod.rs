use std::time::Instant;
use crossterm::event::{KeyCode, KeyModifiers};
use crate::core::TypingSession;
use crate::ui::heatmap::register_key_press;

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

#[derive(Debug, Clone)]
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
                    let processed_char = if modifiers.contains(KeyModifiers::SHIFT) {
                        c.to_ascii_uppercase()
                    } else {
                        self.handle_caps_lock(c)
                    };
                    session.record_keystroke(processed_char);
                    // Register key press for animation
                    register_key_press(processed_char);
                },
                KeyCode::Enter => {
                    // Reset cursor position on Enter press
                    self.cursor_position = 0;
                },
                KeyCode::Backspace => {
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
                    let processed_char = if event.modifiers.contains(KeyModifiers::SHIFT) {
                        c.to_ascii_uppercase()
                    } else {
                        self.handle_caps_lock(c)
                    };
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
        
        // Check character by character up to the current input length
        for (i, (actual, expected)) in current.chars().zip(expected.chars()).enumerate() {
            if actual != expected {
                is_valid = false;
                error = Some(true);
                error_position = i;
                break;
            }
        }

        // If we've matched all characters typed so far, it's valid
        if is_valid && current.len() <= expected.len() {
            is_valid = true;
            error = None;
        }

        ValidationResult {
            is_valid,
            error,
            position: error_position,
        }
    }

    pub fn update_error_state(&mut self, result: &ValidationResult) {
        self.last_error = result.error;
    }

    pub fn clear(&mut self) {
        self.current_text.clear();
        self.cursor_position = 0;  // Reset cursor to start
        self.event_queue.clear();
        self.last_error = None;
        self.last_key_time = None;
    }

    pub fn backspace(&mut self) {
        self.handle_backspace();
    }

    /// Process a token from an automated input sequence
    /// This allows simulating key presses from a space-separated token sequence
    pub fn process_token<'a>(&mut self, token: &str, typing_session: Option<&'a mut TypingSession>) -> bool {
        let success = match token {
            "<space>" => {
                let key = KeyCode::Char(' ');
                self.process_key_event(key, KeyModifiers::NONE, typing_session);
                true
            },
            "<enter>" => {
                let key = KeyCode::Enter;
                self.process_key_event(key, KeyModifiers::NONE, typing_session);
                true
            },
            "<bs>" | "<backspace>" => {
                let key = KeyCode::Backspace;
                self.process_key_event(key, KeyModifiers::NONE, typing_session);
                true
            },
            "<tab>" => {
                let key = KeyCode::Tab;
                self.process_key_event(key, KeyModifiers::NONE, typing_session);
                true
            },
            "<esc>" => {
                let key = KeyCode::Esc;
                self.process_key_event(key, KeyModifiers::NONE, typing_session);
                true
            },
            // Control key combinations
            s if s.starts_with("<ctrl+") && s.ends_with(">") => {
                if let Some(c) = s.chars().nth(6) {
                    let key = KeyCode::Char(c);
                    self.process_key_event(key, KeyModifiers::CONTROL, typing_session);
                    true
                } else {
                    false
                }
            },
            // Shift key combinations
            s if s.starts_with("<shift+") && s.ends_with(">") => {
                if let Some(c) = s.chars().nth(7) {
                    let key = KeyCode::Char(c);
                    self.process_key_event(key, KeyModifiers::SHIFT, typing_session);
                    true
                } else {
                    false
                }
            },
            // Regular single character
            s if s.len() == 1 => {
                if let Some(c) = s.chars().next() {
                    // Skip processing spaces in token sequence
                    if c == ' ' {
                        true
                    } else {
                        let key = KeyCode::Char(c);
                        // If the character is uppercase, use SHIFT modifier
                        let modifiers = if c.is_uppercase() {
                            KeyModifiers::SHIFT
                        } else {
                            KeyModifiers::NONE
                        };
                        self.process_key_event(key, modifiers, typing_session);
                        true
                    }
                } else {
                    false
                }
            },
            // Unknown token
            _ => false
        };
        
        // Process any queued events after handling the token
        if success {
            self.process_queued_events();
        }
        
        success
    }
    
    /// Process a sequence of tokens separated by spaces
    pub fn process_token_sequence(&mut self, sequence: &str, mut typing_session: Option<&mut TypingSession>) -> usize {
        let tokens: Vec<&str> = sequence.split_whitespace().collect();
        let mut processed = 0;
        
        for token in tokens {
            // Use typing_session by reference - create a temporary reference to pass into process_token
            let session_ref = typing_session.as_deref_mut();
            if self.process_token(token, session_ref) {
                processed += 1;
            }
        }
        
        processed
    }
} 