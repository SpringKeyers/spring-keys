use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::{Hide, Show},
    ExecutableCommand,
};
use spring_keys::{TypingMetrics, ui::heatmap};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    // Set up terminal
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(Hide)?;
    stdout.execute(Clear(ClearType::All))?;

    // Create demo metrics
    let mut metrics = TypingMetrics::new();
    
    // Simulate typing data for demo
    metrics.simulate_demo_data();

    // Draw the unified keyboard heatmap
    heatmap::draw_unified_keyboard_heatmap(&mut stdout, &metrics, 2)?;

    // Wait for user input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Clean up terminal
    disable_raw_mode()?;
    stdout.execute(Show)?;
    stdout.execute(Clear(ClearType::All))?;

    Ok(())
} 