use std::io::{self, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use crate::SpringKeys;

pub struct TerminalUI {
    width: u16,
    height: u16,
    should_quit: bool,
}

impl TerminalUI {
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        
        Ok(Self {
            width,
            height,
            should_quit: false,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        io::stdout().execute(Hide)?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        io::stdout().execute(LeaveAlternateScreen)?;
        io::stdout().execute(Show)?;
        Ok(())
    }

    pub fn run(&mut self, app: &mut SpringKeys) -> io::Result<()> {
        // Initialize with a sample typing text
        let sample_text = "The quick brown fox jumps over the lazy dog.".to_string();
        app.start_typing_session(sample_text);
        
        while !self.should_quit {
            self.render(app)?;
            
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    // Process exit command (Ctrl+C or Esc)
                    if key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL 
                        || key_event.code == KeyCode::Esc {
                        self.should_quit = true;
                        continue;
                    }
                    
                    // Pass the key event to the application
                    app.process_input(key_event.code, key_event.modifiers);
                }
            }
        }
        
        Ok(())
    }
    
    fn render(&self, app: &SpringKeys) -> io::Result<()> {
        let mut stdout = io::stdout();
        
        // Clear the screen
        stdout.execute(Clear(ClearType::All))?;
        
        // Draw title
        stdout.execute(MoveTo(1, 1))?;
        stdout.execute(SetForegroundColor(Color::Cyan))?;
        stdout.write_all(b"SpringKeys Typing Tutor")?;
        stdout.execute(ResetColor)?;
        
        // Draw instructions
        stdout.execute(MoveTo(1, 3))?;
        stdout.write_all(b"Press ESC to quit, arrow keys to move cursor")?;
        
        // Draw game status
        stdout.execute(MoveTo(1, 5))?;
        stdout.execute(SetForegroundColor(Color::Yellow))?;
        stdout.write_all(format!("Game: {:?} | Status: {:?}", 
            app.game_state.current_game, 
            app.game_state.status).as_bytes())?;
        stdout.execute(ResetColor)?;
        
        // Draw separator
        stdout.execute(MoveTo(1, 6))?;
        stdout.write_all(b"------------------------------------------------")?;
        
        // Draw typing area title
        stdout.execute(MoveTo(1, 8))?;
        stdout.execute(SetForegroundColor(Color::Green))?;
        stdout.write_all(b"Type the following text:")?;
        stdout.execute(ResetColor)?;
        
        // Draw target text
        if let Some(session) = &app.typing_session {
            stdout.execute(MoveTo(1, 10))?;
            stdout.write_all(session.text.as_bytes())?;
            
            // Draw user input area
            stdout.execute(MoveTo(1, 12))?;
            stdout.write_all(b"Your input:")?;
            
            // Draw input text with character-by-character comparison
            stdout.execute(MoveTo(1, 13))?;
            let input_text = &app.input_processor.current_text;
            
            for (i, c) in input_text.chars().enumerate() {
                let target_char = session.text.chars().nth(i);
                
                if let Some(target) = target_char {
                    if c == target {
                        // Correct character - green
                        stdout.execute(SetForegroundColor(Color::Green))?;
                    } else {
                        // Incorrect character - red
                        stdout.execute(SetForegroundColor(Color::Red))?;
                    }
                } else {
                    // Extra character - yellow
                    stdout.execute(SetForegroundColor(Color::Yellow))?;
                }
                
                stdout.write_all(&[c as u8])?;
                stdout.execute(ResetColor)?;
            }
            
            // Draw cursor
            let cursor_pos = app.input_processor.cursor_position;
            stdout.execute(MoveTo(1 + cursor_pos as u16, 13))?;
            stdout.execute(SetBackgroundColor(Color::DarkGrey))?;
            
            if cursor_pos < input_text.len() {
                let cursor_char = input_text.chars().nth(cursor_pos).unwrap_or(' ');
                stdout.execute(SetForegroundColor(Color::White))?;
                stdout.write_all(&[cursor_char as u8])?;
            } else {
                stdout.write_all(b" ")?;
            }
            stdout.execute(ResetColor)?;
            
            // Draw metrics
            stdout.execute(MoveTo(1, 15))?;
            stdout.write_all(format!("WPM: {:.1} | Accuracy: {:.1}%", 
                session.metrics.wpm, 
                session.metrics.accuracy).as_bytes())?;
            
            // Draw error count
            stdout.execute(MoveTo(1, 16))?;
            stdout.write_all(format!("Errors: {}", session.metrics.errors.len()).as_bytes())?;
        }
        
        stdout.flush()?;
        Ok(())
    }
} 