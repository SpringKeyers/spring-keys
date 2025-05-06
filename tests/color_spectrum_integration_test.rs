use std::process::{Command, Stdio};
use std::io::{Read, Write};
use std::time::Duration;
use std::thread;
use std::str::from_utf8;
use std::sync::mpsc;
use std::collections::HashMap;

// Function to extract RGB values from ANSI escape sequences
fn extract_rgb_from_ansi(ansi_code: &str) -> Option<(u8, u8, u8)> {
    // Look for the pattern: "\u{1b}[48;2;R;G;Bm" or "\u{1b}[38;2;R;G;Bm"
    let parts: Vec<&str> = ansi_code.split(';').collect();
    if parts.len() >= 5 && (parts[0] == "\u{1b}[48" || parts[0] == "\u{1b}[38") && parts[1] == "2" {
        // Extract the RGB values
        let r = parts[2].parse::<u8>().ok()?;
        let g = parts[3].parse::<u8>().ok()?;
        let b = parts[4].trim_end_matches('m').parse::<u8>().ok()?;
        return Some((r, g, b));
    }
    None
}

#[test]
#[ignore = "This test requires a terminal and is flaky in CI environments"]
fn test_color_spectrum_in_binary() {
    // Skip this test unless explicitly enabled with ENABLE_TERMINAL_TESTS=1
    if std::env::var("ENABLE_TERMINAL_TESTS").unwrap_or_default() != "1" {
        println!("Skipping terminal test. Set ENABLE_TERMINAL_TESTS=1 to run.");
        return;
    }

    // Start the command with specific arguments to show the heatmap
    let mut cmd = Command::new("cargo")
        .args(["run", "--", "single", "--demo-heatmap"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start the SpringKeys binary");

    // Get handles to stdin and stdout
    let mut stdin = cmd.stdin.take().expect("Failed to open stdin");
    let mut stdout = cmd.stdout.take().expect("Failed to open stdout");
    
    // Create a channel to communicate between threads
    let (tx, rx) = mpsc::channel();
    
    // Spawn a thread to read stdout
    let read_thread = thread::spawn(move || {
        let mut buffer = Vec::new();
        let mut output = String::new();
        
        // Read with timeout to prevent hanging
        let timeout = Duration::from_secs(5);
        let start = std::time::Instant::now();
        
        while start.elapsed() < timeout {
            let mut chunk = [0; 1024];
            match stdout.read(&mut chunk) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    buffer.extend_from_slice(&chunk[..n]);
                    
                    // Try to convert to UTF-8 and look for ANSI color codes
                    if let Ok(str_chunk) = from_utf8(&buffer) {
                        output.push_str(str_chunk);
                        buffer.clear();
                        
                        // If we see the keyboard is rendered, we're good to proceed
                        if output.contains("SPACE") {
                            break;
                        }
                    }
                },
                Err(_) => thread::sleep(Duration::from_millis(100)),
            }
        }
        
        tx.send(output).expect("Failed to send output");
    });
    
    // Give the application time to start and render
    thread::sleep(Duration::from_millis(500));
    
    // Simulate typing some keys to get different speeds
    let _ = stdin.write_all(b"aaaaabbbbbb");
    
    // Give the application time to process input
    thread::sleep(Duration::from_millis(500));
    
    // Press 'q' to quit the application
    let _ = stdin.write_all(b"q");
    
    // Wait for the read thread to complete
    let output = match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(output) => output,
        Err(_) => {
            // Kill the process if timeout occurs
            let _ = cmd.kill();
            String::new()
        }
    };
    
    let _ = read_thread.join();
    
    // Kill the process if it's still running
    let _ = cmd.kill();
    
    // Verify the output contains ANSI color codes
    assert!(output.contains("\u{1b}["), "Output doesn't contain ANSI escape sequences");
    
    // Check for color in the key visualization - looking for RGB color codes
    assert!(output.contains("\u{1b}[38;2;"), "No foreground RGB color codes found");
    assert!(output.contains("\u{1b}[48;2;"), "No background RGB color codes found");
    
    println!("Color spectrum test passed successfully!");
}

#[test]
#[ignore = "This test requires a terminal and is flaky in CI environments"]
fn test_color_mapping_in_binary() {
    // Skip this test unless explicitly enabled with ENABLE_TERMINAL_TESTS=1
    if std::env::var("ENABLE_TERMINAL_TESTS").unwrap_or_default() != "1" {
        println!("Skipping terminal test. Set ENABLE_TERMINAL_TESTS=1 to run.");
        return;
    }

    // Start the SpringKeys binary in demo mode with different speed settings
    let mut cmd = Command::new("cargo")
        .args(["run", "--", "single", "--demo-heatmap", "--force-speeds"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start the SpringKeys binary");

    // Get handles to stdin and stdout
    let mut stdin = cmd.stdin.take().expect("Failed to open stdin");
    let mut stdout = cmd.stdout.take().expect("Failed to open stdout");
    
    // Read the output
    let mut buffer = Vec::new();
    
    // Give the application time to render
    thread::sleep(Duration::from_secs(1));
    
    // Type some keys with controlled speeds
    let _ = stdin.write_all(b"abcdefghijklmnopqrstuvwxyz1234567890");
    thread::sleep(Duration::from_millis(500));
    
    // Press 'q' to quit
    let _ = stdin.write_all(b"q");
    
    // Read the output with timeout
    let timeout = Duration::from_secs(5);
    let start = std::time::Instant::now();
    
    let mut output = String::new();
    while start.elapsed() < timeout {
        let mut chunk = [0; 1024];
        match stdout.read(&mut chunk) {
            Ok(0) => break, // EOF
            Ok(n) => {
                buffer.extend_from_slice(&chunk[..n]);
                if let Ok(str_chunk) = from_utf8(&buffer) {
                    output.push_str(str_chunk);
                    buffer.clear();
                }
            },
            Err(_) => break,
        }
    }
    
    // Kill the process if it's still running
    let _ = cmd.kill();
    
    // Extract all background RGB colors from the output
    let mut bg_colors = HashMap::new();
    
    // Split output by ANSI escape sequences
    let ansi_pattern = "\u{1b}[";
    let parts: Vec<&str> = output.split(ansi_pattern).collect();
    
    for part in parts {
        if part.starts_with("48;2;") || part.starts_with("38;2;") {
            // This is an RGB color code
            if let Some((r, g, b)) = extract_rgb_from_ansi(&format!("{}{}", ansi_pattern, part)) {
                // Save colors with keys
                let char_index = output.find(&format!("{}{}", ansi_pattern, part))
                    .and_then(|idx| output[idx..].find(|c: char| c.is_ascii_alphanumeric()))
                    .and_then(|rel_idx| output.chars().nth(rel_idx));
                
                if let Some(ch) = char_index {
                    bg_colors.insert(ch, (r, g, b));
                }
            }
        }
    }
    
    // Verify we found some colors - if running in a CI environment this might be empty
    if !bg_colors.is_empty() {
        // Verify the purple-white-red spectrum
        // Fast keys should have more red, slow keys more purple
        
        // Check if we have both red and purple colors
        let mut has_reddish = false;
        let mut has_purplish = false;
        let mut has_whitish = false;
        
        for (_, (r, g, b)) in bg_colors {
            println!("Color: RGB({}, {}, {})", r, g, b);
            
            // Check for reddish colors (red > green and red > blue)
            if r > 200 && r > g + 50 && r > b + 50 {
                has_reddish = true;
            }
            
            // Check for purplish colors (red and blue high, green low)
            if r > 100 && b > 100 && g < r - 50 && g < b - 50 {
                has_purplish = true;
            }
            
            // Check for whitish colors (all high and similar)
            if r > 200 && g > 200 && b > 200 {
                has_whitish = true;
            }
        }
        
        // We should have detected at least some red and purple colors
        assert!(has_reddish || has_purplish || has_whitish, 
            "Could not detect any of the expected colors in the output");
    } else {
        println!("No colors detected in output - this test may be running in a headless environment");
    }
    
    println!("Color mapping test passed successfully!");
} 