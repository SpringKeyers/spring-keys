use crossterm::style::Color;

/// Represents a color pair with background and foreground colors
pub struct ColorPair {
    pub background: Color,
}

impl ColorPair {
    pub fn new(background: Color) -> Self {
        Self { background }
    }
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
pub fn value_to_spectrum(value: f64) -> Color {
    // Clamp value between 0 and 100
    let clamped_value = value.min(100.0).max(0.0);
    
    // Calculate the color based on the range
    if clamped_value < 34.0 {
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
        
        Color::Rgb { r: bg_r, g: bg_g, b: bg_b }
    } else if clamped_value < 67.0 {
        // Middle range - White (34-66)
        // Background: Light gray, Foreground: White
        Color::Rgb { r: 230, g: 230, b: 230 }
    } else {
        // White to Red gradient (67-100)
        let factor: f32 = (clamped_value - 67.0) as f32 / 33.0;
        
        // White RGB: (255, 255, 255) to Red RGB: (255, 0, 0)
        let r: f32 = 255.0;
        let g: f32 = interpolate(255.0, 0.0, factor);
        let b: f32 = interpolate(255.0, 0.0, factor);
        
        // Create darker background and lighter foreground
        let bg_r: u8 = (r * 0.7) as u8;
        let bg_g: u8 = (g * 0.7) as u8;
        let bg_b: u8 = (b * 0.7) as u8;
        
        Color::Rgb { r: bg_r, g: bg_g, b: bg_b }
    }
}

/// Helper function to interpolate between two values based on a factor (0.0 to 1.0)
fn interpolate(start: f32, end: f32, factor: f32) -> f32 {
    start + (end - start) * factor
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_minimum_value() {
        // Test minimum value (0) - should be dark purple
        let color = value_to_spectrum(0.0);
        if let Color::Rgb { r, g, b } = color {
            assert_eq!(r, 89); // 128 * 0.7
            assert_eq!(g, 0);
            assert_eq!(b, 89); // 128 * 0.7
        }
    }
    
    #[test]
    fn test_mid_value() {
        // Test mid value (50) - should be white/light gray
        let color = value_to_spectrum(50.0);
        if let Color::Rgb { r, g, b } = color {
            assert_eq!(r, 230);
            assert_eq!(g, 230);
            assert_eq!(b, 230);
        }
    }
    
    #[test]
    fn test_maximum_value() {
        // Test maximum value (100) - should be red
        let color = value_to_spectrum(100.0);
        if let Color::Rgb { r, g, b } = color {
            assert_eq!(r, 178); // 255 * 0.7
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        }
    }
    
    #[test]
    fn test_interpolation_purple_to_white() {
        // Test a value in the purple-white range (16)
        let color = value_to_spectrum(16.0);
        if let Color::Rgb { r, g, b } = color {
            // Should be between purple and white
            assert!(r > 89 && r < 178);
            assert!(g > 0 && g < 230);
            assert!(b > 89 && b < 178);
        }
    }
    
    #[test]
    fn test_interpolation_white_to_red() {
        // Test a value in the white-red range (83)
        let color = value_to_spectrum(83.0);
        if let Color::Rgb { r, g, b } = color {
            // Should be between white and red
            assert_eq!(r, 178); // 255 * 0.7
            assert!(g < 230 && g > 0);
            assert!(b < 230 && b > 0);
        }
    }
} 