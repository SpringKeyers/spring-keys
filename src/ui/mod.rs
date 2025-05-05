use std::io::{self, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use crate::SpringKeys;

mod heatmap;
use heatmap::KeyboardHeatmap;

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
        // Initialize with a random typing text from the quotes database
        app.start_typing_session(None);
        
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
                    
                    // Handle F5 key to get a new random quote
                    if key_event.code == KeyCode::F(5) {
                        app.start_typing_session(None);
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
        
        // Draw metrics header bar
        self.render_metrics_header(app, &mut stdout)?;
        
        // Draw title
        stdout.execute(MoveTo(1, 1))?;
        stdout.execute(SetForegroundColor(Color::Cyan))?;
        stdout.write_all(b"SpringKeys Typing Tutor")?;
        stdout.execute(ResetColor)?;
        
        // Draw instructions
        stdout.execute(MoveTo(1, 3))?;
        stdout.write_all(b"Press ESC to quit, F5 for new quote, arrow keys to move cursor")?;
        
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
            // Get current quote source (if available)
            let current_quote = app.quote_db.current();
            
            // Draw text source
            stdout.execute(MoveTo(1, 9))?;
            stdout.execute(SetForegroundColor(Color::Blue))?;
            stdout.write_all(format!("Source: {}", current_quote.source).as_bytes())?;
            stdout.execute(ResetColor)?;
            
            stdout.execute(MoveTo(1, 11))?;
            stdout.write_all(session.text.as_bytes())?;
            
            // Draw user input area
            stdout.execute(MoveTo(1, 13))?;
            stdout.write_all(b"Your input:")?;
            
            // Draw input text with character-by-character comparison
            stdout.execute(MoveTo(1, 14))?;
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
            stdout.execute(MoveTo(1 + cursor_pos as u16, 14))?;
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
            stdout.execute(MoveTo(1, 16))?;
            stdout.write_all(format!("WPM: {:.1} | Accuracy: {:.1}%", 
                session.metrics.wpm, 
                session.metrics.accuracy).as_bytes())?;
            
            // Draw error count
            stdout.execute(MoveTo(1, 17))?;
            stdout.write_all(format!("Errors: {}", session.metrics.errors.len()).as_bytes())?;
            
            // Draw quote difficulty
            stdout.execute(MoveTo(1, 18))?;
            stdout.write_all(format!("Difficulty: {:?} | Category: {:?} | Origin: {}", 
                current_quote.difficulty, 
                current_quote.category,
                current_quote.origin).as_bytes())?;
                
            // Draw detailed metrics
            self.render_detailed_metrics(app, &mut stdout)?;
        }
        
        stdout.flush()?;
        Ok(())
    }
    
    fn render_metrics_header(&self, app: &SpringKeys, stdout: &mut io::Stdout) -> io::Result<()> {
        // Create the metrics header at the top of the screen
        stdout.execute(MoveTo(0, 0))?;
        stdout.execute(SetBackgroundColor(Color::DarkBlue))?;
        
        // Fill the entire line
        for x in 0..self.width {
            stdout.execute(MoveTo(x, 0))?;
            stdout.write_all(b" ")?;
        }
        
        if let Some(session) = &app.typing_session {
            // Calculate short-term averages to display
            let letter_avg = session.metrics.letter_metrics.short_term_avg_ms;
            let number_avg = session.metrics.number_metrics.short_term_avg_ms;
            let homerow_avg = session.metrics.home_row_metrics.short_term_avg_ms;
            let toprow_avg = session.metrics.top_row_metrics.short_term_avg_ms;
            let bottomrow_avg = session.metrics.bottom_row_metrics.short_term_avg_ms;
            
            // Format the metrics header with colorized performance indicators
            stdout.execute(MoveTo(1, 0))?;
            stdout.execute(SetForegroundColor(Color::White))?;
            stdout.write_all(b"Speed (ms):")?;
            
            let x_offset = 12;
            
            // Letters average
            self.draw_metric_value(stdout, x_offset, 0, "Let:", letter_avg)?;
            
            // Numbers average
            self.draw_metric_value(stdout, x_offset + 10, 0, "Num:", number_avg)?;
            
            // Home row average
            self.draw_metric_value(stdout, x_offset + 20, 0, "Home:", homerow_avg)?;
            
            // Top row average
            self.draw_metric_value(stdout, x_offset + 30, 0, "Top:", toprow_avg)?;
            
            // Bottom row average
            self.draw_metric_value(stdout, x_offset + 40, 0, "Bot:", bottomrow_avg)?;
            
            // Add real-time WPM
            stdout.execute(MoveTo(x_offset + 52, 0))?;
            stdout.execute(SetForegroundColor(Color::White))?;
            stdout.write_all(format!("WPM: {:.1}", session.metrics.wpm).as_bytes())?;
            
            // Add accuracy
            stdout.execute(MoveTo(x_offset + 65, 0))?;
            stdout.write_all(format!("Acc: {:.1}%", session.metrics.accuracy).as_bytes())?;
        }
        
        stdout.execute(ResetColor)?;
        Ok(())
    }
    
    fn draw_metric_value(&self, stdout: &mut io::Stdout, x: u16, y: u16, label: &str, value: f64) -> io::Result<()> {
        stdout.execute(MoveTo(x, y))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.write_all(label.as_bytes())?;
        
        // Choose color based on typing speed
        let color = if value <= 150.0 {
            Color::Green // Fast
        } else if value <= 250.0 {
            Color::Yellow // Medium
        } else {
            Color::Red // Slow
        };
        
        stdout.execute(SetForegroundColor(color))?;
        stdout.write_all(format!("{:.0}", value).as_bytes())?;
        
        Ok(())
    }
    
    fn render_detailed_metrics(&self, app: &SpringKeys, stdout: &mut io::Stdout) -> io::Result<()> {
        if let Some(session) = &app.typing_session {
            // Draw keyboard heatmap
            stdout.execute(MoveTo(1, 25))?;
            stdout.execute(SetForegroundColor(Color::Cyan))?;
            stdout.write_all(b"Keyboard Speed Heatmap (blue=fast, red=slow):")?;
            stdout.execute(ResetColor)?;
            
            if let Some(heat_map) = app.get_heat_map() {
                // Position the keyboard heatmap centrally
                let x_pos = 5;
                let y_pos = 27;
                KeyboardHeatmap::render(&heat_map, stdout, x_pos, y_pos)?;
            }
            
            // Draw finger performance section
            stdout.execute(MoveTo(1, 20))?;
            stdout.execute(SetForegroundColor(Color::Cyan))?;
            stdout.write_all(b"Finger Performance (ms):")?;
            stdout.execute(ResetColor)?;
            
            // Get finger performance metrics and render with bars
            if let Some(finger_perf) = app.get_finger_performance() {
                KeyboardHeatmap::render_finger_performance(stdout, 1, 20, 20, &finger_perf)?;
            }
            
            // Draw row performance section with performance bars
            stdout.execute(MoveTo(1, 40))?;
            stdout.execute(SetForegroundColor(Color::Cyan))?;
            stdout.write_all(b"Row Performance (ms):")?;
            stdout.execute(ResetColor)?;
            
            KeyboardHeatmap::render_row_performance(
                stdout, 
                1, 
                40, 
                30,
                session.metrics.top_row_metrics.avg_time_ms,
                session.metrics.home_row_metrics.avg_time_ms,
                session.metrics.bottom_row_metrics.avg_time_ms
            )?;
        }
        
        Ok(())
    }
} 