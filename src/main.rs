mod core;
mod input;

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

use input::{InputProcessor, KeyboardEvent};
use core::state::{GameState, GameType};

#[derive(Debug)]
struct TypingError {
    expected: char,
    received: char,
    position: usize,
    timestamp: Instant,
}

fn main() -> io::Result<()> {
    // Enable raw mode and hide cursor
    enable_raw_mode()?;
    io::stdout().execute(Hide)?;

    println!("SpringKeys Typing Tutor");
    println!("Press 'Esc' to quit\n");

    let mut processor = InputProcessor::new();
    let mut game_state = GameState::new(GameType::Practice);

    loop {
        match event::read()? {
            Event::Key(KeyEvent { code, modifiers, .. }) => {
                // Exit on Escape key
                if code == KeyCode::Esc {
                    break;
                }

                let event = KeyboardEvent::new(code, modifiers);
                
                // Process modifiers (like Caps Lock)
                processor.process_modifiers(&event);

                // Handle the key event
                match event.key {
                    KeyCode::Char(c) if event.modifiers == KeyModifiers::NONE => {
                        let c = processor.handle_caps_lock(c);
                        processor.current_text.push(c);
                        processor.cursor_position += 1;
                        
                        let mut stdout = io::stdout();
                        stdout.execute(SetForegroundColor(Color::Green))?;
                        print!("{}", c);
                        stdout.execute(ResetColor)?;
                        stdout.flush()?;
                    }
                    KeyCode::Backspace if event.modifiers == KeyModifiers::NONE => {
                        if processor.cursor_position > 0 {
                            processor.current_text.pop();
                            processor.cursor_position -= 1;
                            print!("\x08 \x08"); // Move back, print space, move back again
                            io::stdout().flush()?;
                        }
                    }
                    KeyCode::Enter if event.modifiers == KeyModifiers::NONE => {
                        println!();
                        processor.current_text.clear();
                        processor.cursor_position = 0;
                        io::stdout().flush()?;
                    }
                    _ => {}
                }

                // Store the event in the input buffer for replay/analysis
                processor.input_buffer.push_back(event);
                
                // Keep buffer size reasonable
                if processor.input_buffer.len() > 100 {
                    processor.input_buffer.pop_front();
                }
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