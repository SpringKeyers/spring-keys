use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::core::metrics::{ExtendedStats, TypingMetrics, Finger};
use crate::ui::color_spectrum::{value_to_spectrum, get_contrasting_text_color};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Track key press animations
#[derive(Clone)]
struct KeyAnimation {
    last_press: Instant,
    previous_count: u32,
}

impl KeyAnimation {
    fn new() -> Self {
        // Initialize as if pressed 1s ago (fully faded)
        Self {
            last_press: Instant::now() - Duration::from_secs(1),
            previous_count: 0,
        }
    }

    fn glow_intensity(&self) -> f64 {
        // Fade over 1 second
        let elapsed = self.last_press.elapsed().as_millis() as f64;
        if elapsed >= 1000.0 {
            0.0
        } else {
            let t = 1.0 - (elapsed / 1000.0);
            // Smooth easing function for more natural fade
            t * t * (3.0 - 2.0 * t)
        }
    }

    fn needs_redraw(&self) -> bool {
        // Only redraw if within fade period of 1 second
        self.last_press.elapsed() < Duration::from_secs(1)
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
    let content_height = content.len();
    
    // Determine dynamic border color (fade from bright purple to black)
    let mut border_color = Color::Black;
    if let Some(key_str) = content.get(0) {
        if key_str.chars().count() == 1 {
            let c = key_str.chars().next().unwrap();
            let intensity = {
                let anims = get_animations();
                anims.get(&c).map(|a| a.glow_intensity()).unwrap_or(0.0)
            };
            if intensity > 0.0 {
                // Bright purple RGB(255,0,255) fades to black
                let v = (255.0 * intensity) as u8;
                border_color = Color::Rgb { r: v, g: 0, b: v };
            }
        }
    }

    // Content lines with borders
    for (i, line) in content.iter().enumerate() {
        queue!(
            stdout,
            MoveTo(x, y + i as u16),
            SetForegroundColor(border_color),
            Print("│"),
            SetBackgroundColor(bg_color),
            SetForegroundColor(text_colors[i]),
            Print(format!("{:^width$}", line, width = width - 2)),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(border_color),
            Print("│")
        )?;
    }

    // Bottom border
    queue!(
        stdout,
        MoveTo(x, y + content_height as u16),
        SetForegroundColor(border_color),
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
        let y = y_offset + (row_idx as u16 * 6); // 6 units per row (4 lines + 2 borders)
        
        // Draw each key in the row
        for (key_idx, c) in row.chars().enumerate() {
            let x = (*indent * 2 + key_idx * 10) as u16; // 10 units per key, 2 units per indent
            
            // Get per-key metrics: count, geometric average, and last speed
            let timings = metrics.key_timings.get(&c).map(|v| v.as_slice()).unwrap_or(&[]);
            let count = timings.len();
            let geo_avg = geometric_avgs.get(&c).copied().unwrap_or(0.0);
            let last_speed = timings.last().copied().unwrap_or(0.0);
            
            // Calculate normalized speed (0.0 to 1.0)
            let normalized_speed = if geo_avg > 0.0 && slowest > fastest {
                (geo_avg - fastest) / (slowest - fastest)
            } else {
                0.0
            };
            
            // Calculate background color based on normalized speed and text colors
            let bg_color = value_to_spectrum(normalized_speed);
            let text_colors = vec![
                get_contrasting_text_color(bg_color),
                get_contrasting_text_color(bg_color),
                get_contrasting_text_color(bg_color),
                get_contrasting_text_color(bg_color),
            ];
            
            // Format key content: char, count, geo avg, last speed
            let content = vec![
                c.to_string(),
                format!("{} hits", count),
                if geo_avg > 0.0 { format!("{:.0}ms", geo_avg) } else { "---".to_string() },
                if last_speed > 0.0 { format!("{:.0}ms", last_speed) } else { "---".to_string() },
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
                !timings.is_empty(),
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

// Public API to register a key press for animation
pub fn register_key_press(key: char) {
    let mut animations = KEY_ANIMATIONS.lock().unwrap();
    let entry = animations.entry(key).or_insert_with(KeyAnimation::new);
    entry.last_press = Instant::now();
} 