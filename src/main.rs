use std::io::{self, Write};
use std::time::Instant;
use std::collections::VecDeque;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
    cursor::{Hide, Show},
    style::{Color, SetForegroundColor, ResetColor},
};

#[derive(Debug)]
struct KeyboardEvent {
    key: KeyCode,
    modifiers: KeyModifiers,
    timestamp: Instant,
}

#[derive(Debug)]
struct InputProcessor {
    current_text: String,
    cursor_position: usize,
    input_buffer: VecDeque<KeyboardEvent>,
    last_error: Option<TypingError>,
}

#[derive(Debug)]
struct TypingError {
    expected: char,
    received: char,
    position: usize,
    timestamp: Instant,
}

impl InputProcessor {
    fn new() -> Self {
        Self {
            current_text: String::new(),
            cursor_position: 0,
            input_buffer: VecDeque::new(),
            last_error: None,
        }
    }

    fn process_key(&mut self, key_event: KeyboardEvent) -> io::Result<()> {
        let mut stdout = io::stdout();

        match key_event.key {
            KeyCode::Char(c) if key_event.modifiers == KeyModifiers::NONE => {
                self.current_text.push(c);
                self.cursor_position += 1;
                stdout.execute(SetForegroundColor(Color::Green))?;
                print!("{}", c);
                stdout.execute(ResetColor)?;
            }
            KeyCode::Backspace if key_event.modifiers == KeyModifiers::NONE => {
                if self.cursor_position > 0 {
                    self.current_text.pop();
                    self.cursor_position -= 1;
                    print!("\x08 \x08"); // Move back, print space, move back again
                }
            }
            KeyCode::Enter if key_event.modifiers == KeyModifiers::NONE => {
                println!();
                self.current_text.clear();
                self.cursor_position = 0;
            }
            _ => {}
        }

        stdout.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Enable raw mode and hide cursor
    enable_raw_mode()?;
    io::stdout().execute(Hide)?;

    println!("SpringKeys Typing Tutor");
    println!("Press 'Esc' to quit\n");

    let mut processor = InputProcessor::new();

    loop {
        match event::read()? {
            Event::Key(KeyEvent { code, modifiers, .. }) => {
                // Exit on Escape key
                if code == KeyCode::Esc {
                    break;
                }

                let event = KeyboardEvent {
                    key: code,
                    modifiers,
                    timestamp: Instant::now(),
                };

                processor.process_key(event)?;
            }
            _ => {}
        }
    }

    // Clean up: show cursor and disable raw mode
    io::stdout().execute(Show)?;
    disable_raw_mode()?;
    
    println!("\nThank you for using SpringKeys!");
    Ok(())
} 