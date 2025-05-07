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
        let result = processor.validate_input(&session.quote_text);
        assert!(result.is_valid);
        assert_eq!(processor.current_text.len(), session.quote_text.len());
    }
    
    #[test]
    fn test_quote_completion() {
        let mut processor = InputProcessor::new();
        let quote = "Hello world";
        let mut session = TypingSession::new(quote.to_string());
        
        // Process the exact quote
        processor.process_token_sequence("H e l l o <space> w o r l d", Some(&mut session));
        println!("Full quote - Current text: '{}', Expected: '{}'", processor.current_text, session.quote_text);
        
        // Validate completion
        let result = processor.validate_input(&session.quote_text);
        assert!(result.is_valid);
        assert_eq!(processor.current_text.len(), session.quote_text.len());
        
        // Test partial input
        processor.clear();
        processor.current_text = "Hel".to_string();
        println!("Partial input - Current text: '{}', Expected: '{}'", processor.current_text, session.quote_text);
        
        let result = processor.validate_input(&session.quote_text);
        assert!(result.is_valid);
        assert!(processor.current_text.len() < session.quote_text.len());
        
        // Test incorrect input
        processor.clear();
        processor.current_text = "Helo".to_string();
        println!("Incorrect input - Current text: '{}', Expected: '{}'", processor.current_text, session.quote_text);
        
        let result = processor.validate_input(&session.quote_text);
        assert!(!result.is_valid);
    }
} 