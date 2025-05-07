use spring_keys::{SpringKeys, TypingSession, TypingMetrics};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_save_and_load_stats() {
    // Create a temporary directory for test stats
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let stats_dir = temp_dir.path().join("stats");
    fs::create_dir_all(&stats_dir).expect("Failed to create stats directory");

    // Create a mock typing session with known data
    let mut session = TypingSession::new("test quote".to_string());
    session.metrics.keystrokes = 100;
    session.metrics.correct_keystrokes = 95;
    session.metrics.wpm = 60.0;
    session.metrics.accuracy = 95.0;

    // Save the stats
    session.metrics.save_to_json("test quote").expect("Failed to save stats");

    // Create a new SpringKeys instance and load the stats
    let app = SpringKeys::new();
    
    // Verify the loaded stats match what we saved
    assert_eq!(app.accumulated_stats.total_keystrokes, 100);
    assert_eq!(app.accumulated_stats.total_correct_keystrokes, 95);
    assert!((app.accumulated_stats.avg_wpm - 60.0).abs() < 0.001);
    assert!((app.accumulated_stats.avg_accuracy - 95.0).abs() < 0.001);
}

#[test]
fn test_multiple_sessions_accumulation() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let stats_dir = temp_dir.path().join("stats");
    fs::create_dir_all(&stats_dir).expect("Failed to create stats directory");

    // Create and save multiple sessions
    let sessions = vec![
        ("First quote", 50.0, 90.0),
        ("Second quote", 60.0, 95.0),
        ("Third quote", 70.0, 92.0),
    ];

    for (quote, wpm, accuracy) in sessions {
        let mut session = TypingSession::new(quote.to_string());
        session.metrics.wpm = wpm;
        session.metrics.accuracy = accuracy;
        session.metrics.save_to_json(quote).expect("Failed to save stats");
    }

    // Load accumulated stats
    let app = SpringKeys::new();
    
    // Verify accumulated averages
    assert!((app.accumulated_stats.avg_wpm - 60.0).abs() < 0.001); // Average of 50, 60, 70
    assert!((app.accumulated_stats.avg_accuracy - 92.33).abs() < 0.1); // Average of 90, 95, 92
    assert_eq!(app.accumulated_stats.total_quotes, 3);
}

#[test]
fn test_invalid_stats_handling() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let stats_dir = temp_dir.path().join("stats");
    fs::create_dir_all(&stats_dir).expect("Failed to create stats directory");

    // Create an invalid stats file
    let invalid_stats = r#"{ "invalid": "json" }"#;
    fs::write(
        stats_dir.join("invalid_stats.json"),
        invalid_stats
    ).expect("Failed to write invalid stats");

    // Create a valid stats file
    let mut session = TypingSession::new("valid quote".to_string());
    session.metrics.wpm = 55.0;
    session.metrics.accuracy = 91.0;
    session.metrics.save_to_json("valid quote").expect("Failed to save stats");

    // Load stats - should skip invalid file and load valid one
    let app = SpringKeys::new();
    
    assert_eq!(app.accumulated_stats.total_quotes, 1);
    assert!((app.accumulated_stats.avg_wpm - 55.0).abs() < 0.001);
    assert!((app.accumulated_stats.avg_accuracy - 91.0).abs() < 0.001);
} 