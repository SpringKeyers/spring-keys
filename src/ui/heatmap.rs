#[cfg(test)]
use std::io::{self, Write};
use std::collections::HashMap;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor, ResetColor},
    ExecutableCommand,
};

/// Struct for rendering keyboard heatmaps
pub struct KeyboardHeatmap;

impl KeyboardHeatmap {
    /// Render a keyboard heatmap showing typing performance
    pub fn render(heat_map: &HashMap<char, f64>, stdout: &mut io::Stdout, x_pos: u16, y_pos: u16) -> io::Result<()> {
        // Keyboard rows
        let rows = [
            "1234567890-=", // Numbers row
            "qwertyuiop[]\\", // Top letter row
            "asdfghjkl;'", // Home row
            "zxcvbnm,./", // Bottom row
            "       ", // Space bar
        ];
        
        // Key width and height
        let key_width = 3;
        let key_height = 2;
        let key_spacing = 1;
        
        // Left edge indentations for each row to make it look like a keyboard
        let indents = [0, 2, 3, 4, 5];
        
        // Render each row
        for (row_idx, row) in rows.iter().enumerate() {
            let row_y = y_pos + (row_idx as u16 * (key_height + key_spacing));
            let indent = indents[row_idx];
            
            // Render each key in the row
            for (col_idx, key) in row.chars().enumerate() {
                let key_x = x_pos + indent + (col_idx as u16 * (key_width + key_spacing));
                
                // Get heat value for this key (or default to 0.5 for neutral)
                let heat_value = heat_map.get(&key).copied().unwrap_or(0.5);
                
                // Render the key
                Self::render_key(stdout, key_x, row_y, key_width, key_height, key, heat_value)?;
            }
        }
        
        Ok(())
    }
    
    /// Render a single keyboard key with heat coloring
    fn render_key(
        stdout: &mut io::Stdout, 
        x: u16, 
        y: u16, 
        width: u16, 
        height: u16, 
        key: char, 
        heat_value: f64
    ) -> io::Result<()> {
        // Determine color based on heat value (0.0 = fastest/coldest, 1.0 = slowest/hottest)
        let bg_color = if heat_value < 0.25 {
            Color::DarkBlue      // Very fast (cold)
        } else if heat_value < 0.5 {
            Color::Blue          // Fast
        } else if heat_value < 0.75 {
            Color::DarkYellow    // Medium
        } else {
            Color::DarkRed       // Slow (hot)
        };
        
        // Draw key background
        stdout.execute(SetBackgroundColor(bg_color))?;
        
        for dy in 0..height {
            for dx in 0..width {
                stdout.execute(MoveTo(x + dx, y + dy))?;
                stdout.write_all(b" ")?;
            }
        }
        
        // Draw key character in the center
        stdout.execute(MoveTo(x + width / 2, y + height / 2))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.write_all(&[key as u8])?;
        
        stdout.execute(ResetColor)?;
        Ok(())
    }
    
    /// Render a row performance bar showing typing speed by keyboard row
    pub fn render_row_performance(
        stdout: &mut io::Stdout, 
        x_pos: u16, 
        y_pos: u16, 
        width: u16,
        top_row_ms: f64,
        home_row_ms: f64,
        bottom_row_ms: f64
    ) -> io::Result<()> {
        // Row labels
        let rows = ["Top", "Home", "Bottom"];
        let values = [top_row_ms, home_row_ms, bottom_row_ms];
        
        // Find max value for normalization
        let max_value = values.iter().fold(0.0f64, |max: f64, &val| max.max(val)).max(500.0);
        
        // Draw header
        stdout.execute(MoveTo(x_pos, y_pos))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.write_all(b"Row Speed Performance (ms):")?;
        
        // Draw each row's performance bar
        for (i, (row, value)) in rows.iter().zip(values.iter()).enumerate() {
            let row_y = y_pos + 1 + i as u16;
            
            // Draw row label
            stdout.execute(MoveTo(x_pos, row_y))?;
            stdout.write_all(format!("{:6}: ", row).as_bytes())?;
            
            // Normalize value to bar width (0-100%)
            let normalized = (*value / max_value).min(1.0);
            let bar_width = (normalized * (width as f64)) as u16;
            
            // Choose color based on speed
            let color = if *value < 150.0 {
                Color::Green     // Fast
            } else if *value < 250.0 {
                Color::Yellow    // Medium
            } else {
                Color::Red       // Slow
            };
            
            // Draw the bar
            stdout.execute(SetBackgroundColor(color))?;
            for dx in 0..bar_width {
                stdout.execute(MoveTo(x_pos + 8 + dx, row_y))?;
                stdout.write_all(b" ")?;
            }
            
            // Show the value
            stdout.execute(SetForegroundColor(Color::White))?;
            stdout.execute(MoveTo(x_pos + 8 + bar_width + 1, row_y))?;
            stdout.write_all(format!("{:.1}", value).as_bytes())?;
        }
        
        stdout.execute(ResetColor)?;
        Ok(())
    }
    
    /// Render finger performance bars
    pub fn render_finger_performance(
        stdout: &mut io::Stdout,
        x_pos: u16,
        y_pos: u16,
        width: u16,
        finger_values: &HashMap<crate::core::metrics::Finger, f64>
    ) -> io::Result<()> {
        use crate::core::metrics::Finger;
        
        // Finger labels
        let left_fingers = [
            (Finger::LeftPinky, "L-Pinky"),
            (Finger::LeftRing, "L-Ring"),
            (Finger::LeftMiddle, "L-Middle"),
            (Finger::LeftIndex, "L-Index"),
        ];
        
        let right_fingers = [
            (Finger::RightIndex, "R-Index"),
            (Finger::RightMiddle, "R-Middle"),
            (Finger::RightRing, "R-Ring"),
            (Finger::RightPinky, "R-Pinky"),
        ];
        
        // Draw header
        stdout.execute(MoveTo(x_pos, y_pos))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.write_all(b"Finger Performance (ms):")?;
        
        // Find max value for normalization
        let max_value = finger_values.values().fold(0.0f64, |max: f64, &val| max.max(val)).max(500.0);
        
        // Draw left hand fingers
        for (i, (finger, label)) in left_fingers.iter().enumerate() {
            let row_y = y_pos + 1 + i as u16;
            
            // Draw finger label
            stdout.execute(MoveTo(x_pos, row_y))?;
            stdout.write_all(format!("{:8}: ", label).as_bytes())?;
            
            // Draw the performance bar if we have data
            if let Some(value) = finger_values.get(finger) {
                Self::draw_performance_bar(stdout, x_pos + 10, row_y, *value, max_value, width)?;
            }
        }
        
        // Draw right hand fingers
        for (i, (finger, label)) in right_fingers.iter().enumerate() {
            let row_y = y_pos + 1 + i as u16;
            
            // Draw finger label
            stdout.execute(MoveTo(x_pos + width + 20, row_y))?;
            stdout.write_all(format!("{:8}: ", label).as_bytes())?;
            
            // Draw the performance bar if we have data
            if let Some(value) = finger_values.get(finger) {
                Self::draw_performance_bar(stdout, x_pos + width + 30, row_y, *value, max_value, width)?;
            }
        }
        
        stdout.execute(ResetColor)?;
        Ok(())
    }
    
    /// Helper function to draw a performance bar
    fn draw_performance_bar(
        stdout: &mut io::Stdout,
        x_pos: u16,
        y_pos: u16,
        value: f64,
        max_value: f64,
        width: u16
    ) -> io::Result<()> {
        // Normalize value to bar width (0-100%)
        let normalized = (value / max_value).min(1.0);
        let bar_width = (normalized * (width as f64)) as u16;
        
        // Choose color based on speed
        let color = if value < 150.0 {
            Color::Green     // Fast
        } else if value < 250.0 {
            Color::Yellow    // Medium
        } else {
            Color::Red       // Slow
        };
        
        // Draw the bar
        stdout.execute(SetBackgroundColor(color))?;
        for dx in 0..bar_width {
            stdout.execute(MoveTo(x_pos + dx, y_pos))?;
            stdout.write_all(b" ")?;
        }
        
        // Show the value
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.execute(MoveTo(x_pos + bar_width + 1, y_pos))?;
        stdout.write_all(format!("{:.1}", value).as_bytes())?;
        
        stdout.execute(ResetColor)?;
        Ok(())
    }
} 