use std::io::{self, Stdout};
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use crate::SpringKeys;
use crate::core::metrics::Finger;

mod heatmap;

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
            // Render the interface - create a closure that doesn't capture self
            let should_quit = &mut self.should_quit; // Create a mutable reference to should_quit
            
            self.terminal.draw(|f| {
                // Use functions that don't require self
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
}

// Standalone function to render UI without borrowing self
fn render_ui(frame: &mut tui::Frame<CrosstermBackend<Stdout>>, app: &SpringKeys) {
    // Create the main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Status bar
            Constraint::Length(2),  // Title
            Constraint::Length(1),  // Spacing
            Constraint::Length(2),  // Instructions
            Constraint::Length(1),  // Spacing
            Constraint::Length(2),  // Game status
            Constraint::Length(1),  // Separator
            Constraint::Length(2),  // Typing area title
            Constraint::Length(2),  // Source info
            Constraint::Length(1),  // Spacing
            Constraint::Length(2),  // Target text
            Constraint::Length(1),  // Spacing
            Constraint::Length(2),  // Input label
            Constraint::Length(2),  // User input
            Constraint::Length(1),  // Spacing
            Constraint::Length(2),  // Metrics
            Constraint::Length(2),  // Error count
            Constraint::Length(2),  // Quote info
            Constraint::Min(0),     // Rest of screen (for detailed metrics)
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
    
    // Draw instructions
    let instructions = Paragraph::new("Press ESC to quit, F5 for new quote, arrow keys to move cursor");
    frame.render_widget(instructions, chunks[3]);
    
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
    
    // Draw typing area title
    let typing_title = Paragraph::new("Type the following text:")
        .style(Style::default().fg(Color::Green));
    frame.render_widget(typing_title, chunks[7]);
    
    // Draw target text and other components if we have a typing session
    if let Some(session) = &app.typing_session {
        // Get current quote source
        let current_quote = app.quote_db.current();
        
        // Draw text source
        let source = Paragraph::new(format!("Source: {}", current_quote.source))
            .style(Style::default().fg(Color::Blue));
        frame.render_widget(source, chunks[8]);
        
        // Draw target text
        let target_text = Paragraph::new(session.text.clone())
            .wrap(Wrap { trim: false });
        frame.render_widget(target_text, chunks[10]);
        
        // Draw user input area label
        let input_label = Paragraph::new("Your input:");
        frame.render_widget(input_label, chunks[12]);
        
        // Draw input text with character-by-character comparison
        let input_text = &app.input_processor.current_text;
        let mut styled_input = vec![];
        
        for (i, c) in input_text.chars().enumerate() {
            let target_char = session.text.chars().nth(i);
            
            let style = if let Some(target) = target_char {
                if c == target {
                    // Correct character - green
                    Style::default().fg(Color::Green)
                } else {
                    // Incorrect character - red
                    Style::default().fg(Color::Red)
                }
            } else {
                // Extra character - yellow
                Style::default().fg(Color::Yellow)
            };
            
            styled_input.push(Span::styled(c.to_string(), style));
        }
        
        // Add cursor
        let cursor_pos = app.input_processor.cursor_position;
        if cursor_pos <= input_text.len() {
            // Render input with cursor
            let input_spans = Spans::from(styled_input);
            let input_paragraph = Paragraph::new(vec![input_spans]);
            frame.render_widget(input_paragraph, chunks[13]);
            
            // Position cursor at the right place
            // Note: tui-rs doesn't directly support cursor positioning
            // but we set the cursor position via the terminal
            frame.set_cursor(
                chunks[13].x + cursor_pos as u16,
                chunks[13].y
            );
        }
        
        // Draw metrics
        let metrics = Paragraph::new(format!(
            "WPM: {:.1} | Accuracy: {:.1}%", 
            session.metrics.wpm, 
            session.metrics.accuracy
        ));
        frame.render_widget(metrics, chunks[15]);
        
        // Draw error count
        let errors = Paragraph::new(format!("Errors: {}", session.metrics.errors.len()));
        frame.render_widget(errors, chunks[16]);
        
        // Draw quote difficulty
        let quote_info = Paragraph::new(format!(
            "Difficulty: {:?} | Category: {:?} | Origin: {}", 
            current_quote.difficulty, 
            current_quote.category,
            current_quote.origin
        ));
        frame.render_widget(quote_info, chunks[17]);
        
        // For detailed metrics, we need extra space
        if chunks[18].height > 10 {
            render_detailed_metrics(frame, app, chunks[18]);
        }
    }
}

fn render_metrics_header(frame: &mut tui::Frame<CrosstermBackend<Stdout>>, app: &SpringKeys, area: Rect) {
    if let Some(session) = &app.typing_session {
        // Calculate short-term averages to display
        let letter_avg = session.metrics.letter_metrics.short_term_avg_ms;
        let number_avg = session.metrics.number_metrics.short_term_avg_ms;
        let homerow_avg = session.metrics.home_row_metrics.short_term_avg_ms;
        let toprow_avg = session.metrics.top_row_metrics.short_term_avg_ms;
        let bottomrow_avg = session.metrics.bottom_row_metrics.short_term_avg_ms;
        
        // Create colored spans for each metric
        let spans = vec![
            Span::raw("Speed (ms): "),
            Span::raw("Let:"),
            colorize_metric(letter_avg),
            Span::raw(" Num:"),
            colorize_metric(number_avg),
            Span::raw(" Home:"),
            colorize_metric(homerow_avg),
            Span::raw(" Top:"),
            colorize_metric(toprow_avg),
            Span::raw(" Bot:"),
            colorize_metric(bottomrow_avg),
            Span::raw(" WPM:"),
            Span::styled(
                format!("{:.1}", session.metrics.wpm),
                Style::default().fg(Color::White)
            ),
            Span::raw(" Acc:"),
            Span::styled(
                format!("{:.1}%", session.metrics.accuracy),
                Style::default().fg(Color::White)
            ),
        ];
        
        let header = Paragraph::new(Spans::from(spans))
            .style(Style::default().bg(Color::Blue)); // Use standard Blue instead of DarkBlue
        
        frame.render_widget(header, area);
    }
}

fn colorize_metric(value: f64) -> Span<'static> {
    let color = if value <= 150.0 {
        Color::Green // Fast
    } else if value <= 250.0 {
        Color::Yellow // Medium
    } else {
        Color::Red // Slow
    };
    
    Span::styled(format!("{:.0}", value), Style::default().fg(color))
}

fn render_detailed_metrics(frame: &mut tui::Frame<CrosstermBackend<Stdout>>, app: &SpringKeys, area: Rect) {
    if let Some(session) = &app.typing_session {
        // Split the area into sections for different metric displays
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),  // Finger performance
                Constraint::Length(5),  // Row performance
                Constraint::Min(10),    // Space for heatmap
            ].as_ref())
            .split(area);
        
        // Create finger performance block
        let finger_block = Block::default()
            .title("Finger Performance (ms)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        
        // Create row performance block
        let row_block = Block::default()
            .title("Row Performance (ms)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        
        // Create heatmap block
        let heatmap_block = Block::default()
            .title("Keyboard Speed Heatmap (blue=fast, red=slow)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        
        frame.render_widget(finger_block.clone(), chunks[0]);
        frame.render_widget(row_block.clone(), chunks[1]);
        frame.render_widget(heatmap_block.clone(), chunks[2]);
        
        // For more detailed visualizations of finger/row performance and heatmap,
        // we'd need to create custom widgets with tui-rs
        // This is a simplified implementation to get started
        
        // For finger performance data, we can render a simpler text representation
        if let Some(finger_perf) = app.get_finger_performance() {
            let mut left_hand = vec![];
            let mut right_hand = vec![];
            
            // Left hand fingers
            if let Some(left_pinky) = finger_perf.get(&Finger::LeftPinky) {
                left_hand.push(format!("Pinky: {:.1}ms", left_pinky));
            }
            if let Some(left_ring) = finger_perf.get(&Finger::LeftRing) {
                left_hand.push(format!("Ring: {:.1}ms", left_ring));
            }
            if let Some(left_middle) = finger_perf.get(&Finger::LeftMiddle) {
                left_hand.push(format!("Middle: {:.1}ms", left_middle));
            }
            if let Some(left_index) = finger_perf.get(&Finger::LeftIndex) {
                left_hand.push(format!("Index: {:.1}ms", left_index));
            }
            
            // Right hand fingers
            if let Some(right_index) = finger_perf.get(&Finger::RightIndex) {
                right_hand.push(format!("Index: {:.1}ms", right_index));
            }
            if let Some(right_middle) = finger_perf.get(&Finger::RightMiddle) {
                right_hand.push(format!("Middle: {:.1}ms", right_middle));
            }
            if let Some(right_ring) = finger_perf.get(&Finger::RightRing) {
                right_hand.push(format!("Ring: {:.1}ms", right_ring));
            }
            if let Some(right_pinky) = finger_perf.get(&Finger::RightPinky) {
                right_hand.push(format!("Pinky: {:.1}ms", right_pinky));
            }
            
            // Get inner area of the block
            let finger_inner = finger_block.inner(CustomMargin { vertical: 0, horizontal: 0 });
            let finger_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(finger_inner);
            
            // Render left hand data
            let left_hand_para = Paragraph::new(vec![
                Spans::from(vec![Span::styled("Left Hand", Style::default().fg(Color::White))]),
                Spans::from(left_hand.join(" | ")),
            ]);
            
            // Render right hand data
            let right_hand_para = Paragraph::new(vec![
                Spans::from(vec![Span::styled("Right Hand", Style::default().fg(Color::White))]),
                Spans::from(right_hand.join(" | ")),
            ]);
            
            frame.render_widget(left_hand_para, finger_chunks[0]);
            frame.render_widget(right_hand_para, finger_chunks[1]);
        }
        
        // Render row performance data
        let row_inner = row_block.inner(CustomMargin { vertical: 0, horizontal: 0 });
        let row_para = Paragraph::new(vec![
            Spans::from(format!("Top Row: {:.1}ms", session.metrics.top_row_metrics.avg_time_ms)),
            Spans::from(format!("Home Row: {:.1}ms", session.metrics.home_row_metrics.avg_time_ms)),
            Spans::from(format!("Bottom Row: {:.1}ms", session.metrics.bottom_row_metrics.avg_time_ms)),
        ]);
        
        frame.render_widget(row_para, row_inner);
        
        // For heatmap, we would need to create a custom widget
        // This would require significant additional code
    }
}

struct CustomMargin {
    vertical: u16,
    horizontal: u16,
}

trait RectExt {
    fn inner(&self, margin: &CustomMargin) -> Rect;
}

impl RectExt for Rect {
    fn inner(&self, margin: &CustomMargin) -> Rect {
        Rect::new(
            self.x + margin.horizontal,
            self.y + margin.vertical,
            self.width.saturating_sub(margin.horizontal * 2),
            self.height.saturating_sub(margin.vertical * 2)
        )
    }
} 