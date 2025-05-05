pub mod minesweeper;

use crate::core::state::GameType;

pub trait Game {
    /// Initialize a new game
    fn new() -> Self;
    
    /// Get the game type
    fn game_type(&self) -> GameType;
    
    /// Process input text
    fn process_input(&mut self, input: &str) -> GameResult;
    
    /// Get the current game status
    fn status(&self) -> GameStatus;
    
    /// Get the current score
    fn score(&self) -> i32;
}

/// Result of a game input
#[derive(Debug, Clone)]
pub enum GameResult {
    /// Valid input, game continues
    Continue,
    /// Valid input, player won
    Win,
    /// Valid input, player lost
    Lose,
    /// Invalid input
    Invalid,
}

/// Game status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    /// Game is starting
    Starting,
    /// Game is in progress
    Playing,
    /// Game is paused
    Paused,
    /// Game is over (player won)
    Won,
    /// Game is over (player lost)
    Lost,
} 