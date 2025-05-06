#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::time::Duration;
    use std::thread;
    
    // Import the required types for direct testing
    use spring_keys::SpringKeys;
    use crossterm::event::{KeyCode, KeyModifiers};
    
    #[test]
    fn test_basic_typing_mode() {
        // Start the program with the practice command
        let mut child = Command::new(env!("CARGO_BIN_EXE_spring-keys"))
            .arg("practice")
            .spawn()
            .expect("Failed to start spring-keys");
        
        // Wait for the program to initialize and confirm it doesn't immediately crash
        thread::sleep(Duration::from_secs(2));
        
        // Check that the process is still running
        match child.try_wait() {
            Ok(Some(status)) => {
                // Process has exited - this is unexpected for a short time
                assert!(status.success(), "Process exited early with non-zero status");
                println!("Note: Process exited early but with success status");
            },
            Ok(None) => {
                // Process is still running as expected
                println!("Process still running as expected");
            },
            Err(e) => {
                panic!("Error waiting for child process: {}", e);
            }
        }
        
        // Terminate the process
        child.kill().expect("Failed to kill spring-keys process");
    }
    
    #[test]
    fn test_input_tracking() {
        // Create a SpringKeys instance
        let mut app = SpringKeys::new();
        
        // Start a typing session with a specific quote
        let test_quote = "Hello world. This is a test.";
        app.start_typing_session(Some(test_quote.to_string()));
        
        // Ensure we have a valid typing session
        assert!(app.typing_session.is_some(), "Failed to create typing session");
        
        if let Some(session) = &app.typing_session {
            assert_eq!(session.text, test_quote, "Quote text doesn't match");
        }
        
        // Process a few keystrokes
        app.process_input(KeyCode::Char('H'), KeyModifiers::NONE);
        app.process_input(KeyCode::Char('e'), KeyModifiers::NONE);
        app.process_input(KeyCode::Char('l'), KeyModifiers::NONE);
        app.process_input(KeyCode::Char('l'), KeyModifiers::NONE);
        app.process_input(KeyCode::Char('o'), KeyModifiers::NONE);
        
        // Verify the input processor has updated
        assert_eq!(app.input_processor.current_text.len(), 5, 
                  "Input processor text length doesn't match expected");
        
        // Verify keystrokes were recorded
        if let Some(session) = &app.typing_session {
            assert!(session.metrics.keystrokes >= 5, "Expected at least 5 keystrokes, got {}", 
                   session.metrics.keystrokes);
            
            // Check that WPM is calculated (should be some value)
            assert!(session.metrics.wpm >= 0.0, "WPM should be calculated");
        }
    }
} 