#[cfg(test)]
mod tests {
    use spring_keys::{InputProcessor, TypingSession};
    

    #[test]
    fn test_input_processor_tokens() {
        // Create a new input processor
        let mut processor = InputProcessor::new();
        
        // Test basic token processing
        assert!(processor.process_token("a", None));
        assert!(processor.process_token("<space>", None));
        assert!(processor.process_token("b", None));
        
        // Check the resulting text
        assert_eq!(processor.current_text, "a b");
        
        // Test special tokens
        assert!(processor.process_token("<backspace>", None));
        assert_eq!(processor.current_text, "a ");
        
        // Test control sequences
        assert!(processor.process_token("<enter>", None));
        
        // Test token sequences
        let processed = processor.process_token_sequence("c d e", None);
        assert_eq!(processed, 3);
        assert_eq!(processor.current_text, "a cde");
    }
    
    #[test]
    fn test_exit_sequence_detection() {
        let processor = InputProcessor::new();
        
        // Test sequence with period + enter at the end
        assert!(processor.contains_exit_sequence("a b c . <enter>"));
        
        // Test sequence without period + enter
        assert!(!processor.contains_exit_sequence("a b c"));
        
        // Test sequence with just period
        assert!(!processor.contains_exit_sequence("a b c ."));
        
        // Test sequence with just enter
        assert!(!processor.contains_exit_sequence("a b c <enter>"));
        
        // Test sequence with period and enter separated
        assert!(!processor.contains_exit_sequence("a . b <enter>"));
    }
    
    #[test]
    fn test_full_typing_session() {
        // Create a simple session and processor
        let mut session = TypingSession::new("abc".to_string());
        let mut processor = InputProcessor::new();
        
        // Process input
        processor.process_token("a", Some(&mut session));
        processor.process_token("b", Some(&mut session));
        processor.process_token("c", Some(&mut session));
        
        // Validate results
        assert_eq!(processor.current_text, "abc");
        assert_eq!(session.current_position, 3);
        
        // Check that the quote is completed
        assert!(processor.is_quote_completed(&session.text));
    }
} 