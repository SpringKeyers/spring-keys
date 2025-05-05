mod core;
mod input;
mod ui;
mod config;
mod logger;
mod games;

use std::path::PathBuf;
use log::{info, LevelFilter};

pub use core::{TypingSession, TypingMetrics, TypingError};
pub use input::{InputProcessor, ValidationResult};
pub use core::state::{GameState, GameType, GameStatus};
pub use ui::TerminalUI;
pub use games::minesweeper::MinesweeperGame;

// Re-export commonly used types from dependencies
pub use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug)]
pub struct SpringKeys {
    pub game_state: GameState,
    pub input_processor: InputProcessor,
    pub typing_session: Option<TypingSession>,
    pub config: config::Config,
}

impl SpringKeys {
    pub fn new() -> Self {
        // Load configuration or create default
        let config_path = PathBuf::from(config::DEFAULT_CONFIG_FILE);
        let config = config::Config::load_or_default(config_path);

        info!("SpringKeys application initialized");
        info!("Game mode: {:?}", config.game.game_mode);
        
        Self {
            game_state: GameState::default(),
            input_processor: InputProcessor::new(),
            typing_session: None,
            config,
        }
    }

    pub fn start_typing_session(&mut self, text: String) {
        info!("Starting typing session with text: {}", text);
        self.typing_session = Some(TypingSession::new(text));
        self.input_processor.clear();
    }

    pub fn process_input(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        self.input_processor.process_key_event(key, modifiers);
        self.input_processor.process_queued_events();

        if let Some(session) = &mut self.typing_session {
            let result = self.input_processor.validate_input(&session.text);
            self.input_processor.update_error_state(result);
            session.calculate_metrics();
        }
    }

    pub fn change_game(&mut self, game_type: GameType) {
        info!("Changing game type to {:?}", game_type);
        self.game_state = GameState::new(game_type);
        self.typing_session = None;
        self.input_processor.clear();
    }
}

fn main() -> std::io::Result<()> {
    // Initialize logger
    let _ = logger::init_logger(LevelFilter::Info, None::<PathBuf>);
    
    info!("Starting SpringKeys application");
    
    let mut app = SpringKeys::new();
    let mut ui = TerminalUI::new()?;
    
    ui.init()?;
    
    let result = ui.run(&mut app);
    
    // Ensure terminal is restored even if an error occurs
    ui.cleanup()?;
    
    // Return any error from the run function
    result
} 