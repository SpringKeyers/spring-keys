use spring_keys::{SpringKeys, GameType};
use std::time::Duration;
use std::thread;
use std::process::Command;

#[test]
fn test_heatmap_verification() {
    // Create application instance
    let mut app = SpringKeys::new();
    
    // Change to consume mode which is good for automated input
    app.change_game(GameType::Consume);
    
    // Start with a specific test quote
    let test_quote = "hello";
    app.start_typing_session(Some(test_quote.to_string()));
    
    // Process each character with position
    for (i, c) in test_quote.chars().enumerate() {
        if let Some(session) = &mut app.typing_session {
            session.metrics.record_keystroke(c, c, i);
            session.calculate_metrics();
        }
        thread::sleep(Duration::from_millis(100));
    }
    
    // Get and verify the heatmap
    if let Some(heatmap) = app.get_heat_map() {
        println!("\nHeatmap after typing '{}':", test_quote);
        
        // Count character occurrences in the test quote
        let mut expected_counts = std::collections::HashMap::new();
        for c in test_quote.chars() {
            *expected_counts.entry(c).or_insert(0) += 1;
        }
        
        // Verify each character in the heatmap
        for (c, count) in expected_counts {
            if let Some(&(speed, actual_count)) = heatmap.get(&c) {
                println!("Key: {}, Speed: {:.1}ms, Count: {}", c, speed, actual_count);
                assert!(speed >= 0.0, "Speed for key '{}' should be >= 0", c);
                assert_eq!(actual_count, count, 
                    "Character '{}' count mismatch. Expected: {}, Got: {}", 
                    c, count, actual_count);
            } else {
                panic!("Character '{}' not found in heatmap", c);
            }
        }
    } else {
        panic!("No heatmap data available");
    }
}

#[test]
fn test_consume_mode_display() {
    // Launch the application in consume mode
    let mut child = Command::new(env!("CARGO_BIN_EXE_spring-keys"))
        .arg("consume")
        .arg("hello") // Input sequence
        .env("SPRING_KEYS_DEMO_HEATMAP", "1") // Enable heatmap visualization
        .spawn()
        .expect("Failed to start spring-keys");
    
    // Give it time to process and display
    thread::sleep(Duration::from_secs(3));
    println!("Visual verification: Check terminal for heatmap display");
    
    // Clean up
    child.kill().expect("Failed to kill spring-keys process");
} 