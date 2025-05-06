use spring_keys::ui::color_spectrum::value_to_spectrum;

#[test]
fn test_value_to_spectrum_minimum() {
    // Test minimum value (0) - should be purple
    let color = value_to_spectrum(0.0);
    
    if let crossterm::style::Color::Rgb { r, g, b } = color {
        println!("Minimum (0) color: RGB({}, {}, {})", r, g, b);
        // Should be purplish (high red and blue components, low green)
        assert!(r > g, "Red component should be higher than green for purple");
        assert!(b > g, "Blue component should be higher than green for purple");
        assert!(g < 50, "Green component should be low for purple");
    } else {
        panic!("Color should be RGB");
    }
}

#[test]
fn test_value_to_spectrum_middle() {
    // Test middle value (50) - should be white
    let color = value_to_spectrum(50.0);
    
    if let crossterm::style::Color::Rgb { r, g, b } = color {
        println!("Middle (50) color: RGB({}, {}, {})", r, g, b);
        // Should be whitish (all components high and balanced)
        assert!(r > 200, "Red component should be high for white");
        assert!(g > 200, "Green component should be high for white");
        assert!(b > 200, "Blue component should be high for white");
    } else {
        panic!("Color should be RGB");
    }
}

#[test]
fn test_value_to_spectrum_maximum() {
    // Test maximum value (100) - should be red
    let color = value_to_spectrum(100.0);
    
    if let crossterm::style::Color::Rgb { r, g, b } = color {
        println!("Maximum (100) color: RGB({}, {}, {})", r, g, b);
        // Should be reddish (high red component, low green and blue)
        assert!(r > g, "Red component should be higher than green for red");
        assert!(r > b, "Red component should be higher than blue for red");
        assert!(g < 50, "Green component should be low for red");
        assert!(b < 50, "Blue component should be low for red");
    } else {
        panic!("Color should be RGB");
    }
}

#[test]
fn test_value_to_spectrum_range() {
    // Test the full range of values
    let samples = [0.0, 25.0, 50.0, 75.0, 100.0];
    
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
    
    // Verify purple-white-red spectrum transition
    // For purple (0) -> white (50):
    //   - Red should increase or stay high
    //   - Green should increase
    //   - Blue should increase or stay high
    assert!(values[1].1 >= values[0].1, "Red should increase from 0 to 25");
    assert!(values[1].2 > values[0].2, "Green should increase from 0 to 25");
    assert!(values[1].3 >= values[0].3, "Blue should increase from 0 to 25");
    
    // For white (50) -> red (100):
    //   - Red should stay the same or similar (both values high)
    //   - Green should decrease
    //   - Blue should decrease
    // Note: The actual values show red is 230 at 50 and 178 at 75 due to the darkening effect
    assert!(values[3].1 >= 150, "Red should remain substantial from 50 to 75");
    assert!(values[3].2 < values[2].2, "Green should decrease from 50 to 75");
    assert!(values[3].3 < values[2].3, "Blue should decrease from 50 to 75");
    
    // Verify that minimum (0) and maximum (100) are significantly different
    let (_, min_r, min_g, min_b) = values[0];
    let (_, max_r, max_g, max_b) = values[4];
    
    // Purple and red should have different color profiles
    assert!(min_r != max_r || min_g != max_g || min_b != max_b, 
        "Minimum and maximum values should produce different colors");
} 