use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    style::{Color, Print, SetForegroundColor, SetBackgroundColor, ResetColor},
    cursor::{MoveTo, Hide, Show},
    queue, ExecutableCommand,
};
use std::io::{self, Write, stdout};
use std::time::Duration;
use std::thread;
use spring_keys::ui::color_spectrum::value_to_spectrum;

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    stdout.execute(Hide)?;
    
    // Draw color spectrum
    for i in 0..=100 {
        let color = value_to_spectrum(i as f64);
        
        // Draw color bar
        queue!(
            stdout,
            MoveTo(0, i as u16),
            SetBackgroundColor(color),
            SetForegroundColor(Color::White),
            Print(format!(" Value: {:3} ", i)),
            ResetColor
        )?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(50));
    }
    
    // Wait for a moment before clearing
    thread::sleep(Duration::from_secs(2));
    
    // Draw color blocks
    let test_values = [0, 25, 50, 75, 100];
    for (i, value) in test_values.iter().enumerate() {
        let color = value_to_spectrum(*value as f64);
        
        // Draw color block
        queue!(
            stdout,
            MoveTo(0, i as u16 * 3),
            SetBackgroundColor(color),
            SetForegroundColor(Color::White),
            Print(format!(" Test value: {:3} ", value)),
            ResetColor
        )?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(500));
    }
    
    // Wait for a moment before exiting
    thread::sleep(Duration::from_secs(2));
    
    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
} 