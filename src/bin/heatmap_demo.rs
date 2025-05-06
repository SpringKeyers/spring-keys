use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    event::{self, Event, KeyCode},
    cursor::{Hide, Show},
    queue, ExecutableCommand,
};
use std::io::{self, stdout, Write};
use spring_keys::TypingMetrics;
use spring_keys::ui::heatmap;
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    
    // Create demo metrics
    let mut metrics = TypingMetrics::new();
    let start_time = Instant::now();
    
    // Main demo loop
    loop {
        // Clear screen
        queue!(
            stdout,
            Clear(ClearType::All)
        )?;
        
        // Update demo data based on time
        let elapsed = start_time.elapsed().as_secs_f64();
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            let speed = 150.0 + (elapsed * 10.0).sin() * 50.0;
            metrics.record_keystroke(c, c, speed as usize);
        }
        
        // Draw heatmap
        heatmap::draw_enhanced_keyboard_heatmap(&mut stdout, &metrics, 2)?;
        stdout.flush()?;
        
        // Check for quit
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
} 