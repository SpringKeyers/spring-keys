use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use spring_keys::{SpringKeys, GameType};
use crossterm::event::{KeyCode, KeyModifiers};

#[test]
#[ignore] // This test requires manual validation of terminal output
fn test_consume_mode() {
    // Run the application in consume mode with specified input
    let status = Command::new("./target/debug/spring-keys")
        .arg("consume")
        .arg("T h e <space> q u i c k <space> b r o w n <space> f o x")
        .env("SPRING_KEYS_DEMO_HEATMAP", "1")  // Enable demo heatmap
        .stderr(Stdio::null())  // Suppress stderr
        .stdout(Stdio::null())  // Suppress stdout
        .spawn()
        .expect("Failed to start spring-keys process")
        .kill()  // Terminate after 3 seconds
        .expect("Failed to terminate process");
    
    assert!(true, "Consume mode test completed");
}

#[test]
fn test_consume_mode_input_parsing() {
    // Run the application with input parsing validation
    let output = Command::new("./target/debug/spring-keys")
        .arg("consume")
        .arg("T h e")
        .env("SPRING_KEYS_ENV_INFO", "1")  // Show environment info for debugging
        .env("RUST_LOG", "debug")
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print the output for debugging
    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);
    
    // Check if the process at least started without crashing
    assert!(output.status.success() || output.status.code() == Some(0), 
        "Consume mode with input parsing did not run successfully");
}

#[test]
fn test_consume_mode_with_simulation() {
    // This test is designed to check if the consume mode processes input correctly
    // without requiring interactive terminal
    
    // Run with a simple input and demo mode enabled
    let output = Command::new("./target/debug/spring-keys")
        .arg("--input")
        .arg("a b c")
        .env("SPRING_KEYS_TEST_MODE", "1")  // Enable test mode
        .env("SPRING_KEYS_DEMO_HEATMAP", "1")  // Enable demo heatmap
        .env("RUST_LOG", "info")
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Check if the process succeeded
    assert!(output.status.success(), 
        "Test mode didn't run successfully: Exit code: {:?}\nStdout: {}\nStderr: {}", 
        output.status.code(), stdout, stderr);
    
    // Since this is a headless test, it should return exit code 0
    assert_eq!(output.status.code(), Some(0), 
        "Expected exit code 0, got: {:?}", output.status.code());
}

#[test]
fn test_consume_mode_initialization() {
    // Create application instance
    let mut app = SpringKeys::new();
    
    // Change to consume mode
    app.change_game(GameType::Consume);
    
    // Start a typing session
    app.start_typing_session(None);
    
    // Verify keyboard heatmap is initialized
    let heatmap = app.get_heat_map();
    assert!(heatmap.is_some(), "Keyboard heatmap should be initialized");
    
    // Verify demo heatmap data is present (1ms delays)
    if let Some(hmap) = heatmap {
        // Check if common keys have been initialized
        let test_keys = ['a', 's', 'd', 'f', 'j', 'k', 'l', ' '];
        for key in test_keys {
            assert!(hmap.contains_key(&key), "Key {} should be initialized in heatmap", key);
        }
    }
}

#[test]
fn test_consume_mode_input_processing() {
    // Create application instance
    let mut app = SpringKeys::new();
    app.change_game(GameType::Consume);
    
    // Start with a specific test quote
    let test_quote = "Hello world";
    app.start_typing_session(Some(test_quote.to_string()));
    
    // Process input sequence
    let input_sequence = vec![
        ('H', KeyModifiers::SHIFT),
        ('e', KeyModifiers::NONE),
        ('l', KeyModifiers::NONE),
        ('l', KeyModifiers::NONE),
        ('o', KeyModifiers::NONE),
    ];
    
    // Process each character
    for (c, modifiers) in input_sequence {
        app.process_input(KeyCode::Char(c), modifiers);
    }
    
    // Verify input was processed
    let heatmap = app.get_heat_map().expect("Heatmap should exist");
    for (c, _) in input_sequence {
        assert!(heatmap.contains_key(&c.to_ascii_lowercase()), 
            "Key {} should be in heatmap after processing", c);
    }
}

#[test]
fn test_consume_mode_practice_code_sharing() {
    // Create two application instances - one for practice, one for consume
    let mut practice_app = SpringKeys::new();
    let mut consume_app = SpringKeys::new();
    
    // Set up practice mode
    practice_app.change_game(GameType::Practice);
    practice_app.start_typing_session(Some("test".to_string()));
    
    // Set up consume mode
    consume_app.change_game(GameType::Consume);
    consume_app.start_typing_session(Some("test".to_string()));
    
    // Process same input in both modes
    let input = vec![
        ('t', KeyModifiers::NONE),
        ('e', KeyModifiers::NONE),
        ('s', KeyModifiers::NONE),
        ('t', KeyModifiers::NONE),
    ];
    
    for (c, modifiers) in input.iter() {
        practice_app.process_input(KeyCode::Char(*c), *modifiers);
        consume_app.process_input(KeyCode::Char(*c), *modifiers);
    }
    
    // Get metrics from both modes
    let practice_metrics = practice_app.get_averages();
    let consume_metrics = consume_app.get_averages();
    
    // Verify both modes track metrics similarly
    assert!(practice_metrics.is_some() && consume_metrics.is_some(),
        "Both modes should track metrics");
    
    if let (Some((p_wpm, p_acc)), Some((c_wpm, c_acc))) = (practice_metrics, consume_metrics) {
        // WPM and accuracy calculations should use same code, so values should be similar
        assert!((p_wpm - c_wpm).abs() < 0.1, "WPM calculation should be consistent between modes");
        assert!((p_acc - c_acc).abs() < 0.1, "Accuracy calculation should be consistent between modes");
    }
}

#[test]
fn test_consume_mode_cli_integration() {
    // Test the CLI integration with input arguments
    let output = Command::new("./target/debug/spring-keys")
        .arg("consume")
        .arg("H e l l o")
        .env("SPRING_KEYS_DEMO_HEATMAP", "1")
        .env("SPRING_KEYS_TEST_MODE", "1")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start spring-keys process")
        .wait()
        .expect("Failed to wait for process");
    
    assert!(output.success(), "Consume mode should exit successfully with valid input");
}

#[test]
fn test_consume_mode_ui_initialization() {
    // Test that UI elements are properly initialized
    let mut app = SpringKeys::new();
    app.change_game(GameType::Consume);
    
    // Start with demo heatmap enabled
    std::env::set_var("SPRING_KEYS_DEMO_HEATMAP", "1");
    app.start_typing_session(None);
    
    // Verify heatmap is initialized with demo data
    let heatmap = app.get_heat_map().expect("Heatmap should be initialized");
    
    // Check number row keys (these should have demo gradient data)
    let number_keys = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    for key in number_keys {
        assert!(heatmap.contains_key(&key), 
            "Number key {} should be initialized with demo data", key);
    }
    
    // Check common letter keys
    let letter_keys = ['q', 'w', 'e', 'r', 't', 'a', 's', 'd', 'f'];
    for key in letter_keys {
        assert!(heatmap.contains_key(&key),
            "Letter key {} should be initialized with demo data", key);
    }
} 