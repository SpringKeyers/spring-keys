use std::io::{self, Write};
use std::collections::HashMap;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor, ResetColor},
    ExecutableCommand,
};
use crate::core::metrics::ExtendedStats;

/// Struct for rendering keyboard heatmaps
pub struct KeyboardHeatmap;

impl KeyboardHeatmap {
    /// Render a keyboard heatmap showing typing performance
    pub fn render(heat_map: &HashMap<char, (f64, u64)>, stdout: &mut io::Stdout, x_pos: u16, y_pos: u16) -> io::Result<()> {
        // Keyboard rows
        let rows = [
            "1234567890-=", // Numbers row
            "qwertyuiop[]\\", // Top letter row
            "asdfghjkl;'", // Home row
            "zxcvbnm,./", // Bottom row
            "       ", // Space bar
        ];
        
        // Key width and height (adjusted for compact display)
        let key_width = 6;  // Width to fit "123/456"
        let key_height = 3; // 3 rows: char, last, avg
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
                
                // Get heat value and speed for this key
                let (heat_value, last_delay) = heat_map.get(&key).copied().unwrap_or((0.5, 0));
                
                // Calculate geometric average
                let geo_avg = if heat_value == 0.5 {
                    0.0 // No data
                } else {
                    heat_value * 400.0 + 100.0 // Convert normalized heat to ms
                };
                
                // Render the key
                Self::render_key(stdout, key_x, row_y, key_width, key_height, key, heat_value, last_delay, geo_avg)?;
            }
        }
        
        Ok(())
    }
    
    /// Render a single keyboard key with heat coloring and speed values
    fn render_key(
        stdout: &mut io::Stdout, 
        x: u16, 
        y: u16, 
        width: u16, 
        height: u16, 
        key: char, 
        heat_value: f64,
        last_delay: u64,
        geo_avg: f64
    ) -> io::Result<()> {
        // Determine background color based on heat value
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
            stdout.execute(MoveTo(x, y + dy))?;
            for _ in 0..width {
                stdout.write_all(b" ")?;
            }
        }
        
        // Get color for last delay
        let last_color = if last_delay == 0 {
            Color::White
        } else if last_delay < 80 {
            Color::Green       // Ultra fast
        } else if last_delay < 120 {
            Color::Blue       // Very fast
        } else if last_delay < 200 {
            Color::Yellow     // Medium
        } else if last_delay < 350 {
            Color::Red        // Slow
        } else {
            Color::Magenta    // Needs practice
        };

        // Get color for geometric average
        let geo_color = if geo_avg == 0.0 {
            Color::White
        } else if geo_avg < 80.0 {
            Color::Green       // Ultra fast
        } else if geo_avg < 120.0 {
            Color::Blue       // Very fast
        } else if geo_avg < 200.0 {
            Color::Yellow     // Medium
        } else if geo_avg < 350.0 {
            Color::Red        // Slow
        } else {
            Color::Magenta    // Needs practice
        };
        
        // Draw key information in 3 rows:
        // 1. Key character (centered)
        // 2. Last delay (just the number)
        // 3. Geometric average (just the number)
        
        // Row 1: Key character (centered)
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.execute(MoveTo(x + width / 2, y))?;
        stdout.write_all(&[key as u8])?;
        
        // Row 2: Last delay (just number)
        stdout.execute(SetForegroundColor(last_color))?;
        stdout.execute(MoveTo(x + 1, y + 1))?;
        if last_delay > 0 {
            stdout.write_all(format!("{:3}", last_delay).as_bytes())?;
        } else {
            stdout.write_all(b"---")?;
        }
        
        // Row 3: Geometric average (just number)
        stdout.execute(SetForegroundColor(geo_color))?;
        stdout.execute(MoveTo(x + 1, y + 2))?;
        if geo_avg > 0.0 {
            stdout.write_all(format!("{:3.0}", geo_avg).as_bytes())?;
        } else {
            stdout.write_all(b"---")?;
        }
        
        stdout.execute(ResetColor)?;
        Ok(())
    }
    
    /// Render a row performance bar showing typing speed by keyboard row
    pub fn render_row_performance(
        stdout: &mut io::Stdout, 
        x_pos: u16, 
        y_pos: u16, 
        width: u16,
        number_row_ms: f64,
        top_row_ms: f64,
        home_row_ms: f64,
        bottom_row_ms: f64
    ) -> io::Result<()> {
        // Row labels and values
        let rows = ["Numbers", "Top", "Home", "Bottom"];
        let values = [number_row_ms, top_row_ms, home_row_ms, bottom_row_ms];
        
        // Find max value for normalization
        let max_value = values.iter().fold(0.0f64, |max: f64, val| max.max(*val)).max(500.0);
        
        // Draw header
        stdout.execute(MoveTo(x_pos, y_pos))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.write_all(b"Row Speed Performance:")?;
        
        // Draw each row's performance bar
        for (i, (row, value)) in rows.iter().zip(values.iter()).enumerate() {
            let row_y = y_pos + 1 + i as u16;
            
            // Choose color based on speed
            let color = if *value < 150.0 {
                Color::Green     // Fast
            } else if *value < 250.0 {
                Color::Yellow    // Medium
            } else {
                Color::Red       // Slow
            };
            
            // Draw row label and value first (left-aligned)
            stdout.execute(MoveTo(x_pos, row_y))?;
            stdout.execute(SetForegroundColor(color))?;
            stdout.write_all(format!("{:7} {:5.0}ms ", row, value).as_bytes())?;
            
            // Calculate bar position and width
            let bar_x = x_pos + 15; // After label and value
            let available_width = width.saturating_sub(17); // Leave space for label and value
            
            // Normalize value to bar width (0-100%)
            let normalized = (*value / max_value).min(1.0);
            let bar_width = (normalized * (available_width as f64)) as u16;
            
            // Draw the bar
            stdout.execute(SetBackgroundColor(color))?;
            for dx in 0..bar_width {
                stdout.execute(MoveTo(bar_x + dx, row_y))?;
                stdout.write_all(b" ")?;
            }
            stdout.execute(ResetColor)?;
        }
        
        stdout.execute(ResetColor)?;
        Ok(())
    }
    
    /// Helper function to format time in milliseconds
    fn format_ms(value: f64) -> String {
        if value.is_infinite() || value.is_nan() || value == 0.0 {
            return "---".to_string();
        }
        format!("{:5.0}ms", value)
    }

    /// Renders a compact finger performance chart showing metrics for each finger (excluding thumb).
    /// 
    /// The chart displays:
    /// - Column headers: Two-letter finger abbreviations (LP=Left Pinky, LR=Left Ring, etc.)
    /// - Row metrics: Current speed, 10s/60s averages, fastest/slowest times
    /// - Color-coded values based on speed:
    ///   * Green: < 80ms (ultra fast)
    ///   * Blue: 80-120ms (very fast)
    ///   * Yellow: 120-200ms (medium)
    ///   * Red: 200-350ms (slow)
    ///   * Magenta: > 350ms (needs practice)
    /// 
    /// Example output:
    /// ```text
    /// Finger Performance (ms):
    ///           LP     LR     LM     LI     RI     RM     RR     RP
    /// Current   123    145    167    134    156    178    189    234
    /// 10s Avg   134    156    145    167    145    167    178    245
    /// 60s Avg   145    167    156    178    156    178    189    256
    /// Fastest    89     95    102     98     95    105    112    145
    /// Slowest   234    256    278    245    267    289    301    345
    /// ```
    pub fn render_finger_performance(
        stdout: &mut io::Stdout,
        x_pos: u16,
        y_pos: u16,
        finger_metrics: &HashMap<crate::core::metrics::Finger, ExtendedStats>
    ) -> io::Result<()> {
        use crate::core::metrics::Finger;
        
        // Define fingers in order from left to right (excluding thumb)
        let fingers = [
            (Finger::LeftPinky, "LP"),
            (Finger::LeftRing, "LR"),
            (Finger::LeftMiddle, "LM"),
            (Finger::LeftIndex, "LI"),
            (Finger::RightIndex, "RI"),
            (Finger::RightMiddle, "RM"),
            (Finger::RightRing, "RR"),
            (Finger::RightPinky, "RP"),
        ];

        // Define metrics to display with their accessor functions
        type StatGetter = Box<dyn Fn(&ExtendedStats) -> f64>;
        let metrics: [(&str, StatGetter); 5] = [
            ("Current", Box::new(|stats| stats.current)),
            ("10s Avg", Box::new(|stats| stats.avg_10s)),
            ("60s Avg", Box::new(|stats| stats.avg_60s)),
            ("Fastest", Box::new(|stats| stats.fastest)),
            ("Slowest", Box::new(|stats| stats.slowest)),
        ];

        // Draw header
        stdout.execute(MoveTo(x_pos, y_pos))?;
        stdout.execute(SetForegroundColor(Color::White))?;
        stdout.write_all(b"Finger Performance (ms):")?;

        // Draw column headers (finger labels)
        stdout.execute(MoveTo(x_pos + 10, y_pos + 1))?;
        for (_, label) in &fingers {
            stdout.execute(SetForegroundColor(Color::White))?;
            stdout.write_all(format!("{:>6}", label).as_bytes())?;
            stdout.write_all(b" ")?;
        }

        // Draw each metric row
        for (row_idx, (metric_name, get_value)) in metrics.iter().enumerate() {
            let row_y = y_pos + 2 + row_idx as u16;
            
            // Write metric name
            stdout.execute(MoveTo(x_pos, row_y))?;
            stdout.execute(SetForegroundColor(Color::White))?;
            stdout.write_all(format!("{:<9}", metric_name).as_bytes())?;
            stdout.write_all(b" ")?;

            // Write values for each finger with color coding
            for (finger, _) in &fingers {
                if let Some(stats) = finger_metrics.get(finger) {
                    let value = get_value(stats);
                    
                    // Simplified color coding based on speed thresholds
                    let color = if value < 80.0 {
                        Color::Green     // Ultra fast
                    } else if value < 120.0 {
                        Color::Blue      // Very fast
                    } else if value < 200.0 {
                        Color::Yellow    // Medium
                    } else if value < 350.0 {
                        Color::Red       // Slow
                    } else {
                        Color::Magenta   // Needs practice
                    };
                    
                    stdout.execute(SetForegroundColor(color))?;
                    stdout.write_all(format!("{:>6.0}", value).as_bytes())?;
                } else {
                    stdout.execute(SetForegroundColor(Color::White))?;
                    stdout.write_all(b"   ---")?;
                }
                stdout.write_all(b" ")?;
            }
        }

        stdout.execute(ResetColor)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Add tests here
} 