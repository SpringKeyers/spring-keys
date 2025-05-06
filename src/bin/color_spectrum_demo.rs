use spring_keys::ui::color_spectrum::value_to_spectrum;
use crossterm::{
    execute,
    style::{Print, SetBackgroundColor, SetForegroundColor, ResetColor},
    cursor::MoveTo,
    terminal::{self, Clear, ClearType},
};
use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All))?;
    
    // Display title
    execute!(stdout, MoveTo(2, 1))?;
    execute!(stdout, Print("Color Spectrum Demo: Purple (min) -> White (mid) -> Red (max)"))?;
    
    // Display value bar from 0 to 100
    for i in 0..=100 {
        let row = 3 + (i / 20);
        let col = 2 + (i % 20) * 4;
        
        // Get colors for current value
        let colors = value_to_spectrum(i as u8);
        
        // Display value with appropriate colors
        execute!(
            stdout,
            MoveTo(col as u16, row as u16),
            SetBackgroundColor(colors.background),
            SetForegroundColor(colors.foreground),
            Print(format!("{:3}", i)),
            ResetColor
        )?;
    }
    
    // Display color bands for visualization
    let bands = [
        "Minimum (Purple)", 
        "Low-Mid", 
        "Mid (White)", 
        "Mid-High", 
        "Maximum (Red)"
    ];
    
    let values = [0, 20, 50, 80, 100];
    
    for (i, (label, value)) in bands.iter().zip(values.iter()).enumerate() {
        let row = 10 + i;
        
        // Get colors for band
        let colors = value_to_spectrum(*value);
        
        // Display color band
        execute!(stdout, MoveTo(2, row as u16))?;
        execute!(stdout, Print(format!("{}: ", label)))?;
        
        // Display color bar
        execute!(
            stdout, 
            SetBackgroundColor(colors.background),
            SetForegroundColor(colors.foreground)
        )?;
        
        for _ in 0..20 {
            execute!(stdout, Print(" "))?;
        }
        
        execute!(stdout, ResetColor)?;
        execute!(stdout, Print(format!(" Value: {}", value)))?;
    }
    
    // Display instructions
    execute!(stdout, MoveTo(2_u16, 17_u16))?;
    execute!(stdout, Print("Press any key to exit..."))?;
    stdout.flush()?;
    
    // Wait for keypress
    crossterm::event::read()?;
    
    // Cleanup
    terminal::disable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All))?;
    
    Ok(())
} 