mod core;
mod input;
mod ui;
mod config;
mod logger;
mod games;
mod quotes;
mod help;
mod vga_test;

use std::path::PathBuf;
use std::env;
use log::{info, LevelFilter};

pub use core::{TypingSession, TypingError};
pub use core::metrics::{TypingMetrics, CharacterMetrics, KeyboardRow, Finger};
pub use input::{InputProcessor, ValidationResult};
pub use core::state::{GameState, GameType, GameStatus};
pub use ui::TerminalUI;
pub use games::minesweeper::MinesweeperGame;
pub use quotes::{Quote, QuoteDatabase, QuoteDifficulty, QuoteCategory};

// Re-export commonly used types from dependencies
pub use crossterm::event::{KeyCode, KeyModifiers};

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
        let config_path = PathBuf::from(config::DEFAULT_CONFIG_FILE);
        let config = config::Config::load_or_default(config_path);

        info!("SpringKeys application initialized");
        info!("Game mode: {:?}", config.game.game_mode);
        
        Self {
            game_state: GameState::default(),
            input_processor: InputProcessor::new(),
            typing_session: None,
            config,
            quote_db: QuoteDatabase::new(),
        }
    }

    pub fn start_typing_session(&mut self, text: Option<String>) {
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
        
        info!("Starting typing session with text: {}", quote_text);
        self.typing_session = Some(TypingSession::new(quote_text));
        self.input_processor.clear();
    }

    pub fn process_input(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Pass the typing session as a mutable reference to the input processor
        let mut_session = self.typing_session.as_mut();
        self.input_processor.process_key_event(key, modifiers, mut_session);
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
    
    pub fn get_heat_map(&self) -> Option<std::collections::HashMap<char, f64>> {
        self.typing_session.as_ref().map(|session| session.metrics.generate_heat_map())
    }
    
    pub fn get_finger_performance(&self) -> Option<std::collections::HashMap<Finger, f64>> {
        self.typing_session.as_ref().map(|session| session.metrics.finger_performance())
    }
}

fn parse_difficulty(arg: &str) -> Option<QuoteDifficulty> {
    match arg.to_lowercase().as_str() {
        "easy" => Some(QuoteDifficulty::Easy),
        "medium" => Some(QuoteDifficulty::Medium),
        "hard" => Some(QuoteDifficulty::Hard),
        _ => None
    }
}

fn main() -> std::io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut difficulty = None;
    let mut quiet_mode = false;
    let mut command = None;
    
    // Process arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                help::print_help();
                return Ok(());
            },
            "-v" | "--version" => {
                println!("SpringKeys v{}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            },
            "-d" | "--difficulty" => {
                if i + 1 < args.len() {
                    difficulty = parse_difficulty(&args[i + 1]);
                    if difficulty.is_none() {
                        eprintln!("Invalid difficulty level. Use: easy, medium, or hard");
                        return Ok(());
                    }
                    i += 1;
                }
            },
            "-q" | "--quiet" => {
                quiet_mode = true;
            },
            "practice" | "stats" | "config" | "game" | "test" => {
                command = Some(args[i].clone());
            },
            _ => {
                if !args[i].starts_with('-') {
                    command = Some(args[i].clone());
                }
            }
        }
        i += 1;
    }
    
    // Set up logging based on quiet mode
    let log_level = if quiet_mode { LevelFilter::Error } else { LevelFilter::Info };
    let _ = logger::init_logger(log_level, None::<PathBuf>);
    
    info!("Starting SpringKeys application");
    
    // If no command is provided, run the VGA test screen
    if command.is_none() {
        return vga_test::run_test_screen();
    }
    
    // Initialize application
    let mut app = SpringKeys::new();
    
    // Apply difficulty if specified
    if let Some(diff) = difficulty {
        app.config.preferences.difficulty = match diff {
            QuoteDifficulty::Easy => config::DifficultyLevel::Beginner,
            QuoteDifficulty::Medium => config::DifficultyLevel::Intermediate,
            QuoteDifficulty::Hard => config::DifficultyLevel::Advanced,
        };
    }
    
    // Handle specific commands
    let cmd = command.unwrap();
    match cmd.as_str() {
        "practice" => {
            app.change_game(GameType::Practice);
        },
        "game" => {
            app.change_game(GameType::Minesweeper);
        },
        "stats" => {
            println!("Statistics viewing not yet implemented");
            return Ok(());
        },
        "config" => {
            println!("Configuration editing not yet implemented");
            return Ok(());
        },
        "test" => {
            return vga_test::run_test_screen();
        },
        _ => {
            eprintln!("Unknown command: {}", cmd);
            help::print_help();
            return Ok(());
        }
    }
    
    // Initialize and run the UI
    let mut ui = TerminalUI::new()?;
    ui.init()?;
    let result = ui.run(&mut app);
    ui.cleanup()?;
    
    result
} 