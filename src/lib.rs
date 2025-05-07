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
use std::collections::HashMap;

// Re-export commonly used types for convenience
pub use core::{TypingSession, TypingError};
pub use core::metrics::{TypingMetrics, CharacterMetrics, KeyboardRow, Finger};
pub use core::metrics::ExtendedStats;
pub use core::stats::AccumulatedStats;
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
    pub accumulated_stats: AccumulatedStats,
}

impl SpringKeys {
    pub fn new() -> Self {
        // Load configuration or create default
        let config_path = std::path::PathBuf::from(config::DEFAULT_CONFIG_FILE);
        let config = config::Config::load_or_default(config_path);
        
        // Load accumulated stats from the stats directory
        info!("Loading accumulated statistics from stats directory...");
        let accumulated_stats = AccumulatedStats::load_from_directory();
        info!("Loaded stats from {} quotes", accumulated_stats.total_quotes);
        
        Self {
            game_state: GameState::default(),
            input_processor: InputProcessor::new(),
            typing_session: None,
            config,
            quote_db: QuoteDatabase::new(),
            accumulated_stats,
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
    
    pub fn process_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> bool {
        // Pass the typing session as a mutable reference to the input processor
        let mut_session = self.typing_session.as_mut();
        self.input_processor.process_key_event(code, modifiers, mut_session);
        self.input_processor.process_queued_events();

        // Register key press for heatmap animation
        if let KeyCode::Char(c) = code {
            crate::ui::heatmap::register_key_press(c);
        }

        if let Some(session) = &mut self.typing_session {
            let result = self.input_processor.validate_input(&session.quote_text);
            
            // Check if this input resulted in an error
            if !result.is_valid {
                // Increment both session and total error counts immediately
                self.accumulated_stats.session_errors += 1;
                self.accumulated_stats.total_errors += 1;
            }
            
            self.input_processor.update_error_state(&result);
            session.calculate_metrics();

            // Start a new typing session if the current text matches the expected text
            if result.is_valid && self.input_processor.current_text.len() == session.quote_text.len() {
                // Update accumulated stats before starting new session
                self.accumulated_stats.update_from_session(session);
                self.start_typing_session(None);
            }
        }
        true
    }
    
    pub fn get_heat_map(&self) -> Option<HashMap<char, f64>> {
        self.typing_session.as_ref().map(|session| session.metrics.get_heat_map())
    }
    
    pub fn get_finger_performance(&self) -> Option<&HashMap<Finger, ExtendedStats>> {
        self.typing_session.as_ref().map(|session| session.metrics.finger_performance())
    }

    pub fn get_averages(&self) -> Option<(f64, f64)> {
        // If we have a current session, use its stats
        if let Some(session) = &self.typing_session {
            Some(session.get_averages())
        } else {
            // Otherwise use accumulated stats
            Some((self.accumulated_stats.avg_wpm, self.accumulated_stats.avg_accuracy))
        }
    }
    
    pub fn change_game(&mut self, game_type: GameType) {
        info!("Changing game type to {:?}", game_type);
        self.game_state = GameState::new(game_type);
        self.typing_session = None;
        self.input_processor.clear();
        // Keep accumulated stats when changing game type
    }
} 