// Export modules for testing and library use
pub mod core;
pub mod input;
pub mod ui;
pub mod config;
pub mod logger;
pub mod quotes;
pub mod help;
pub mod vga_test;

// Import required crates
use log::info;

// Re-export commonly used types for convenience
pub use core::{TypingSession, TypingError};
pub use core::metrics::{TypingMetrics, CharacterMetrics, KeyboardRow, Finger};
pub use core::metrics::ExtendedStats;
pub use input::{InputProcessor, ValidationResult};
pub use core::state::{GameState, GameType, GameStatus};
pub use ui::TerminalUI;
pub use quotes::{Quote, QuoteDatabase, QuoteDifficulty, QuoteCategory};
pub use ui::color_spectrum;

// Re-export commonly used types from dependencies
pub use crossterm::event::{KeyCode, KeyModifiers};

// Define SpringKeys struct for testing
#[derive(Debug)]
pub struct SpringKeys {
    pub game_state: GameState,
    pub input_processor: InputProcessor,
    pub typing_session: Option<TypingSession>,
    pub config: config::Config,
    pub quote_db: QuoteDatabase,
}

impl SpringKeys {
    pub fn new() -> Self {
        // Load configuration or create default
        let config_path = std::path::PathBuf::from(config::DEFAULT_CONFIG_FILE);
        let config = config::Config::load_or_default(config_path);
        
        Self {
            game_state: GameState::default(),
            input_processor: InputProcessor::new(),
            typing_session: None,
            config,
            quote_db: QuoteDatabase::new(),
        }
    }

    /// Load a new quote and prepare the session for typing
    fn load_quote(&mut self, text: Option<String>) {
        let quote_text = match text {
            Some(t) => t,
            None => {
                // Use a random quote based on user's difficulty setting
                let difficulty = match self.config.preferences.difficulty {
                    config::DifficultyLevel::Beginner => QuoteDifficulty::Easy,
                    config::DifficultyLevel::Intermediate => QuoteDifficulty::Medium,
                    config::DifficultyLevel::Advanced | config::DifficultyLevel::Expert => QuoteDifficulty::Hard,
                };
                
                if let Some(quote) = self.quote_db.next_by_difficulty(difficulty) {
                    info!("Selected quote: \"{}\" ({})", quote.text, quote.source);
                    quote.text.clone()
                } else {
                    // Fallback to a random quote if no quote for the specific difficulty
                    let quote = self.quote_db.next_random();
                    info!("Selected random quote: \"{}\" ({})", quote.text, quote.source);
                    quote.text.clone()
                }
            }
        };
        
        info!("Loading new quote: {}", quote_text);
        
        // Clear input processor state
        self.input_processor.clear();
        
        // Create new session or reset existing one
        if let Some(session) = &mut self.typing_session {
            session.load_new_quote(quote_text);
        } else {
            self.typing_session = Some(TypingSession::new(quote_text));
        }
    }
    
    pub fn start_typing_session(&mut self, text: Option<String>) {
        self.load_quote(text);

        // Initialize demo data for consume mode if needed
        if let Some(session) = &mut self.typing_session {
            if self.game_state.current_game == GameType::Consume {
                session.metrics.simulate_demo_data();
            }
        }
    }
    
    pub fn process_input(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Pass the typing session as a mutable reference to the input processor
        let mut_session = self.typing_session.as_mut();
        self.input_processor.process_key_event(key, modifiers, mut_session);
        self.input_processor.process_queued_events();

        if let Some(session) = &mut self.typing_session {
            let result = self.input_processor.validate_input(&session.text);
            self.input_processor.update_error_state(&result);
            session.calculate_metrics();
        }
    }
    
    pub fn get_heat_map(&self) -> Option<std::collections::HashMap<char, (f64, u64)>> {
        self.typing_session.as_ref().map(|session| session.metrics.generate_heat_map())
    }
    
    pub fn get_averages(&self) -> Option<(f64, f64)> {
        self.typing_session.as_ref().map(|session| session.get_averages())
    }
    
    pub fn change_game(&mut self, game_type: GameType) {
        self.game_state = GameState::new(game_type);
        self.typing_session = None;
        self.input_processor.clear();
    }
} 