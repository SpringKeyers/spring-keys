use crossterm::style::{Color, Colors};

/// Represents a color pair with background and foreground colors
pub struct ColorPair {
    pub background: Color,
    pub foreground: Color,
}

/// Maps a value from 0 to 100 onto a purple-white-red spectrum
/// - 0-33: Purple to White gradient (minimum to mid)
/// - 34-66: White (mid range)
/// - 67-100: White to Red gradient (mid to maximum)
///
/// Background colors are darker variants of the hue
/// Foreground colors are lighter variants of the hue
///
/// Returns a ColorPair with background and foreground colors
pub fn value_to_spectrum(value: u8) -> ColorPair {
    // Clamp value between 0 and 100
    let clamped_value = value.min(100);
    
    // Calculate the color based on the range
    if clamped_value < 34 {
        // Purple to White gradient (0-33)
        let factor: f32 = clamped_value as f32 / 33.0;
        
        // Purple RGB: (128, 0, 128) to White RGB: (255, 255, 255)
        let r: f32 = interpolate(128.0, 255.0, factor);
        let g: f32 = interpolate(0.0, 255.0, factor);
        let b: f32 = interpolate(128.0, 255.0, factor);
        
        // Create darker background and lighter foreground
        let bg_r: u8 = (r * 0.7) as u8;
        let bg_g: u8 = (g * 0.7) as u8;
        let bg_b: u8 = (b * 0.7) as u8;
        
        let fg_r: u8 = r.min(255.0) as u8;
        let fg_g: u8 = g.min(255.0) as u8;
        let fg_b: u8 = b.min(255.0) as u8;
        
        ColorPair {
            background: Color::Rgb { r: bg_r, g: bg_g, b: bg_b },
            foreground: Color::Rgb { r: fg_r, g: fg_g, b: fg_b },
        }
    } else if clamped_value < 67 {
        // Middle range - White (34-66)
        // Background: Light gray, Foreground: White
        ColorPair {
            background: Color::Rgb { r: 230, g: 230, b: 230 },
            foreground: Color::Rgb { r: 255, g: 255, b: 255 },
        }
    } else {
        // White to Red gradient (67-100)
        let factor: f32 = (clamped_value - 67) as f32 / 33.0;
        
        // White RGB: (255, 255, 255) to Red RGB: (255, 0, 0)
        let r: f32 = 255.0;
        let g: f32 = interpolate(255.0, 0.0, factor);
        let b: f32 = interpolate(255.0, 0.0, factor);
        
        // Create darker background and lighter foreground
        let bg_r: u8 = (r * 0.7) as u8;
        let bg_g: u8 = (g * 0.7) as u8;
        let bg_b: u8 = (b * 0.7) as u8;
        
        let fg_r: u8 = r.min(255.0) as u8;
        let fg_g: u8 = g.min(255.0) as u8;
        let fg_b: u8 = b.min(255.0) as u8;
        
        ColorPair {
            background: Color::Rgb { r: bg_r, g: bg_g, b: bg_b },
            foreground: Color::Rgb { r: fg_r, g: fg_g, b: fg_b },
        }
    }
}

/// Helper function to interpolate between two values based on a factor (0.0 to 1.0)
fn interpolate(start: f32, end: f32, factor: f32) -> f32 {
    start + (end - start) * factor
}

/// Generates crossterm style Colors from a ColorPair
pub fn to_crossterm_colors(pair: &ColorPair) -> Colors {
    Colors {
        foreground: Some(pair.foreground),
        background: Some(pair.background),
    }
}

/// Utility function for converting a percentage to color
/// Maps a value from 0 to 100 to the color spectrum
pub fn percentage_to_colors(percentage: u8) -> Colors {
    to_crossterm_colors(&value_to_spectrum(percentage))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_minimum_value() {
        // Test minimum value (0) - should be dark purple
        let pair = value_to_spectrum(0);
        if let Color::Rgb { r, g, b } = pair.background {
            assert_eq!(r, 89); // 128 * 0.7
            assert_eq!(g, 0);
            assert_eq!(b, 89); // 128 * 0.7
        }
    }
    
    #[test]
    fn test_mid_value() {
        // Test mid value (50) - should be white/light gray
        let pair = value_to_spectrum(50);
        if let Color::Rgb { r, g, b } = pair.background {
            assert_eq!(r, 230);
            assert_eq!(g, 230);
            assert_eq!(b, 230);
        }
    }
    
    #[test]
    fn test_maximum_value() {
        // Test maximum value (100) - should be red
        let pair = value_to_spectrum(100);
        if let Color::Rgb { r, g, b } = pair.background {
            assert_eq!(r, 178); // 255 * 0.7
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        }
    }
    
    #[test]
    fn test_interpolation_purple_to_white() {
        // Test a value in the purple-white range (16)
        let pair = value_to_spectrum(16);
        if let Color::Rgb { r, g, b } = pair.background {
            // Should be between purple and white
            assert!(r > 89 && r < 178);
            assert!(g > 0 && g < 230);
            assert!(b > 89 && b < 178);
        }
    }
    
    #[test]
    fn test_interpolation_white_to_red() {
        // Test a value in the white-red range (83)
        let pair = value_to_spectrum(83);
        if let Color::Rgb { r, g, b } = pair.background {
            // Should be between white and red
            assert_eq!(r, 178); // 255 * 0.7
            assert!(g < 230 && g > 0);
            assert!(b < 230 && b > 0);
        }
    }
} 