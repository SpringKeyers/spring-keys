use std::collections::{HashMap, HashSet};
use log::{debug, info};
use rand::{seq::SliceRandom, Rng};

use crate::core::state::GameType;
use super::{Game, GameResult, GameStatus};

/// Minesweeper word game implementation
pub struct MinesweeperGame {
    /// Game board dimensions
    board_size: (usize, usize),
    /// Words on the board
    words: HashMap<Position, Word>,
    /// Exposed (revealed) positions
    exposed: HashSet<Position>,
    /// Flagged positions
    flagged: HashSet<Position>,
    /// Current game status
    status: GameStatus,
    /// Current score
    score: i32,
}

/// Word data structure
#[derive(Debug, Clone)]
pub struct Word {
    /// The word text
    pub text: String,
    /// Whether this word is a mine
    pub is_mine: bool,
    /// Number of neighboring mines
    pub neighbor_count: u8,
}

/// Board position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub usize, pub usize);

/// Similarity groups for word generation
const SIMILARITY_GROUPS: &[&[&str]] = &[
    &["doggy", "foggy", "soggy", "boggy", "goggle", "dodgy"],
    &["tappy", "happy", "sappy", "zappy", "nappy", "taping"],
    &["flippy", "floppy", "flappy", "slippy", "trippy", "flimsy"],
    &["goddy", "goofy", "goody", "golly", "gordy", "godly"],
    &["piggy", "wiggy", "diggy", "jiggy", "riggy", "piggly"],
    &["tabby", "tubby", "cabby", "libby", "lobby", "taboo"],
    &["twerpy", "twirly", "tweets", "twenty", "twofold", "tweaks"],
    &["saggy", "baggy", "waggy", "naggy", "faggy", "soggy"],
];

impl Game for MinesweeperGame {
    fn new() -> Self {
        let board_size = (3, 3);  // 3x3 grid of words
        let mut game = Self {
            board_size,
            words: HashMap::new(),
            exposed: HashSet::new(),
            flagged: HashSet::new(),
            status: GameStatus::Starting,
            score: 0,
        };
        
        game.initialize_board();
        game.status = GameStatus::Playing;
        
        info!("Minesweeper game initialized with board size {:?}", board_size);
        game
    }
    
    fn game_type(&self) -> GameType {
        GameType::Minesweeper
    }
    
    fn process_input(&mut self, input: &str) -> GameResult {
        if self.status != GameStatus::Playing {
            return GameResult::Invalid;
        }
        
        // Check if it's a flag command (CAPS)
        let is_flag = input.chars().all(|c| c.is_uppercase());
        let input = input.to_lowercase();
        
        // Find the word on the board
        let mut found_pos = None;
        for (pos, word) in &self.words {
            if word.text == input {
                found_pos = Some(*pos);
                break;
            }
        }
        
        if let Some(pos) = found_pos {
            if is_flag {
                // Flag the position
                if !self.exposed.contains(&pos) {
                    self.flagged.insert(pos);
                    debug!("Flagged position {:?} with word '{}'", pos, input);
                    
                    // Award points for correctly flagging mines
                    if let Some(word) = self.words.get(&pos) {
                        if word.is_mine {
                            self.score += 10;
                        } else {
                            self.score -= 5;
                        }
                    }
                }
                
                // Check win condition (all mines flagged)
                if self.check_win_condition() {
                    self.status = GameStatus::Won;
                    info!("Player won the game with score {}", self.score);
                    return GameResult::Win;
                }
                
                return GameResult::Continue;
            } else {
                // Expose the position
                if !self.exposed.contains(&pos) && !self.flagged.contains(&pos) {
                    self.exposed.insert(pos);
                    debug!("Exposed position {:?} with word '{}'", pos, input);
                    
                    // Check if it's a mine
                    if let Some(word) = self.words.get(&pos) {
                        if word.is_mine {
                            self.status = GameStatus::Lost;
                            info!("Player lost the game at position {:?}", pos);
                            return GameResult::Lose;
                        } else {
                            // Award points for exposing safe areas
                            self.score += 5 + (word.neighbor_count as i32) * 2;
                            
                            // Auto-expose neighbors if neighbor_count is 0
                            if word.neighbor_count == 0 {
                                self.auto_expose_neighbors(pos);
                            }
                            
                            // Check win condition (all non-mines exposed)
                            if self.check_win_condition() {
                                self.status = GameStatus::Won;
                                info!("Player won the game with score {}", self.score);
                                return GameResult::Win;
                            }
                        }
                    }
                }
                
                return GameResult::Continue;
            }
        } else {
            // Word not found on the board
            debug!("Invalid input: word '{}' not found on the board", input);
            return GameResult::Invalid;
        }
    }
    
    fn status(&self) -> GameStatus {
        self.status
    }
    
    fn score(&self) -> i32 {
        self.score
    }
}

impl MinesweeperGame {
    /// Initialize the game board with words and mines
    fn initialize_board(&mut self) {
        let (width, height) = self.board_size;
        let total_cells = width * height;
        let mine_count = total_cells / 3; // About 1/3 of cells are mines
        
        // Create a list of all positions
        let mut positions = Vec::with_capacity(total_cells);
        for y in 0..height {
            for x in 0..width {
                positions.push(Position(x, y));
            }
        }
        
        // Randomly select positions for mines
        let mut rng = rand::thread_rng();
        let mine_positions: HashSet<Position> = positions
            .choose_multiple(&mut rng, mine_count)
            .cloned()
            .collect();
        
        // Place words on the board
        for y in 0..height {
            for x in 0..width {
                let pos = Position(x, y);
                let is_mine = mine_positions.contains(&pos);
                
                // Select a random word group and word from that group
                let group_idx = rng.gen_range(0..SIMILARITY_GROUPS.len());
                let word_idx = rng.gen_range(0..SIMILARITY_GROUPS[group_idx].len());
                let word_text = SIMILARITY_GROUPS[group_idx][word_idx].to_string();
                
                // Count neighboring mines
                let neighbor_count = self.count_neighboring_mines(&pos, &mine_positions);
                
                self.words.insert(pos, Word {
                    text: word_text,
                    is_mine,
                    neighbor_count,
                });
            }
        }
    }
    
    /// Count neighboring mines for a position
    fn count_neighboring_mines(&self, pos: &Position, mines: &HashSet<Position>) -> u8 {
        let mut count = 0;
        let (width, height) = self.board_size;
        let neighbors = self.get_neighbors(pos);
        
        for neighbor in neighbors {
            if neighbor.0 < width && neighbor.1 < height && mines.contains(&neighbor) {
                count += 1;
            }
        }
        
        count
    }
    
    /// Get neighboring positions
    fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut neighbors = Vec::with_capacity(8);
        let x = pos.0 as isize;
        let y = pos.1 as isize;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Skip the position itself
                }
                
                let nx = x + dx;
                let ny = y + dy;
                
                if nx >= 0 && ny >= 0 {
                    neighbors.push(Position(nx as usize, ny as usize));
                }
            }
        }
        
        neighbors
    }
    
    /// Auto-expose neighbors for cells with 0 neighboring mines
    fn auto_expose_neighbors(&mut self, pos: Position) {
        let neighbors = self.get_neighbors(&pos);
        
        for neighbor in neighbors {
            if !self.exposed.contains(&neighbor) && !self.flagged.contains(&neighbor) {
                if let Some(word) = self.words.get(&neighbor) {
                    if !word.is_mine {
                        self.exposed.insert(neighbor);
                        
                        // Recursive call for 0-neighbors
                        if word.neighbor_count == 0 {
                            self.auto_expose_neighbors(neighbor);
                        }
                    }
                }
            }
        }
    }
    
    /// Check win condition
    fn check_win_condition(&self) -> bool {
        let all_mines_flagged = self.words.iter()
            .filter(|(_, word)| word.is_mine)
            .all(|(pos, _)| self.flagged.contains(pos));
            
        let all_safe_cells_exposed = self.words.iter()
            .filter(|(_, word)| !word.is_mine)
            .all(|(pos, _)| self.exposed.contains(pos));
            
        all_mines_flagged || all_safe_cells_exposed
    }
    
    /// Get a formatted representation of the board
    pub fn get_board_display(&self) -> String {
        let (width, height) = self.board_size;
        let mut display = String::from("# Minesweeper\n~ Exposed\n? Unknown\n");
        
        // Add top border
        display.push_str(&"#".repeat(50));
        display.push('\n');
        
        for y in 0..height {
            for x in 0..width {
                let pos = Position(x, y);
                let word = self.words.get(&pos).unwrap();
                
                display.push_str("###");
                
                if self.exposed.contains(&pos) {
                    display.push_str("~");
                    
                    if word.is_mine {
                        display.push_str(" MINE ");
                    } else if word.neighbor_count > 0 {
                        display.push_str(&format!("   {}   ", word.neighbor_count));
                    } else {
                        display.push_str("       ");
                    }
                    
                    display.push_str("~");
                } else if self.flagged.contains(&pos) {
                    display.push_str(" FLAG    ");
                } else {
                    display.push_str("   ?    ");
                }
                
                display.push_str("###");
            }
            display.push('\n');
            
            for x in 0..width {
                let pos = Position(x, y);
                let word = self.words.get(&pos).unwrap();
                
                display.push_str("###");
                
                if self.exposed.contains(&pos) {
                    display.push_str("~");
                    let padding = (7 - word.text.len()) / 2;
                    display.push_str(&" ".repeat(padding));
                    display.push_str(&word.text);
                    display.push_str(&" ".repeat(7 - word.text.len() - padding));
                    display.push_str("~");
                } else if self.flagged.contains(&pos) {
                    display.push_str(&format!(" !{}! ", word.text));
                } else {
                    display.push_str(&format!(" {}  ", word.text));
                }
                
                display.push_str("###");
            }
            display.push('\n');
            
            // Add separator
            display.push_str(&"#".repeat(50));
            display.push('\n');
        }
        
        display
    }
} 