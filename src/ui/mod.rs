use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::MoveTo,
    style::{Color as CrosstermColor, SetForegroundColor, ResetColor},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Wrap},
    Terminal,
    Frame,
};
use std::io::{self, Stdout, Write};
use crate::SpringKeys;
use std::time::Duration;

mod heatmap;
pub mod histogram_display;

pub struct TerminalUI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    should_quit: bool,
}

impl TerminalUI {
    pub fn new() -> io::Result<Self> {
        // Set up terminal
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;
        
        Ok(Self {
            terminal,
            should_quit: false,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        io::stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn run(&mut self, app: &mut SpringKeys) -> io::Result<()> {
        // Initialize with a random typing text from the quotes database
        app.start_typing_session(None);
        
        while !self.should_quit {
            let should_quit = &mut self.should_quit;
            
            self.terminal.draw(|f| {
                f.render_widget(tui::widgets::Clear, f.size());
                render_ui(f, app);
            })?;
            
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    // Process exit command (Ctrl+C or Esc)
                    if key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL 
                        || key_event.code == KeyCode::Esc {
                        *should_quit = true;
                        continue;
                    }
                    
                    // Handle F-keys for category cycling
                    match key_event.code {
                        KeyCode::F(5) => {
                            app.start_typing_session(None);
                        },
                        KeyCode::F(6) => {
                            // Cycle Typewriter categories
                            let category = app.quote_db.cycle_category(crate::quotes::CategoryCycle::Typewriter);
                            let quote_text = app.quote_db.next_by_category(category)
                                .map(|quote| quote.text.clone());
                            if let Some(text) = quote_text {
                                app.start_typing_session(Some(text));
                            }
                        },
                        KeyCode::F(7) => {
                            // Cycle Programming categories
                            let category = app.quote_db.cycle_category(crate::quotes::CategoryCycle::Programming);
                            let quote_text = app.quote_db.next_by_category(category)
                                .map(|quote| quote.text.clone());
                            if let Some(text) = quote_text {
                                app.start_typing_session(Some(text));
                            }
                        },
                        KeyCode::F(8) => {
                            // Cycle Literature categories
                            let category = app.quote_db.cycle_category(crate::quotes::CategoryCycle::Literature);
                            let quote_text = app.quote_db.next_by_category(category)
                                .map(|quote| quote.text.clone());
                            if let Some(text) = quote_text {
                                app.start_typing_session(Some(text));
                            }
                        },
                        _ => {
                            // Pass other key events to the application
                            app.process_input(key_event.code, key_event.modifiers);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

// Standalone function to render UI without borrowing self
fn render_ui(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &SpringKeys) {
    // Create the main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Status bar
            Constraint::Length(2),  // Title
            Constraint::Length(1),  // Spacing
            Constraint::Length(2),  // Instructions
            Constraint::Length(1),  // Category status
            Constraint::Length(2),  // Game status
            Constraint::Length(1),  // Spacing
            Constraint::Length(2),  // Metrics
            Constraint::Length(1),  // Error count
            Constraint::Length(1),  // Quote info
            Constraint::Length(15), // Keyboard heatmap
            Constraint::Length(1),  // Spacing
            Constraint::Length(4),  // Row performance
            Constraint::Length(1),  // Spacing
            Constraint::Length(13), // Key Speed Performance
            Constraint::Length(1),  // Spacing
            Constraint::Length(7),  // Finger Performance Chart
            Constraint::Length(1),  // Spacing
            Constraint::Length(1),  // Typing area title
            Constraint::Length(1),  // Source info
            Constraint::Length(1),  // Target text
            Constraint::Length(2),  // User input and cursor arrow
            Constraint::Min(0),     // Remaining space
        ].as_ref())
        .split(frame.size());
    
    // Draw the metrics header (status bar)
    if let Some(_) = &app.typing_session {
        render_metrics_header(frame, app, chunks[0]);
    }
    
    // Draw title
    let title = Paragraph::new("SpringKeys Typing Tutor")
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(title, chunks[1]);
    
    // Draw instructions with category selection options
    let instructions = Paragraph::new(
        "ESC: Quit | F5: Random Quote | F6: Typewriter | F7: Programming | F8: Literature"
    );
    frame.render_widget(instructions, chunks[3]);

    // Draw active categories
    let active_categories = format!(
        "Active Categories: Typewriter: {:?} | Programming: {:?} | Literature: {:?}",
        app.quote_db.get_active_category(crate::quotes::CategoryCycle::Typewriter),
        app.quote_db.get_active_category(crate::quotes::CategoryCycle::Programming),
        app.quote_db.get_active_category(crate::quotes::CategoryCycle::Literature),
    );
    let category_status = Paragraph::new(active_categories)
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(category_status, chunks[4]);
    
    // Draw game status
    let game_status = Paragraph::new(format!(
        "Game: {:?} | Status: {:?}", 
        app.game_state.current_game, 
        app.game_state.status
    )).style(Style::default().fg(Color::Yellow));
    frame.render_widget(game_status, chunks[5]);
    
    // Draw separator
    let separator = Paragraph::new("------------------------------------------------");
    frame.render_widget(separator, chunks[6]);

    // Draw key speed distribution
    if let Some(session) = &app.typing_session {
        let stats = &session.metrics.key_histogram;
        let mut stdout = io::stdout();
        
        // Draw header
        let _ = stdout.execute(MoveTo(chunks[14].x, chunks[14].y));
        let _ = stdout.execute(SetForegroundColor(CrosstermColor::White));
        let _ = stdout.write_all(b"Key Speed Performance:");

        // Helper function to draw a bar
        let draw_bar = |stdout: &mut io::Stdout, x: u16, y: u16, value: f64, max_value: f64, width: u16| {
            let normalized = if max_value > 0.0 { value / max_value } else { 0.0 };
            let bar_width = (normalized * (width as f64)) as u16;
            let _ = stdout.execute(MoveTo(x, y));
            let _ = stdout.write_all(b"[");
            
            // Draw bar with dots and x's
            for i in 0..width {
                let char = if i < bar_width {
                    if i % 3 == 0 { "x" } else { "." }
                } else {
                    " "
                };
                let _ = stdout.write_all(char.as_bytes());
            }
            let _ = stdout.write_all(b"]");
        };

        // Define rows and their data
        let max_value = 500.0; // Maximum expected value for normalization
        let bar_width = 40; // Width of the bar visualization
        let rows = [
            ("Numbers   ", session.metrics.number_metrics.avg_time_ms),
            ("Top      ", session.metrics.top_row_metrics.avg_time_ms),
            ("Home     ", session.metrics.home_row_metrics.avg_time_ms),
            ("Bottom   ", session.metrics.bottom_row_metrics.avg_time_ms),
            ("Quote Min", stats.min_value),
            ("Quote Max", stats.max_value),
            ("Quote 10s", stats.quote_10s_avg),
            ("Quote 60s", stats.quote_60s_avg),
            ("Quote Geo", stats.quote_geo_avg),
            ("Any Last ", stats.arithmetic_mean),
            ("Any 10s  ", stats.running_10s_avg),
            ("Any 60s  ", stats.running_60s_avg),
            ("Any Geo  ", stats.running_geo_avg),
        ];

        // Draw each row
        for (i, (label, value)) in rows.iter().enumerate() {
            let row_y = chunks[14].y + 1 + i as u16;
            
            // Choose color based on value
            let color = if *value < 150.0 {
                CrosstermColor::Green
            } else if *value < 250.0 {
                CrosstermColor::Yellow
            } else {
                CrosstermColor::Red
            };

            // Draw label
            let _ = stdout.execute(MoveTo(chunks[14].x, row_y));
            let _ = stdout.execute(SetForegroundColor(color));
            let _ = stdout.write_all(format!("{} {:5.0}ms ", label, value).as_bytes());

            // Draw bar
            let _ = stdout.execute(SetForegroundColor(color));
            draw_bar(&mut stdout, chunks[14].x + 20, row_y, *value, max_value, bar_width);
            let _ = stdout.execute(ResetColor);
        }
    }

    // Draw finger performance chart
    if let Some(session) = &app.typing_session {
        let finger_metrics = session.metrics.finger_performance();
        let _ = heatmap::KeyboardHeatmap::render_finger_performance(
            &mut io::stdout(),
            chunks[16].x,
            chunks[16].y,
            finger_metrics
        );
    }

    // Draw typing area title
    let typing_title = Paragraph::new("Type the following text:")
        .style(Style::default().fg(Color::Green));
    frame.render_widget(typing_title, chunks[18]);
    
    // Draw target text and other components if we have a typing session
    if let Some(session) = &app.typing_session {
        // Get current quote source
        let current_quote = app.quote_db.current();
        
        // Get and render keyboard heatmap first (it's larger)
        let heat_map = session.metrics.generate_heat_map();
        let _ = heatmap::KeyboardHeatmap::render(&heat_map, &mut io::stdout(), chunks[10].x, chunks[10].y);

        // Get row performance data
        let number_metrics = &session.metrics.number_metrics;
        let top_row_metrics = &session.metrics.top_row_metrics;
        let home_row_metrics = &session.metrics.home_row_metrics;
        let bottom_row_metrics = &session.metrics.bottom_row_metrics;
        
        // Render row performance after heatmap
        let _ = heatmap::KeyboardHeatmap::render_row_performance(
            &mut io::stdout(),
            chunks[12].x,
            chunks[12].y,
            chunks[12].width,
            number_metrics.avg_time_ms,
            top_row_metrics.avg_time_ms,
            home_row_metrics.avg_time_ms,
            bottom_row_metrics.avg_time_ms
        );

        // Draw text source
        let source = Paragraph::new(format!("Source: {}", current_quote.source))
            .style(Style::default().fg(Color::Blue));
        frame.render_widget(source, chunks[19]);
        
        // Draw target text
        let target_text = Paragraph::new(session.text.clone())
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));
        frame.render_widget(target_text, chunks[20]);
        
        // Draw metrics
        let metrics = Paragraph::new(format!(
            "WPM: {:.1} | Accuracy: {:.1}%", 
            session.metrics.wpm, 
            session.metrics.accuracy
        ));
        frame.render_widget(metrics, chunks[7]);
        
        // Draw error count
        let errors = Paragraph::new(format!("Errors: {}", session.metrics.errors.len()));
        frame.render_widget(errors, chunks[8]);
        
        // Draw quote difficulty
        let quote_info = Paragraph::new(format!(
            "Difficulty: {:?} | Category: {:?} | Origin: {}", 
            current_quote.difficulty, 
            current_quote.category,
            current_quote.origin
        ));
        frame.render_widget(quote_info, chunks[9]);
    }

    // Draw input text with character-by-character comparison
    let input_text = &app.input_processor.current_text;
    let mut styled_input = vec![];
    
    // Create input text without label, ensure we have enough space for the full text
    let mut current_width = 0;
    let typing_session = app.typing_session.as_ref().unwrap();
    
    for (i, c) in input_text.chars().enumerate() {
        let target_char = typing_session.text.chars().nth(i);
        
        let style = if let Some(target) = target_char {
            if c == target {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            }
        } else {
            Style::default().fg(Color::Yellow)
        };
        
        styled_input.push(Span::styled(c.to_string(), style));
        current_width += 1;
    }

    // Ensure we have at least as much width as the target text
    let target_width = typing_session.text.chars().count();
    if current_width < target_width {
        styled_input.push(Span::raw(" ".repeat(target_width - current_width)));
    }
    
    // Create the input line and cursor arrow line in a more compact way
    let input_spans = Spans::from(styled_input);
    let mut cursor_line = vec![Span::raw(" ".repeat(app.input_processor.cursor_position))];
    cursor_line.push(Span::styled("↑", Style::default().fg(Color::Cyan)));
    
    // Ensure cursor line is at least as wide as the input text
    if app.input_processor.cursor_position < target_width {
        cursor_line.push(Span::raw(" ".repeat(target_width - app.input_processor.cursor_position - 1)));
    }
    
    let cursor_spans = Spans::from(cursor_line);
    
    // Combine both lines into a paragraph with explicit width
    let input_paragraph = Paragraph::new(vec![input_spans, cursor_spans])
        .wrap(Wrap { trim: true });
    frame.render_widget(input_paragraph, chunks[19]);
    
    // Set cursor position
    let cursor_pos = app.input_processor.cursor_position;
    frame.set_cursor(
        chunks[19].x + cursor_pos as u16,
        chunks[19].y
    );
}

fn render_metrics_header(frame: &mut tui::Frame<CrosstermBackend<Stdout>>, app: &SpringKeys, area: Rect) {
    if let Some(session) = &app.typing_session {
        let wpm = session.metrics.wpm;
        let accuracy = session.metrics.accuracy;
        let (avg_wpm, avg_accuracy) = session.get_averages();
        
        let metrics_text = format!(
            "WPM: {:.1} | Accuracy: {:.1}% | Avg WPM: {:.1} | Avg Accuracy: {:.1}% | Completed Quotes: {}",
            wpm,
            accuracy,
            avg_wpm,
            avg_accuracy,
            session.completed_quotes
        );
        
        let style = match () {
            _ if wpm >= 60.0 => Style::default().fg(Color::Green),
            _ if wpm >= 40.0 => Style::default().fg(Color::Yellow),
            _ => Style::default().fg(Color::Red),
        };
        
        let metrics = Paragraph::new(metrics_text).style(style);
        frame.render_widget(metrics, area);
    }
} 