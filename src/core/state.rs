#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameType {
    Practice,
    Minesweeper,
    Tetris,
    FlappyBird,
    RcRacing,
    FortuneTeller,
    HockeyFight,
    Consume,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStatus {
    Menu,
    Playing,
    Paused,
    GameOver,
    Victory,
}

#[derive(Debug)]
pub struct GameState {
    pub current_game: GameType,
    pub score: i32,
    pub level: i32,
    pub status: GameStatus,
    pub high_score: i32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_game: GameType::Practice,
            score: 0,
            level: 1,
            status: GameStatus::Menu,
            high_score: 0,
        }
    }
}

impl GameState {
    pub fn new(game_type: GameType) -> Self {
        Self {
            current_game: game_type,
            ..Default::default()
        }
    }

    pub fn update_score(&mut self, points: i32) {
        self.score += points;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn next_level(&mut self) {
        self.level += 1;
    }
} 