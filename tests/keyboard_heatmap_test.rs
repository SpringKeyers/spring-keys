#[cfg(test)]
mod tests {
    
    // Import the required types
    use spring_keys::TypingMetrics;
    use spring_keys::ui::heatmap;
    
    #[test]
    fn test_unified_heatmap_drawing() {
        // Create a test metrics structure with some data
        let mut metrics = TypingMetrics::new();
        
        // Simulate some keystrokes with timing
        let test_keys = "abcdefghijklmnopqrstuvwxyz1234567890 ";
        for c in test_keys.chars() {
            for _ in 0..5 {  // 5 keystrokes per character for good data
                // Simulate different speeds based on character
                let speed = match c {
                    'a'..='f' => 120.0,  // Fast
                    'g'..='m' => 200.0,  // Medium
                    'n'..='t' => 300.0,  // Slow
                    _ => 400.0,          // Very slow
                };
                
                metrics.record_keystroke(c, c, metrics.keystrokes);
                
                // Directly update timing since we don't have actual events
                if let Some(char_metrics) = metrics.char_metrics.get_mut(&c) {
                    char_metrics.update(speed, true);
                }
                
                metrics.keystrokes += 1;
            }
        }
        
        // Create a buffer to capture the output
        let mut buffer = Vec::new();
        
        // Test the unified keyboard heatmap rendering
        let result = heatmap::draw_unified_keyboard_heatmap(&mut buffer, &metrics, 1);
        
        // Make sure rendering succeeds
        assert!(result.is_ok(), "Unified heatmap rendering failed: {:?}", result.err());
        
        // Basic validation: output should contain some data
        assert!(!buffer.is_empty(), "Unified heatmap rendering produced no output");
        
        // Sanity check: buffer should contain some ANSI escape sequences
        let output = String::from_utf8_lossy(&buffer);
        assert!(output.contains("\u{1b}["), "Output doesn't contain ANSI escape sequences");
    }
} 