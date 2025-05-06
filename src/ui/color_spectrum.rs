use crossterm::style::Color;

/// Convert a normalized value (0.0 to 1.0) to a color in the spectrum
/// 0.0 = fastest (purple), 1.0 = slowest (red)
pub fn value_to_spectrum(normalized: f64) -> Color {
    // Ensure value is in 0-1 range
    let value = normalized.clamp(0.0, 1.0);

    // Define color stops
    let stops = [
        (0.0, (128, 0, 128)),   // Purple (fastest)
        (0.25, (0, 0, 255)),    // Blue
        (0.5, (0, 255, 0)),     // Green
        (0.75, (255, 165, 0)),  // Orange
        (1.0, (255, 0, 0)),     // Red (slowest)
    ];

    // Find the two color stops to interpolate between
    let mut lower = &stops[0];
    let mut upper = &stops[1];
    for window in stops.windows(2) {
        if value >= window[0].0 && value <= window[1].0 {
            lower = &window[0];
            upper = &window[1];
            break;
        }
    }

    // Calculate interpolation factor
    let factor = (value - lower.0) / (upper.0 - lower.0);

    // Interpolate RGB values
    let r = (lower.1.0 as f64 + (upper.1.0 as f64 - lower.1.0 as f64) * factor) as u8;
    let g = (lower.1.1 as f64 + (upper.1.1 as f64 - lower.1.1 as f64) * factor) as u8;
    let b = (lower.1.2 as f64 + (upper.1.2 as f64 - lower.1.2 as f64) * factor) as u8;

    Color::Rgb { r, g, b }
}

/// Get contrasting text color (black or white) for a given background color
pub fn get_contrasting_text_color(bg: Color) -> Color {
    if let Color::Rgb { r, g, b } = bg {
        // Calculate perceived brightness using the formula:
        // (R * 299 + G * 587 + B * 114) / 1000
        let brightness = (r as f64 * 299.0 + g as f64 * 587.0 + b as f64 * 114.0) / 1000.0;
        
        if brightness > 128.0 {
            Color::Black
        } else {
            Color::White
        }
    } else {
        Color::White // Default to white for non-RGB colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fastest_speed() {
        // Test fastest speed (0.0) - should be deep purple
        let color = value_to_spectrum(0.0);
        if let Color::Rgb { r, g, b } = color {
            assert_eq!(r, 128);
            assert_eq!(g, 0);
            assert_eq!(b, 128);
        }
    }
    
    #[test]
    fn test_mid_speed() {
        // Test mid speed (0.5) - should be lime green
        let color = value_to_spectrum(0.5);
        if let Color::Rgb { r, g, b } = color {
            assert_eq!(r, 0);
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
    }
    
    #[test]
    fn test_slowest_speed() {
        // Test slowest speed (1.0) - should be deep red
        let color = value_to_spectrum(1.0);
        if let Color::Rgb { r, g, b } = color {
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        }
    }
    
    #[test]
    fn test_quarter_speed() {
        // Test quarter speed (0.25) - should be blue
        let color = value_to_spectrum(0.25);
        if let Color::Rgb { r, g, b } = color {
            assert_eq!(r, 0);
            assert_eq!(g, 0);
            assert_eq!(b, 255);
        }
    }
    
    #[test]
    fn test_three_quarter_speed() {
        // Test three-quarter speed (0.75) - should be orange
        let color = value_to_spectrum(0.75);
        if let Color::Rgb { r, g, b } = color {
            assert_eq!(r, 255);
            assert_eq!(g, 165);
            assert_eq!(b, 0);
        }
    }
    
    #[test]
    fn test_out_of_range_values() {
        // Test values outside 0-1 range are clamped
        let below = value_to_spectrum(-0.5);
        let above = value_to_spectrum(1.5);
        
        if let Color::Rgb { r, g, b } = below {
            assert_eq!(r, 128); // Should be purple (0.0)
            assert_eq!(g, 0);
            assert_eq!(b, 128);
        }
        
        if let Color::Rgb { r, g, b } = above {
            assert_eq!(r, 255); // Should be red (1.0)
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        }
    }
} 