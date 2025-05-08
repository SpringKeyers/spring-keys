use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    style::{Color, Print, SetForegroundColor, ResetColor, SetBackgroundColor},
    cursor::{MoveTo, Hide, Show},
    queue,
    execute,
};
use std::io::{self, Write, Stdout};
use crate::SpringKeys;
use std::time::Duration;
use crate::quotes::CategoryCycle;

pub mod heatmap;
pub mod color_spectrum;

pub struct TerminalUI {
    stdout: Stdout,
    should_quit: bool,
    terminal_size: (u16, u16),
}

impl TerminalUI {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            stdout: io::stdout(),
            should_quit: false,
            terminal_size: terminal::size()?,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        // Enable raw mode
        enable_raw_mode()?;
        
        // Switch to alternate screen and hide cursor
        execute!(
            self.stdout,
            EnterAlternateScreen,
            Hide,
            Clear(ClearType::All)
        )?;

        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        // Show cursor and leave alternate screen
        execute!(
            self.stdout,
            Show,
            LeaveAlternateScreen
        )?;
        
        // Disable raw mode
        disable_raw_mode()?;
        
        Ok(())
    }

    pub fn run(&mut self, app: &mut SpringKeys) -> io::Result<()> {
        // Initialize with a random typing text from the quotes database
        app.start_typing_session(None);
        
        // Check if demo heatmap mode is enabled via an environment variable
        let demo_heatmap = std::env::var("SPRING_KEYS_DEMO_HEATMAP").is_ok();
        
        // Apply demo data if in demo heatmap mode
        if demo_heatmap {
            // Set up demo typing data for spectrum visualization
            if let Some(session) = &mut app.typing_session {
                session.metrics.simulate_demo_data();
            }
        }
        
        while !self.should_quit {
            self.draw_ui(app)?;
            
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    // Process exit command (Ctrl+C or Esc)
                    if key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL 
                        || key_event.code == KeyCode::Esc {
                        self.should_quit = true;
                        continue;
                    }
                    
                    match key_event.code {
                        KeyCode::Enter => {
                            // Clear input and load new quote without resetting stats
                            app.input_processor.clear();
                            app.start_typing_session(None);
                        },
                        KeyCode::F(5) => {
                            // Load a new random quote
                            app.start_typing_session(None);
                        },
                        KeyCode::F(6) => {
                            // Switch to typewriter quotes
                            app.quote_db.set_active_category(CategoryCycle::Typewriter);
                            app.start_typing_session(None);
                        },
                        KeyCode::F(7) => {
                            // Switch to programming quotes
                            app.quote_db.set_active_category(CategoryCycle::Programming);
                            app.start_typing_session(None);
                        },
                        KeyCode::F(8) => {
                            // TODO: investigate quote transition on F8 key (Literature category)
                            // Switch to literature quotes
                            app.quote_db.set_active_category(CategoryCycle::Literature);
                            app.start_typing_session(None);
                        },
                        KeyCode::Backspace => {
                            // Remove the last character from input
                            app.input_processor.backspace();
                        },
                        _ => {
                            app.process_input(key_event.code, key_event.modifiers);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn render_frame(&mut self, app: &SpringKeys) -> io::Result<()> {
        self.draw_ui(app)
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn draw_ui(&mut self, app: &SpringKeys) -> io::Result<()> {
        // Instead of clearing the whole screen, we'll just reset cursor
        queue!(self.stdout, MoveTo(0, 0))?;
        
        // Draw active categories
        let active_categories = format!(
            "Active: Type:{:?} Prog:{:?} Lit:{:?}",
            app.quote_db.get_active_category(),
            app.quote_db.get_active_category(),
            app.quote_db.get_active_category(),
        );
        queue!(
            self.stdout,
            MoveTo(0, 0),
            SetForegroundColor(Color::Yellow),
            Print(&active_categories),
            ResetColor
        )?;

        // Draw metrics if there's an active session
        if let Some(session) = &app.typing_session {
            let metrics_text = format!(
                "Current WPM: {:.1} | Acc: {:.1}% | All-time WPM: {:.1} | All-time Acc: {:.1}% | Total Quotes: {}",
                session.metrics.wpm,
                session.metrics.accuracy,
                app.accumulated_stats.avg_wpm,
                app.accumulated_stats.avg_accuracy,
                app.accumulated_stats.total_quotes
            );
            queue!(
                self.stdout,
                MoveTo(0, 1),
                SetForegroundColor(Color::Green),
                Print(&metrics_text),
                ResetColor
            )?;

            // Draw unified keyboard heatmap with color temperature and hit counts
            heatmap::draw_unified_keyboard_heatmap(&mut self.stdout, &session.metrics, 3)?;

            // Draw typing area at a position below the visualization
            let typing_area_y = 35;
            
            // Get error count
            let error_count = session.metrics.errors.len();

            // Clear the entire typing area first (5 lines: errors, top cursor, quote, input, bottom cursor)
            for y in typing_area_y..typing_area_y+6 {
                queue!(
                    self.stdout,
                    MoveTo(0, y),
                    Print(" ".repeat(self.terminal_size.0 as usize))
                )?;
            }

            // Draw speed range and error counts
            let speed_range = format!(
                "Speed Range: {}ms (fastest) to {}ms (slowest) | Errors: {} (Session: {}, Total: {})",
                session.metrics.key_histogram.min as u64,
                session.metrics.key_histogram.max as u64,
                error_count,
                app.accumulated_stats.session_errors,
                app.accumulated_stats.total_errors
            );
            queue!(
                self.stdout,
                MoveTo(0, typing_area_y),
                SetForegroundColor(Color::White),
                Print(&speed_range),
                ResetColor
            )?;

            // Draw the quote text
            queue!(
                self.stdout,
                MoveTo(0, typing_area_y + 2),
                SetForegroundColor(Color::White),
                Print(&session.quote_text),
                ResetColor
            )?;

            // Draw typing area at a position below the visualization
            let typing_area_y = 35;
            
            // Get error count
            let error_count = session.metrics.errors.len();
            let total_keystrokes = app.accumulated_stats.total_keystrokes;

            // Clear the entire typing area first (5 lines: errors, top cursor, quote, input, bottom cursor)
            for y in typing_area_y..typing_area_y+6 {
                queue!(
                    self.stdout,
                    MoveTo(0, y),
                    Print(" ".repeat(self.terminal_size.0 as usize))
                )?;
            }

            // Draw error counts and total keystrokes
            queue!(
                self.stdout,
                MoveTo(0, typing_area_y),
                SetForegroundColor(Color::White),
                Print(format!("Errors: {} (Total Keys: {})", error_count, total_keystrokes)),
                ResetColor
            )?;

            // Draw the quote text
            queue!(
                self.stdout,
                MoveTo(0, typing_area_y + 2),
                SetForegroundColor(Color::White),
                Print(&session.quote_text),
                ResetColor
            )?;

            // Draw the input text with cursor
            let input_text = &app.input_processor.current_text;
            let cursor_pos = app.input_processor.cursor_position;
            
            // Draw input text
            queue!(
                self.stdout,
                MoveTo(0, typing_area_y + 3),
                SetForegroundColor(Color::Cyan)
            )?;

            // Draw text before cursor
            if cursor_pos > 0 {
                queue!(self.stdout, Print(&input_text[..cursor_pos]))?;
            }

            // Draw cursor
            queue!(
                self.stdout,
                SetBackgroundColor(Color::White),
                SetForegroundColor(Color::Black),
                Print(if cursor_pos < input_text.len() {
                    input_text[cursor_pos..=cursor_pos].to_string()
                } else {
                    " ".to_string()
                }),
                ResetColor
            )?;

            // Draw text after cursor
            if cursor_pos < input_text.len() {
                queue!(
                    self.stdout,
                    SetForegroundColor(Color::Cyan),
                    Print(&input_text[cursor_pos + 1..]),
                    ResetColor
                )?;
            }

            // Draw cursors at the current position
            let cursor_x = app.input_processor.cursor_position as u16;
            
            // Top cursor
            queue!(
                self.stdout,
                MoveTo(cursor_x, typing_area_y + 1),
                Print("▼")
            )?;

            // Bottom cursor
            queue!(
                self.stdout,
                MoveTo(cursor_x, typing_area_y + 4),
                Print("▲")
            )?;

            // Add the underline
            queue!(
                self.stdout,
                MoveTo(0, typing_area_y + 5),
                Print("─".repeat(session.quote_text.len()))
            )?;
        }

        // Draw category indicators
        queue!(
            self.stdout,
            MoveTo(0, self.terminal_size.1 - 2),
            SetForegroundColor(Color::DarkGrey)
        )?;

        let active_category = app.quote_db.get_active_category();
        let category_indicators = [
            (CategoryCycle::Typewriter, "⌨"),
            (CategoryCycle::Programming, "⚡"),
            (CategoryCycle::Literature, "📚"),
        ];

        for (category, symbol) in &category_indicators {
            if *category == active_category {
                queue!(
                    self.stdout,
                    SetForegroundColor(Color::White),
                    Print(symbol),
                    SetForegroundColor(Color::DarkGrey),
                    Print(" ")
                )?;
            } else {
                queue!(
                    self.stdout,
                    Print(symbol),
                    Print(" ")
                )?;
            }
        }

        // Flush all queued changes to the terminal
        self.stdout.flush()?;
        Ok(())
    }
} 