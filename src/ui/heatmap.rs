use crossterm::{
    style::{Color, Print, SetForegroundColor, ResetColor, SetBackgroundColor},
    cursor::MoveTo,
    queue,
};
use std::io::{self, Write};
use crate::{TypingMetrics, Finger};
use crate::ui::color_spectrum::value_to_spectrum;

/// Enhanced keyboard visualization matching screenshot design
pub fn draw_keyboard_heatmap(
    stdout: &mut impl Write,
    metrics: &TypingMetrics,
    y_offset: u16
) -> io::Result<()> {
    // Get typing statistics
    let heat_map = metrics.generate_heat_map();
    let geometric_avgs = metrics.get_key_geometric_averages();
    
    // Draw difficulty and category info
    queue!(
        stdout,
        MoveTo(0, y_offset),
        SetForegroundColor(Color::White),
        Print("Errors: 0"),
        MoveTo(0, y_offset + 1),
        Print("Difficulty: Easy | Category: Proverbs | Origin: English"),
        ResetColor
    )?;
    
    // Keyboard layout rows
    let rows = [
        "1234567890-=",
        "qwertyuiop[]\\",
        "asdfghjkl;'",
        "zxcvbnm,./",
    ];

    // Space between keys
    let key_spacing: u16 = 1;
    
    // Key dimensions
    let key_width: u16 = 7;
    let key_height: u16 = 2;
    
    // Draw keyboard layout
    for (row_idx, row) in rows.iter().enumerate() {
        // Calculate row indentation based on standard keyboard layout
        let indent: u16 = match row_idx {
            0 => 0, // Number row
            1 => 2, // Top QWERTY row
            2 => 4, // Home row
            3 => 6, // Bottom row
            _ => 0,
        };
        
        let base_y = y_offset + 3 + (row_idx as u16 * (key_height + key_spacing));
        
        for (col_idx, key) in row.chars().enumerate() {
            let base_x = indent + (col_idx as u16 * (key_width + key_spacing));
            
            // Get metrics for this key
            let (recent_speed, _) = heat_map.get(&key).unwrap_or(&(0.0, 0));
            let geo_avg = geometric_avgs.get(&key).unwrap_or(&0.0);
            
            // Format speeds for display
            let speed_text = if *recent_speed > 0.0 {
                format!("{}ms", recent_speed.round() as u32)
            } else {
                String::from("---")
            };
            
            // Determine colors
            let bg_color = get_background_color(*geo_avg);
            let fg_color = get_foreground_color(*recent_speed);
            
            // Draw key box
            queue!(
                stdout,
                MoveTo(base_x, base_y),
                SetBackgroundColor(bg_color),
                SetForegroundColor(fg_color),
                Print(format!("{:^width$}", key, width = key_width as usize)),
                MoveTo(base_x, base_y + 1),
                Print(format!("{:^width$}", speed_text, width = key_width as usize)),
                ResetColor
            )?;
        }
    }
    
    // Draw space bar
    let space_y = y_offset + 3 + (4 * (key_height + key_spacing));
    let space_x: u16 = 7; // Center under the keyboard
    let space_width: u16 = 7 * 7; // Cover about 7 key widths
    
    let (space_speed, _) = heat_map.get(&' ').unwrap_or(&(0.0, 0));
    let space_geo_avg = geometric_avgs.get(&' ').unwrap_or(&0.0);
    
    let space_bg_color = get_background_color(*space_geo_avg);
    let space_fg_color = get_foreground_color(*space_speed);
    
    let space_speed_text = if *space_speed > 0.0 {
        format!("{}ms", space_speed.round() as u32)
    } else {
        String::from("---")
    };
    
    queue!(
        stdout,
        MoveTo(space_x, space_y),
        SetBackgroundColor(space_bg_color),
        SetForegroundColor(space_fg_color),
        Print(format!("{:^width$}", "", width = space_width as usize)),
        MoveTo(space_x, space_y + 1),
        Print(format!("{:^width$}", space_speed_text, width = space_width as usize)),
        ResetColor
    )?;
    
    Ok(())
}

/// Get background color based on geometric average speed
fn get_background_color(speed_ms: f64) -> Color {
    if speed_ms <= 0.0 {
        return Color::Rgb { r: 40, g: 40, b: 40 }; // Dark gray for unused keys
    } else if speed_ms < 130.0 {
        Color::Rgb { r: 0, g: 60, b: 160 } // Dark blue for fast typing
    } else if speed_ms < 160.0 {
        Color::Rgb { r: 0, g: 100, b: 0 } // Dark green for good typing
    } else if speed_ms < 250.0 {
        Color::Rgb { r: 140, g: 140, b: 0 } // Dark yellow for medium typing
    } else {
        Color::Rgb { r: 140, g: 0, b: 0 } // Dark red for slow typing
    }
}

/// Get foreground (text) color based on recent speed
fn get_foreground_color(speed_ms: f64) -> Color {
    if speed_ms <= 0.0 {
        return Color::White; // White for unused keys
    } else if speed_ms < 130.0 {
        Color::Rgb { r: 150, g: 200, b: 255 } // Light blue for fast typing
    } else if speed_ms < 160.0 {
        Color::Rgb { r: 150, g: 255, b: 150 } // Light green for good typing
    } else if speed_ms < 250.0 {
        Color::Rgb { r: 255, g: 255, b: 150 } // Light yellow for medium typing
    } else {
        Color::Rgb { r: 255, g: 150, b: 150 } // Light red for slow typing
    }
}

/// Enhanced keyboard visualization with larger keys, showing current and geometric average speeds
pub fn draw_enhanced_keyboard_heatmap(
    stdout: &mut impl Write,
    metrics: &TypingMetrics,
    y_offset: u16
) -> io::Result<()> {
    let heat_map = metrics.generate_heat_map();
    let geometric_avgs = metrics.get_key_geometric_averages();
    
    // Draw header with typing info
    queue!(
        stdout,
        MoveTo(0, y_offset),
        SetForegroundColor(Color::White),
        Print("Errors: 0"),
        MoveTo(0, y_offset + 1),
        Print("Difficulty: Easy | Category: Proverbs | Origin: English"),
        ResetColor
    )?;
    
    // Keyboard layout rows
    let rows = [
        "1234567890-=",
        "qwertyuiop[]\\",
        "asdfghjkl;'",
        "zxcvbnm,./",
    ];

    // Left edge indentations for each row
    let indents = [0u16, 2u16, 4u16, 6u16];
    
    // Key dimensions
    let key_width: u16 = 8;
    let key_height: u16 = 4;
    let key_spacing: u16 = 1;
    
    // Draw keyboard
    for (row_idx, (row, indent)) in rows.iter().zip(indents.iter()).enumerate() {
        let base_y = y_offset + 3 + (row_idx as u16 * (key_height + key_spacing));
        
        for (col_idx, key) in row.chars().enumerate() {
            let base_x = indent + (col_idx as u16 * (key_width + key_spacing));
            
            // Get speeds for this key
            let (recent_speed, _) = heat_map.get(&key).unwrap_or(&(0.0, 0));
            let geo_avg = geometric_avgs.get(&key).unwrap_or(&0.0);
            
            draw_big_key(stdout, base_x, base_y, key, *recent_speed, *geo_avg)?;
        }
    }
    
    // Draw space bar
    let space_base_y = y_offset + 3 + (4 * (key_height + key_spacing));
    let space_base_x: u16 = 12;
    
    let (space_speed, _) = heat_map.get(&' ').unwrap_or(&(0.0, 0));
    let space_geo_avg = geometric_avgs.get(&' ').unwrap_or(&0.0);
    
    draw_spacebar(stdout, space_base_x, space_base_y, *space_speed, *space_geo_avg)?;
    
    Ok(())
}

/// Draw a key showing both current and geometric average speeds
fn draw_big_key(
    stdout: &mut impl Write,
    x: u16,
    y: u16,
    key: char,
    recent_speed: f64,
    geo_avg: f64
) -> io::Result<()> {
    // Determine colors based on speeds
    let top_bg_color = get_background_color(geo_avg);
    let top_fg_color = get_foreground_color(recent_speed);
    
    // Format speeds for display
    let recent_speed_str = if recent_speed > 0.0 {
        format!("{}ms", recent_speed.round() as u32)
    } else {
        "---".to_string()
    };
    
    let geo_avg_str = if geo_avg > 0.0 {
        format!("{}ms", geo_avg.round() as u32)
    } else {
        "---".to_string()
    };
    
    // Map speed to a 0-100 scale for the color spectrum
    // 100ms = 100 (fastest), 300ms+ = 0 (slowest)
    let spectrum_value = if recent_speed <= 0.0 {
        0 // No data
    } else if recent_speed <= 100.0 {
        100 // Maximum (fastest)
    } else if recent_speed >= 300.0 {
        0 // Minimum (slowest)
    } else {
        // Linear mapping from 100-300ms to 100-0
        ((300.0 - recent_speed) / 2.0) as u8
    };
    
    // Get color from spectrum for second row
    let spectrum_colors = value_to_spectrum(spectrum_value);
    
    // Draw key box with spectrum colors for second row
    queue!(
        stdout,
        MoveTo(x, y),
        SetBackgroundColor(top_bg_color),
        SetForegroundColor(top_fg_color),
        Print(format!("{:^8}", key)),
        MoveTo(x, y + 1),
        SetBackgroundColor(spectrum_colors.background),
        SetForegroundColor(spectrum_colors.foreground),
        Print(format!("{:^8}", recent_speed_str)),
        MoveTo(x, y + 2),
        SetBackgroundColor(top_bg_color),
        SetForegroundColor(top_fg_color),
        Print(format!("{:^8}", geo_avg_str)),
        MoveTo(x, y + 3),
        Print("        "),
        ResetColor
    )?;
    
    Ok(())
}

/// Draw space bar with current and geometric average speeds
fn draw_spacebar(
    stdout: &mut impl Write,
    x: u16,
    y: u16,
    recent_speed: f64,
    geo_avg: f64
) -> io::Result<()> {
    // Determine colors based on speeds
    let top_bg_color = get_background_color(geo_avg);
    let top_fg_color = get_foreground_color(recent_speed);
    
    // Format speeds for display
    let recent_speed_str = if recent_speed > 0.0 {
        format!("{}ms", recent_speed.round() as u32)
    } else {
        "---".to_string()
    };
    
    let geo_avg_str = if geo_avg > 0.0 {
        format!("{}ms", geo_avg.round() as u32)
    } else {
        "---".to_string()
    };
    
    // Map speed to a 0-100 scale for the color spectrum
    // 100ms = 100 (fastest), 300ms+ = 0 (slowest)
    let spectrum_value = if recent_speed <= 0.0 {
        0 // No data
    } else if recent_speed <= 100.0 {
        100 // Maximum (fastest)
    } else if recent_speed >= 300.0 {
        0 // Minimum (slowest)
    } else {
        // Linear mapping from 100-300ms to 100-0
        ((300.0 - recent_speed) / 2.0) as u8
    };
    
    // Get color from spectrum for second row
    let spectrum_colors = value_to_spectrum(spectrum_value);
    
    // Draw space bar with spectrum colors for second row 
    queue!(
        stdout,
        MoveTo(x, y),
        SetBackgroundColor(top_bg_color),
        SetForegroundColor(top_fg_color),
        Print("             SPACE              "),
        MoveTo(x, y + 1),
        SetBackgroundColor(spectrum_colors.background),
        SetForegroundColor(spectrum_colors.foreground),
        Print(format!("{:^30}", recent_speed_str)),
        MoveTo(x, y + 2),
        SetBackgroundColor(top_bg_color),
        SetForegroundColor(top_fg_color),
        Print(format!("{:^30}", geo_avg_str)),
        MoveTo(x, y + 3),
        Print("                              "),
        ResetColor
    )?;
    
    Ok(())
}

pub fn draw_finger_performance(
    stdout: &mut impl Write,
    y_offset: u16,
    metrics: &TypingMetrics
) -> io::Result<()> {
    // Draw header
    queue!(
        stdout,
        MoveTo(0, y_offset),
        SetForegroundColor(Color::White),
        Print("Finger Performance (ms):"),
        ResetColor
    )?;

    // Get finger performance stats
    let finger_stats = metrics.finger_performance();
    
    // Define fingers in order from left to right (excluding thumb)
    let fingers = [
        ("LP", Finger::LeftPinky),
        ("LR", Finger::LeftRing),
        ("LM", Finger::LeftMiddle),
        ("LI", Finger::LeftIndex),
        ("RI", Finger::RightIndex),
        ("RM", Finger::RightMiddle),
        ("RR", Finger::RightRing),
        ("RP", Finger::RightPinky),
    ];

    // Draw finger metrics
    for (i, (label, finger)) in fingers.iter().enumerate() {
        let x = i as u16 * 10;
        let y = y_offset + 1;

        if let Some(stats) = finger_stats.get(finger) {
            let color = if stats.current < 150.0 {
                Color::Green
            } else if stats.current < 250.0 {
                Color::Yellow
            } else {
                Color::Red
            };

            queue!(
                stdout,
                MoveTo(x, y),
                Print(label),
                MoveTo(x, y + 1),
                SetForegroundColor(color),
                Print(format!("{:3.0}", stats.current)),
                ResetColor
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_color_mapping() {
        // Test background colors
        assert_ne!(get_background_color(0.0), get_background_color(120.0));
        assert_ne!(get_background_color(120.0), get_background_color(180.0));
        assert_ne!(get_background_color(180.0), get_background_color(300.0));
        
        // Test foreground colors
        assert_ne!(get_foreground_color(0.0), get_foreground_color(120.0));
        assert_ne!(get_foreground_color(120.0), get_foreground_color(180.0));
        assert_ne!(get_foreground_color(180.0), get_foreground_color(300.0));
    }
} 