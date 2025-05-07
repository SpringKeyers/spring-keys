use crossterm::{
    style::{Color, Print, SetForegroundColor, SetBackgroundColor, ResetColor},
    cursor::MoveTo,
    queue,
};
use std::io::{self, Write};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use crate::{TypingMetrics, Finger};
use crate::ui::color_spectrum::{value_to_spectrum, get_contrasting_text_color};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use termion::color;
use crate::core::metrics::{ExtendedStats};

// Track key press animations
#[derive(Clone)]
struct KeyAnimation {
    last_press: Instant,
    previous_count: u32,
}

impl KeyAnimation {
    fn new() -> Self {
        Self {
            last_press: Instant::now() - Duration::from_secs(10), // Start inactive
            previous_count: 0,
        }
    }

    fn glow_intensity(&self) -> f64 {
        let elapsed = self.last_press.elapsed().as_millis() as f64;
        if elapsed >= 500.0 {
            0.0
        } else {
            // Smooth fade out with easing
            let t = 1.0 - (elapsed / 500.0);
            t * t * (3.0 - 2.0 * t) // Smoother cubic easing
        }
    }

    fn needs_redraw(&self) -> bool {
        // Only redraw if the key is still glowing
        self.last_press.elapsed() < Duration::from_millis(500)
    }
}

// Track key press animations using a thread-safe global
static KEY_ANIMATIONS: Lazy<Mutex<HashMap<char, KeyAnimation>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

fn get_animations() -> std::sync::MutexGuard<'static, HashMap<char, KeyAnimation>> {
    KEY_ANIMATIONS.lock().unwrap()
}

// Cache for previous frame's key states
static PREVIOUS_FRAME: Lazy<Mutex<HashMap<char, (Color, u32)>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

fn should_redraw_key(key: char, bg_color: Color, hits: u32) -> bool {
    let mut prev_frame = PREVIOUS_FRAME.lock().unwrap();
    let prev_state = prev_frame.get(&key);
    
    // Check if key state changed
    let should_redraw = match prev_state {
        Some(&(prev_color, prev_hits)) => {
            prev_color != bg_color || prev_hits != hits || get_animations().get(&key).map_or(false, |anim| anim.needs_redraw())
        }
        None => true // First time seeing this key
    };
    
    // Update cache if redrawing
    if should_redraw {
        prev_frame.insert(key, (bg_color, hits));
    }
    
    should_redraw
}

/// Get color based on typing speed using the spectrum
fn get_speed_color(speed_ms: f64) -> Color {
    value_to_spectrum(speed_ms)
}

/// Draw a bordered key with content
fn draw_key(
    stdout: &mut impl Write,
    x: u16,
    y: u16,
    width: usize,
    content: &[String],
    bg_color: Color,
    text_colors: &[Color],
    _is_active: bool, // Prefix with _ to indicate intentionally unused
) -> io::Result<()> {
    let content_height = content.len(); // Rename to be more specific
    
    // Top border
    queue!(
        stdout,
        MoveTo(x, y),
        SetForegroundColor(Color::White),
        Print("╭"),
        Print("─".repeat(width - 2)),
        Print("╮")
    )?;

    // Content lines with borders
    for (i, line) in content.iter().enumerate() {
        queue!(
            stdout,
            MoveTo(x, y + 1 + i as u16),
            SetForegroundColor(Color::White),
            Print("│"),
            SetBackgroundColor(bg_color),
            SetForegroundColor(text_colors[i]),
            Print(format!("{:^width$}", line, width = width - 2)),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::White),
            Print("│")
        )?;
    }

    // Bottom border
    queue!(
        stdout,
        MoveTo(x, y + content_height as u16 + 1),
        SetForegroundColor(Color::White),
        Print("╰"),
        Print("─".repeat(width - 2)),
        Print("╯"),
        ResetColor
    )?;

    Ok(())
}

/// Find the fastest and slowest speeds from the heat map data
fn find_speed_range(heat_map: &HashMap<char, f64>) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for &speed in heat_map.values() {
        min = min.min(speed);
        max = max.max(speed);
    }

    (min, max)
}

/// Unified keyboard visualization with large keys, hit counts, and color temperature
pub fn draw_unified_keyboard_heatmap(
    stdout: &mut impl Write,
    metrics: &TypingMetrics,
    y_offset: u16
) -> io::Result<()> {
    let heat_map = metrics.get_heat_map();
    let geometric_avgs = metrics.get_key_geometric_averages();
    
    // Find speed range for color normalization
    let (fastest, slowest) = find_speed_range(&heat_map);
    
    // Define keyboard layout
    let rows = [
        "1234567890-=",
        "qwertyuiop[]\\",
        "asdfghjkl;'",
        "zxcvbnm,./",
    ];
    
    let indents = [0, 0, 1, 2]; // Number of spaces to indent each row
    
    // Draw each row of the keyboard
    for (row_idx, (row, indent)) in rows.iter().zip(indents.iter()).enumerate() {
        let y = y_offset + (row_idx as u16 * 5); // 5 units per row for spacing
        
        // Draw each key in the row
        for (key_idx, c) in row.chars().enumerate() {
            let x = (*indent * 2 + key_idx * 10) as u16; // 10 units per key, 2 units per indent
            
            // Get heat map data for this key
            let speed = heat_map.get(&c).copied().unwrap_or(0.0);
            
            // Get geometric average for this key
            let geo_avg = geometric_avgs.get(&c).copied().unwrap_or(0.0);
            
            // Calculate normalized speed (0.0 to 1.0)
            let normalized_speed = if speed > 0.0 && slowest > fastest {
                (speed - fastest) / (slowest - fastest)
            } else {
                0.0
            };
            
            // Calculate background color based on normalized speed
            let bg_color = value_to_spectrum(normalized_speed);
            
            // Calculate text colors with contrast
            let text_colors = [
                get_contrasting_text_color(bg_color),
                get_contrasting_text_color(bg_color),
                get_contrasting_text_color(bg_color),
            ];
            
            // Format key content
            let content = vec![
                c.to_string(),
                format!("{} hits", heat_map.get(&c).copied().unwrap_or(0.0)),
                if geo_avg > 0.0 {
                    format!("{:.0}ms", geo_avg)
                } else {
                    "---".to_string()
                },
            ];
            
            // Draw the key with all its information
            draw_key(
                stdout,
                x,
                y,
                8, // key width
                &content,
                bg_color,
                &text_colors,
                heat_map.get(&c).copied().unwrap_or(0.0) > 0.0, // active if has hits
            )?;
        }
    }

    // Calculate base Y position for finger metrics (4 rows * 5 units + 2 units padding)
    let finger_metrics_y = y_offset + 22;
    
    // Draw finger performance metrics
    let finger_stats = metrics.finger_performance();
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

    // Draw finger metrics row
    for (i, (label, finger)) in fingers.iter().enumerate() {
        let x = i as u16 * 12; // Increased spacing

        if let Some(stats) = finger_stats.get(finger) {
            // Normalize finger speed against overall speed range
            let normalized_speed = if stats.current > 0.0 && slowest > fastest {
                (stats.current - fastest) / (slowest - fastest)
            } else {
                0.0
            };
            
            let speed_color = value_to_spectrum(normalized_speed);
            let bg_color = speed_color;
            let text_color = get_contrasting_text_color(bg_color);
            
            // Draw mini bordered metric
            draw_key(
                stdout,
                x,
                finger_metrics_y,
                9,
                &[label.to_string(), format!("{:3.0}ms", stats.current)],
                bg_color,
                &[text_color, text_color],
                stats.current > 0.0,
            )?;
        }
    }
    
    // Draw speed range info
    if slowest > fastest {
        queue!(
            stdout,
            MoveTo(0, finger_metrics_y + 4),
            SetForegroundColor(Color::White),
            Print(format!("Speed Range: {:.0}ms (fastest) to {:.0}ms (slowest)", fastest, slowest))
        )?;
    }

    // Draw legend at the bottom
    draw_legend(stdout, finger_metrics_y + 6)?;
    
    Ok(())
}

/// Draw a legend explaining the key information layout
fn draw_legend(
    stdout: &mut impl Write,
    y_offset: u16,
) -> io::Result<()> {
    // Draw color spectrum explanation
    queue!(
        stdout,
        MoveTo(0, y_offset),
        SetForegroundColor(Color::White),
        Print("Color Scale: ")
    )?;

    // Draw color spectrum samples
    let labels = ["Fastest", "Fast", "Medium", "Slow", "Slowest"];
    let values = [0.0, 0.25, 0.5, 0.75, 1.0];
    
    for (i, (&value, label)) in values.iter().zip(labels.iter()).enumerate() {
        let color = value_to_spectrum(value);
        queue!(
            stdout,
            MoveTo(13 + (i * 12) as u16, y_offset),
            SetBackgroundColor(color),
            Print("   "),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::White),
            MoveTo(13 + (i * 12) as u16, y_offset + 1),
            Print(label)
        )?;
    }

    Ok(())
}

pub fn draw_heat_map(metrics: &TypingMetrics) -> Option<String> {
    let heat_map = metrics.get_heat_map();
    let (fastest, slowest) = find_speed_range(&heat_map);
    
    let mut output = String::new();
    
    // Define keyboard layout
    let layout = [
        "QWERTYUIOP",
        "ASDFGHJKL;",
        "ZXCVBNM,./",
    ];

    // Draw keyboard
    for row in layout.iter() {
        for c in row.chars() {
            let speed = heat_map.get(&c).copied().unwrap_or(0.0);
            let normalized = if slowest == fastest {
                0.5
            } else {
                (speed - fastest) / (slowest - fastest)
            };
            
            // Generate color based on speed
            let color = get_heat_color(normalized);
            output.push_str(&format!("{} ", color));
        }
        output.push('\n');
    }

    Some(output)
}

fn get_heat_color(normalized: f64) -> String {
    // Convert normalized value to RGB
    let r = (255.0 * normalized) as u8;
    let g = (255.0 * (1.0 - normalized)) as u8;
    let b = 0;
    
    format!("\x1b[38;2;{};{};{}m█\x1b[0m", r, g, b)
}

fn draw_finger_metrics(
    stdout: &mut io::Stdout,
    x: u16,
    y: u16,
    label: &str,
    stats: &ExtendedStats,
    text_color: Color,
    bg_color: Color,
) -> io::Result<()> {
    draw_key(
        stdout,
        x,
        y,
        9,
        &[label.to_string(), format!("{:3.0}ms", stats.current)],
        bg_color,
        &[text_color, text_color],
        stats.current > 0.0,
    )
} 