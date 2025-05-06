use spring_keys::ui::color_spectrum::value_to_spectrum;

#[test]
fn test_value_to_spectrum_minimum() {
    // Test minimum value (0) - should be purple
    let color = value_to_spectrum(0.0);
    
    if let crossterm::style::Color::Rgb { r, g, b } = color {
        println!("Minimum (0) color: RGB({}, {}, {})", r, g, b);
        // Should be purple (128, 0, 128)
        assert_eq!(r, 128, "Red component should be 128 for purple");
        assert_eq!(g, 0, "Green component should be 0 for purple");
        assert_eq!(b, 128, "Blue component should be 128 for purple");
    } else {
        panic!("Color should be RGB");
    }
}

#[test]
fn test_value_to_spectrum_middle() {
    // Test middle value (0.5) - should be green
    let color = value_to_spectrum(0.5);
    
    if let crossterm::style::Color::Rgb { r, g, b } = color {
        println!("Middle (0.5) color: RGB({}, {}, {})", r, g, b);
        // Should be green (0, 255, 0)
        assert_eq!(r, 0, "Red component should be 0 for green");
        assert_eq!(g, 255, "Green component should be 255 for green");
        assert_eq!(b, 0, "Blue component should be 0 for green");
    } else {
        panic!("Color should be RGB");
    }
}

#[test]
fn test_value_to_spectrum_maximum() {
    // Test maximum value (1.0) - should be red
    let color = value_to_spectrum(1.0);
    
    if let crossterm::style::Color::Rgb { r, g, b } = color {
        println!("Maximum (1.0) color: RGB({}, {}, {})", r, g, b);
        // Should be red (255, 0, 0)
        assert_eq!(r, 255, "Red component should be 255 for red");
        assert_eq!(g, 0, "Green component should be 0 for red");
        assert_eq!(b, 0, "Blue component should be 0 for red");
    } else {
        panic!("Color should be RGB");
    }
}

#[test]
fn test_value_to_spectrum_range() {
    // Test the full range of values
    let samples = [0.0, 0.25, 0.5, 0.75, 1.0];
    
    // Store RGB values for each sample
    let mut values = Vec::new();
    
    for sample in samples {
        let color = value_to_spectrum(sample);
        
        if let crossterm::style::Color::Rgb { r, g, b } = color {
            println!("Sample {} color: RGB({}, {}, {})", sample, r, g, b);
            values.push((sample, r, g, b));
        } else {
            panic!("Color should be RGB");
        }
    }
    
    // Verify color transitions
    // 0.0: Purple (128, 0, 128)
    assert_eq!(values[0], (0.0, 128, 0, 128), "Start should be purple");
    
    // 0.25: Blue (0, 0, 255)
    assert_eq!(values[1], (0.25, 0, 0, 255), "Quarter should be blue");
    
    // 0.5: Green (0, 255, 0)
    assert_eq!(values[2], (0.5, 0, 255, 0), "Middle should be green");
    
    // 0.75: Orange (255, 165, 0)
    assert_eq!(values[3], (0.75, 255, 165, 0), "Three quarters should be orange");
    
    // 1.0: Red (255, 0, 0)
    assert_eq!(values[4], (1.0, 255, 0, 0), "End should be red");
} 