use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub text: String,
    pub source: String,
    pub difficulty: QuoteDifficulty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CategoryCycle {
    All,
    Programming,
    Literature,
    Typewriter,
}

#[derive(Debug, Clone)]
pub struct QuoteDatabase {
    quotes: Vec<Quote>,
    active_category: CategoryCycle,
    quiet_mode: bool,
}

impl QuoteDatabase {
    pub fn new() -> Self {
        Self::new_with_options(false)
    }

    pub fn new_silent() -> Self {
        Self::new_with_options(true)
    }

    fn new_with_options(quiet_mode: bool) -> Self {
        let mut db = Self {
            quotes: Vec::new(),
            active_category: CategoryCycle::All,
            quiet_mode,
        };
        db.load_quotes();
        db
    }

    fn load_quotes(&mut self) {
        let categories_dir = "quotes/categories";
        if let Ok(entries) = fs::read_dir(categories_dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "json" {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if let Ok(mut quotes) = serde_json::from_str::<Vec<Quote>>(&content) {
                                let count = quotes.len();
                                self.quotes.append(&mut quotes);
                                if !self.quiet_mode {
                                    println!("Loaded {:3} quotes from {:?}", count, entry.path().display());
                                }
                            }
                        }
                    }
                }
            }
            if !self.quiet_mode {
                println!("Successfully loaded {} quotes from JSON files", self.quotes.len());
            }
        }
    }

    pub fn next_random(&mut self) -> Quote {
        let mut rng = rand::thread_rng();
        let quotes = match self.active_category {
            CategoryCycle::All => &self.quotes,
            CategoryCycle::Programming => &self.quotes, // TODO: Filter by category
            CategoryCycle::Literature => &self.quotes,  // TODO: Filter by category
            CategoryCycle::Typewriter => &self.quotes,  // TODO: Filter by category
        };
        quotes[rng.gen_range(0..quotes.len())].clone()
    }

    pub fn next_by_difficulty(&mut self, difficulty: QuoteDifficulty) -> Option<Quote> {
        let mut rng = rand::thread_rng();
        let matching_quotes: Vec<_> = self.quotes.iter()
            .filter(|q| q.difficulty == difficulty)
            .collect();

        if matching_quotes.is_empty() {
            None
        } else {
            Some(matching_quotes[rng.gen_range(0..matching_quotes.len())].clone())
        }
    }

    pub fn set_active_category(&mut self, category: CategoryCycle) {
        self.active_category = category;
    }

    pub fn get_active_category(&self) -> CategoryCycle {
        self.active_category
    }

    pub fn total_quotes(&self) -> usize {
        self.quotes.len()
    }
} 