mod core;
mod input;
mod ui;
mod config;
mod logger;
mod quotes;
mod help;
mod vga_test;
mod moosesay;

use std::path::PathBuf;
use std::env;
use log::{info, LevelFilter};
use std::collections::HashMap;
use std::io::{self, IsTerminal};
use std::time::Duration;
use std::thread;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use crate::core::{TypingSession, TypingError};
use crate::core::metrics::{TypingMetrics, Finger, ExtendedStats};
use crate::core::state::{GameState, GameType, GameStatus};
use crate::core::stats::AccumulatedStats;
use crate::input::InputProcessor;
use crate::config::{Config, DifficultyLevel};
use crate::quotes::{Quote, QuoteDatabase, QuoteDifficulty, CategoryCycle};
use crate::ui::TerminalUI;

#[derive(Debug)]
pub struct SpringKeys {
    pub game_state: GameState,
    pub input_processor: InputProcessor,
    pub typing_session: Option<TypingSession>,
    pub config: Config,
    pub quote_db: QuoteDatabase,
    pub accumulated_stats: AccumulatedStats,
}

impl SpringKeys {
    pub fn new() -> Self {
        // Load configuration or create default
        let config_path = PathBuf::from(config::DEFAULT_CONFIG_FILE);
        let config = Config::load_or_default(config_path);
        
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

    pub fn start_typing_session(&mut self, text: Option<String>) {
        let quote_text = match text {
            Some(t) => t,
            None => {
                // Use a random quote based on user's difficulty setting
                let difficulty = match self.config.preferences.difficulty {
                    DifficultyLevel::Beginner => QuoteDifficulty::Easy,
                    DifficultyLevel::Intermediate => QuoteDifficulty::Medium,
                    DifficultyLevel::Advanced | DifficultyLevel::Expert => QuoteDifficulty::Hard,
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
        if let Some(session) = &mut self.typing_session {
            // If we already have a session, just load the new quote
            session.load_new_quote(quote_text);
        } else {
            // Create a new session if we don't have one
            self.typing_session = Some(TypingSession::new(quote_text));
        }
        self.input_processor.clear();
    }

    pub fn process_input(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Pass the typing session as a mutable reference to the input processor
        let mut_session = self.typing_session.as_mut();
        self.input_processor.process_key_event(key, modifiers, mut_session);
        self.input_processor.process_queued_events();

        if let Some(session) = &mut self.typing_session {
            let result = self.input_processor.validate_input(&session.quote_text);
            self.input_processor.update_error_state(&result);
            session.calculate_metrics();

            // Start a new typing session if the current text matches the expected text
            if result.is_valid && self.input_processor.current_text.len() == session.quote_text.len() {
                // Update accumulated stats before starting new session
                self.accumulated_stats.update_from_session(session);
                self.start_typing_session(None);
            }
        }
    }

    pub fn change_game(&mut self, game_type: GameType) {
        info!("Changing game type to {:?}", game_type);
        self.game_state = GameState::new(game_type);
        self.typing_session = None;
        self.input_processor.clear();
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
}

fn parse_difficulty(arg: &str) -> Option<QuoteDifficulty> {
    match arg.to_lowercase().as_str() {
        "easy" => Some(QuoteDifficulty::Easy),
        "medium" => Some(QuoteDifficulty::Medium),
        "hard" => Some(QuoteDifficulty::Hard),
        _ => None
    }
}

fn run_consume_mode(app: &mut SpringKeys, input_sequence: Option<&str>) -> io::Result<()> {
    // Set demo heatmap to ensure the visualization works
    std::env::set_var("SPRING_KEYS_DEMO_HEATMAP", "1");
    
    // Initialize and run the UI
    let mut ui = TerminalUI::new()?;
    ui.init()?;
    
    // Process input sequence if provided
    if let Some(input_text) = input_sequence {
        info!("Processing input sequence in consume mode: {}", input_text);
        
        // Start a new typing session with the input text
        app.start_typing_session(Some(input_text.to_string()));
        
        // Process input
        for c in input_text.chars() {
            if let Some(_) = &mut app.typing_session {
                // Process the character
                app.process_input(KeyCode::Char(c), KeyModifiers::NONE);
            }
        }
    }

    // Main consume-mode loop (quit on ESC)
    while !ui.should_quit() {
        ui.render_frame(app)?;
        // Check for ESC key to exit
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

    ui.cleanup()
}

fn main() -> std::io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut difficulty = None;
    let mut quiet_mode = false;
    let mut verbose_mode = false;
    let mut command = None;
    let mut demo_heatmap = true;  // Default to demo heatmap enabled for better visual experience
    let mut consume_input = None; // Input for consume mode
    let mut force_non_interactive = false; // New flag for non-interactive mode
    let mut duration = None; // Duration for screensaver mode
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--" => {
                force_non_interactive = true;
                // Enable verbose mode if screensaver command is used
                if args.len() > i + 1 && args[i + 1] == "screensaver" {
                    verbose_mode = true;
                }
                i += 1;
                continue;
            },
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
            "--verbose" => {
                verbose_mode = true;
            },
            "--demo-heatmap" => {
                demo_heatmap = true;
            },
            "--no-demo" => {
                demo_heatmap = false;
            },
            "practice" | "config" | "test" | "consume" | "quote" | "moosesay" | "screensaver" => {
                command = Some(args[i].clone());
                
                // If this is consume mode and the next arg doesn't start with '-'
                if args[i].as_str() == "consume" && i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    consume_input = Some(args[i + 1].clone());
                    i += 1;
                }
                // If this is screensaver mode and the next arg is a number
                else if args[i].as_str() == "screensaver" && i + 1 < args.len() {
                    if let Ok(dur) = args[i + 1].parse::<u64>() {
                        duration = Some(dur);
                        i += 1;
                    }
                }
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
    
    if !quiet_mode {
        info!("Starting SpringKeys application");
    }
    
    // Initialize application
    let mut app = SpringKeys::new();

    // Create a single instance of QuoteDatabase
    let mut quote_db = if quiet_mode {
        quotes::QuoteDatabase::new_silent()
    } else {
        quotes::QuoteDatabase::new()
    };

    // Apply difficulty if specified
    if let Some(diff) = difficulty {
        app.config.preferences.difficulty = match diff {
            QuoteDifficulty::Easy => DifficultyLevel::Beginner,
            QuoteDifficulty::Medium => DifficultyLevel::Intermediate,
            QuoteDifficulty::Hard => DifficultyLevel::Advanced,
        };
    }

    // Handle special commands that don't need the full app initialization
    if let Some(cmd) = &command {
        match cmd.as_str() {
            "quote" | "moosesay" | "screensaver" => {
                let mut quote_db = if quiet_mode {
                    quotes::QuoteDatabase::new_silent()
                } else {
                    quotes::QuoteDatabase::new()
                };
                
                match cmd.as_str() {
                    "quote" => {
                        let quote = quote_db.next_random();
                        if !quiet_mode {
                            println!("{}", quote.text);
                            println!("— {}", quote.source);
                        }
                    }
                    "moosesay" => {
                        // Skip animation in non-interactive mode or quiet mode
                        if force_non_interactive || !std::io::stdout().is_terminal() || quiet_mode {
                            let quote = quote_db.next_random();
                            if !quiet_mode {
                                println!("{}", quote.text);
                                println!("— {}", quote.source);
                            }
                        } else {
                            moosesay::animate_moose_quote(1, quiet_mode, verbose_mode)?;
                        }
                    }
                    "screensaver" => {
                        // Skip animation in non-interactive mode or quiet mode
                        if !std::io::stdout().is_terminal() || quiet_mode {
                            let quote = quote_db.next_random();
                            if !quiet_mode {
                                println!("{}", quote.text);
                                println!("— {}", quote.source);
                            }
                        } else {
                            let duration = duration.unwrap_or(1);
                            moosesay::animate_moose_quote(duration, quiet_mode, verbose_mode)?;
                        }
                    }
                    _ => unreachable!()
                }
                return Ok(());
            }
            "practice" => {
                app.change_game(GameType::Practice);
                // Start a typing session to show the keyboard immediately
                app.start_typing_session(None);
            },
            "config" => {
                println!("Configuration editing not yet implemented");
                return Ok(());
            },
            "test" => {
                return vga_test::run_test_screen();
            },
            "consume" => {
                app.change_game(GameType::Consume);
                return run_consume_mode(&mut app, consume_input.as_deref());
            },
            _ => {
                // If no terminal is detected, default to practice mode instead of showing error
                if !std::io::stdout().is_terminal() {
                    app.change_game(GameType::Consume);
                } else {
                    if quiet_mode {
                        help::print_help_quiet();
                    } else {
                        help::print_help();
                    }
                    return Ok(());
                }
            }
        }
    } else {
        // If no command is provided, show help
        if quiet_mode {
            help::print_help_quiet();
        } else {
            help::print_help();
        }
        return Ok(());
    }

    // Don't try to initialize UI in non-interactive mode
    if force_non_interactive || !std::io::stdout().is_terminal() {
        return Ok(());
    }

    // Set environment variable if demo heatmap is enabled
    if demo_heatmap {
        info!("Setting SPRING_KEYS_DEMO_HEATMAP=1 for colored keyboard visualization");
        std::env::set_var("SPRING_KEYS_DEMO_HEATMAP", "1");
    }
    
    // Initialize and run the UI
    let mut ui = TerminalUI::new()?;
    ui.init()?;
    
    let result = ui.run(&mut app);
    ui.cleanup()?;
    
    result
} 