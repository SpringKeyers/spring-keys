use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, enable_raw_mode, disable_raw_mode, Clear, ClearType},
    style::{Color, Print, SetForegroundColor, ResetColor},
    cursor::{MoveTo, Hide, Show},
    ExecutableCommand,
    queue,
};
use std::io::{self, Write, Stdout};
use crate::SpringKeys;
use std::time::Duration;

pub mod heatmap;
pub mod histogram_display;
pub mod color_spectrum;

pub struct TerminalUI {
    stdout: Stdout,
    should_quit: bool,
    terminal_size: (u16, u16),
}

impl TerminalUI {
    pub fn new() -> io::Result<Self> {
        let stdout = io::stdout();
        let terminal_size = terminal::size()?;
        
        Ok(Self {
            stdout,
            should_quit: false,
            terminal_size,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        self.stdout.execute(Hide)?;
        self.stdout.execute(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        self.stdout.execute(Show)?;
        self.stdout.execute(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn run(&mut self, app: &mut SpringKeys) -> io::Result<()> {
        // Initialize with a random typing text from the quotes database
        app.start_typing_session(None);
        
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
                    
                    // Handle F-keys for category cycling
                    match key_event.code {
                        KeyCode::F(5) => {
                            app.start_typing_session(None);
                        },
                        KeyCode::F(6) => {
                            let category = app.quote_db.cycle_category(crate::quotes::CategoryCycle::Typewriter);
                            let quote_text = app.quote_db.next_by_category(category)
                                .map(|quote| quote.text.clone());
                            if let Some(text) = quote_text {
                                app.start_typing_session(Some(text));
                            }
                        },
                        KeyCode::F(7) => {
                            let category = app.quote_db.cycle_category(crate::quotes::CategoryCycle::Programming);
                            let quote_text = app.quote_db.next_by_category(category)
                                .map(|quote| quote.text.clone());
                            if let Some(text) = quote_text {
                                app.start_typing_session(Some(text));
                            }
                        },
                        KeyCode::F(8) => {
                            let category = app.quote_db.cycle_category(crate::quotes::CategoryCycle::Literature);
                            let quote_text = app.quote_db.next_by_category(category)
                                .map(|quote| quote.text.clone());
                            if let Some(text) = quote_text {
                                app.start_typing_session(Some(text));
                            }
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

    fn draw_ui(&mut self, app: &SpringKeys) -> io::Result<()> {
        // Clear screen
        queue!(self.stdout, Clear(ClearType::All))?;
        
        // Draw title (centered)
        let title = "SpringKeys Typing Tutor";
        let title_pos = (self.terminal_size.0 as usize / 2).saturating_sub(title.len() / 2) as u16;
        queue!(
            self.stdout,
            MoveTo(title_pos, 1),
            SetForegroundColor(Color::Cyan),
            Print(title),
            ResetColor
        )?;

        // Draw instructions
        let instructions = "ESC: Quit | F5: Random Quote | F6: Typewriter | F7: Programming | F8: Literature";
        queue!(
            self.stdout,
            MoveTo(0, 3),
            Print(instructions)
        )?;

        // Draw active categories
        let active_categories = format!(
            "Active: Type:{:?} Prog:{:?} Lit:{:?}",
            app.quote_db.get_active_category(crate::quotes::CategoryCycle::Typewriter),
            app.quote_db.get_active_category(crate::quotes::CategoryCycle::Programming),
            app.quote_db.get_active_category(crate::quotes::CategoryCycle::Literature),
        );
        queue!(
            self.stdout,
            MoveTo(0, 4),
            SetForegroundColor(Color::Yellow),
            Print(&active_categories),
            ResetColor
        )?;

        // Draw game status
        let game_status = format!(
            "Game: {:?} | Status: {:?}",
            app.game_state.current_game,
            app.game_state.status
        );
        queue!(
            self.stdout,
            MoveTo(0, 5),
            SetForegroundColor(Color::Yellow),
            Print(&game_status),
            ResetColor
        )?;

        // Draw metrics if there's an active session
        if let Some(session) = &app.typing_session {
            let (avg_wpm, avg_accuracy) = app.get_averages().unwrap_or((0.0, 0.0));
            let metrics_text = format!(
                "WPM: {:.1} | Acc: {:.1}% | Avg WPM: {:.1} | Avg Acc: {:.1}%",
                session.metrics.wpm,
                session.metrics.accuracy,
                avg_wpm,
                avg_accuracy
            );
            queue!(
                self.stdout,
                MoveTo(0, 0),
                SetForegroundColor(Color::Green),
                Print(&metrics_text),
                ResetColor
            )?;

            // Draw enhanced keyboard heatmap - use the new function
            heatmap::draw_enhanced_keyboard_heatmap(&mut self.stdout, &session.metrics, 7)?;

            // Draw typing area at a position below the keyboard visualization
            // Adjusted position to account for the larger keyboard display
            let typing_area_y = 40;
            queue!(
                self.stdout,
                MoveTo(0, typing_area_y),
                Print("Type the following text:"),
                MoveTo(0, typing_area_y + 1),
                SetForegroundColor(Color::Blue),
                Print(&session.text),
                ResetColor,
                MoveTo(0, typing_area_y + 2),
                Print(&session.text[..session.current_position]),
                MoveTo(session.current_position as u16, typing_area_y + 2),
                Print("▶")  // Cursor indicator
            )?;
        }

        // Flush all queued changes to the terminal
        self.stdout.flush()?;
        Ok(())
    }
} 