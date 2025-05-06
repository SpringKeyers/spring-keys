use crossterm::{
    style::{Color, Print, SetForegroundColor, SetBackgroundColor, ResetColor},
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

/// Update the spectrum value calculation and color usage
fn get_speed_color(speed_ms: f64) -> Color {
    // Convert speed to a value between 0 and 100
    let spectrum_value = if speed_ms < 100.0 {
        0.0 // Fastest - purple
    } else if speed_ms > 400.0 {
        100.0 // Slowest - red
    } else {
        // Linear interpolation between 100ms and 400ms
        ((speed_ms - 100.0) / 300.0) * 100.0
    };
    
    value_to_spectrum(spectrum_value)
}

/// Enhanced keyboard visualization with larger keys, showing current and geometric average speeds
pub fn draw_enhanced_keyboard_heatmap(
    stdout: &mut impl Write,
    metrics: &TypingMetrics,
    y_offset: u16
) -> io::Result<()> {
    let heat_map = metrics.generate_heat_map();
    let geometric_avgs = metrics.get_key_geometric_averages();
    
    // Define keyboard layout
    let rows = [
        "1234567890-=",
        "qwertyuiop[]\\",
        "asdfghjkl;'",
        "zxcvbnm,./",
    ];
    
    let indents = [0, 0, 1, 2]; // Number of spaces to indent each row
    let key_width = 8;
    let key_height = 3;
    let key_spacing = 1;
    
    // Draw each row of the keyboard
    for (row_idx, (row, indent)) in rows.iter().zip(indents.iter()).enumerate() {
        let base_y = y_offset + (row_idx as u16 * (key_height + key_spacing));
        let indent_spaces = " ".repeat(*indent);
        
        // Draw each key in the row
        for (i, key) in row.chars().enumerate() {
            let x = (indent_spaces.len() + i * (key_width + 1)) as u16;
            
            // Get speed data for the key
            let (recent_speed, hits) = heat_map.get(&key).copied().unwrap_or((0.0, 0));
            let geo_avg = geometric_avgs.get(&key).copied().unwrap_or(0.0);
            
            // Format speed strings
            let recent_speed_str = if hits > 0 {
                format!("{:.0}ms", recent_speed)
            } else {
                String::from("-")
            };
            
            let geo_avg_str = if geo_avg > 0.0 {
                format!("{:.0}ms", geo_avg)
            } else {
                String::from("-")
            };
            
            // Get color based on speed
            let bg_color = get_speed_color(geo_avg);
            let fg_color = Color::White;
            
            // Draw key box
            queue!(
                stdout,
                MoveTo(x, base_y),
                SetBackgroundColor(bg_color),
                SetForegroundColor(fg_color),
                Print(format!("{:^8}", key)),
                MoveTo(x, base_y + 1),
                Print(format!("{:^8}", recent_speed_str)),
                MoveTo(x, base_y + 2),
                Print(format!("{:^8}", geo_avg_str)),
                ResetColor
            )?;
        }
    }
    
    // Draw space bar
    let space_y = y_offset + (4 * (key_height + key_spacing));
    let space_x = 12;
    
    // Get space bar metrics
    let (recent_speed, hits) = heat_map.get(&' ').copied().unwrap_or((0.0, 0));
    let geo_avg = geometric_avgs.get(&' ').copied().unwrap_or(0.0);
    
    // Format speed strings
    let recent_speed_str = if hits > 0 {
        format!("{:.0}ms", recent_speed)
    } else {
        String::from("-")
    };
    
    let geo_avg_str = if geo_avg > 0.0 {
        format!("{:.0}ms", geo_avg)
    } else {
        String::from("-")
    };
    
    // Get color based on speed
    let bg_color = get_speed_color(geo_avg);
    let fg_color = Color::White;
    
    // Draw space bar
    queue!(
        stdout,
        MoveTo(space_x, space_y),
        SetBackgroundColor(bg_color),
        SetForegroundColor(fg_color),
        Print("             SPACE              "),
        MoveTo(space_x, space_y + 1),
        Print(format!("{:^30}", recent_speed_str)),
        MoveTo(space_x, space_y + 2),
        Print(format!("{:^30}", geo_avg_str)),
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
    use crate::ui::color_spectrum;
    
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
    
    #[test]
    fn test_key_1_color_spectrum() {
        // Test color spectrum for key '1'
        
        // Case 1: No data (0 ms)
        let no_data_speed = 0.0;
        let no_data_spectrum_value = if no_data_speed <= 0.0 {
            0
        } else if no_data_speed <= 100.0 {
            100
        } else if no_data_speed >= 300.0 {
            0
        } else {
            ((300.0 - no_data_speed) / 2.0) as u8
        };
        let no_data_colors = color_spectrum::value_to_spectrum(no_data_spectrum_value);
        
        // Case 2: Fast typing (90 ms)
        let fast_speed = 90.0;
        let fast_spectrum_value = if fast_speed <= 0.0 {
            0
        } else if fast_speed <= 100.0 {
            100
        } else if fast_speed >= 300.0 {
            0
        } else {
            ((300.0 - fast_speed) / 2.0) as u8
        };
        let fast_colors = color_spectrum::value_to_spectrum(fast_spectrum_value);
        
        // Case 3: Medium typing (200 ms)
        let medium_speed = 200.0;
        let medium_spectrum_value = if medium_speed <= 0.0 {
            0
        } else if medium_speed <= 100.0 {
            100
        } else if medium_speed >= 300.0 {
            0
        } else {
            ((300.0 - medium_speed) / 2.0) as u8
        };
        let medium_colors = color_spectrum::value_to_spectrum(medium_spectrum_value);
        
        // Case 4: Slow typing (300 ms)
        let slow_speed = 300.0;
        let slow_spectrum_value = if slow_speed <= 0.0 {
            0
        } else if slow_speed <= 100.0 {
            100
        } else if slow_speed >= 300.0 {
            0
        } else {
            ((300.0 - slow_speed) / 2.0) as u8
        };
        let slow_colors = color_spectrum::value_to_spectrum(slow_spectrum_value);
        
        // Verify that the spectrum values are correctly calculated
        assert_eq!(no_data_spectrum_value, 0);
        assert_eq!(fast_spectrum_value, 100);
        assert_eq!(medium_spectrum_value, 50);
        assert_eq!(slow_spectrum_value, 0);
        
        // Different speeds should result in different background colors
        if let (Color::Rgb { r: r1, g: g1, b: b1 }, Color::Rgb { r: r2, g: g2, b: b2 }) = 
            (fast_colors.background, medium_colors.background) {
            // Fast (red) should be different from medium (white)
            assert!(r1 != r2 || g1 != g2 || b1 != b2);
        }
        
        if let (Color::Rgb { r: r1, g: g1, b: b1 }, Color::Rgb { r: r2, g: g2, b: b2 }) = 
            (medium_colors.background, slow_colors.background) {
            // Medium (white) should be different from slow (purple)
            assert!(r1 != r2 || g1 != g2 || b1 != b2);
        }
        
        // Verify color range
        if let Color::Rgb { r, g, b } = fast_colors.background {
            println!("Fast typing (90ms) color: RGB({}, {}, {})", r, g, b);
            // Should be reddish (high red component)
            assert!(r > g && r > b);
        }
        
        if let Color::Rgb { r, g, b } = medium_colors.background {
            println!("Medium typing (200ms) color: RGB({}, {}, {})", r, g, b);
            // Should be whitish (balanced RGB components)
            assert!(r > 200 && g > 200 && b > 200);
        }
        
        if let Color::Rgb { r, g, b } = slow_colors.background {
            println!("Slow typing (300ms) color: RGB({}, {}, {})", r, g, b);
            // Should be purplish (high red and blue components)
            assert!(r > g && b > g);
        }
    }
} 