use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Difficulty levels for quotes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuoteDifficulty {
    /// Easy quotes for beginners
    Easy,
    /// Medium quotes with some challenges
    Medium,
    /// Hard quotes for advanced typists
    Hard,
}

/// Category of quotes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuoteCategory {
    /// Wisdom and proverbs
    Proverbs,
    /// Tongue twisters
    TongueTwisters,
    /// Famous literature quotes
    Literature,
    /// Programming-related quotes
    Programming,
    /// Humorous quotes
    Humor,
    /// Multilingual quotes
    Multilingual,
}

/// A typing quote with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// The quote text to type
    pub text: String,
    /// The quote author or source
    pub source: String,
    /// The quote difficulty
    pub difficulty: QuoteDifficulty,
    /// The quote category
    pub category: QuoteCategory,
    /// Language of origin
    pub origin: String,
}

/// Quote database for typing practice
#[derive(Debug)]
pub struct QuoteDatabase {
    /// All available quotes
    quotes: Vec<Quote>,
    /// Quotes by category
    quotes_by_category: HashMap<QuoteCategory, Vec<usize>>,
    /// Quotes by difficulty
    quotes_by_difficulty: HashMap<QuoteDifficulty, Vec<usize>>,
    /// Current quote index
    current_index: usize,
    /// Random number generator
    rng: rand::rngs::ThreadRng,
}

impl QuoteDatabase {
    /// Create a new quote database with default quotes
    pub fn new() -> Self {
        let quotes = Self::default_quotes();
        let mut quotes_by_category = HashMap::new();
        let mut quotes_by_difficulty = HashMap::new();
        
        // Organize quotes by category and difficulty
        for (i, quote) in quotes.iter().enumerate() {
            quotes_by_category
                .entry(quote.category)
                .or_insert_with(Vec::new)
                .push(i);
                
            quotes_by_difficulty
                .entry(quote.difficulty)
                .or_insert_with(Vec::new)
                .push(i);
        }
        
        Self {
            quotes,
            quotes_by_category,
            quotes_by_difficulty,
            current_index: 0,
            rng: rand::thread_rng(),
        }
    }
    
    /// Get the next random quote
    pub fn next_random(&mut self) -> &Quote {
        let index = (0..self.quotes.len())
            .choose(&mut self.rng)
            .unwrap_or(0);
        self.current_index = index;
        &self.quotes[index]
    }
    
    /// Get the next random quote of a specific difficulty
    pub fn next_by_difficulty(&mut self, difficulty: QuoteDifficulty) -> Option<&Quote> {
        if let Some(indices) = self.quotes_by_difficulty.get(&difficulty) {
            if let Some(&index) = indices.choose(&mut self.rng) {
                self.current_index = index;
                return Some(&self.quotes[index]);
            }
        }
        None
    }
    
    /// Get the next random quote of a specific category
    pub fn next_by_category(&mut self, category: QuoteCategory) -> Option<&Quote> {
        if let Some(indices) = self.quotes_by_category.get(&category) {
            if let Some(&index) = indices.choose(&mut self.rng) {
                self.current_index = index;
                return Some(&self.quotes[index]);
            }
        }
        None
    }
    
    /// Get the current quote
    pub fn current(&self) -> &Quote {
        &self.quotes[self.current_index]
    }
    
    /// Get a specific quote by index
    pub fn get(&self, index: usize) -> Option<&Quote> {
        self.quotes.get(index)
    }
    
    /// Get all quotes
    pub fn all(&self) -> &[Quote] {
        &self.quotes
    }
    
    /// Add a new quote to the database
    pub fn add(&mut self, quote: Quote) {
        let index = self.quotes.len();
        
        // Add to category and difficulty maps
        self.quotes_by_category
            .entry(quote.category)
            .or_insert_with(Vec::new)
            .push(index);
            
        self.quotes_by_difficulty
            .entry(quote.difficulty)
            .or_insert_with(Vec::new)
            .push(index);
            
        // Add to the main collection
        self.quotes.push(quote);
    }
    
    /// Create default quotes collection
    fn default_quotes() -> Vec<Quote> {
        vec![
            Quote {
                text: "The early bird might get the worm, but the second mouse gets the cheese.".to_string(),
                source: "English wisdom".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::Proverbs,
                origin: "English".to_string(),
            },
            Quote {
                text: "Six sitting scientists sorted sixty slippery snakes successfully.".to_string(),
                source: "English tongue twister".to_string(),
                difficulty: QuoteDifficulty::Hard,
                category: QuoteCategory::TongueTwisters,
                origin: "English".to_string(),
            },
            Quote {
                text: "He who asks is a fool for five minutes, but he who does not ask remains a fool forever.".to_string(),
                source: "Chinese proverb".to_string(),
                difficulty: QuoteDifficulty::Medium,
                category: QuoteCategory::Proverbs,
                origin: "Chinese".to_string(),
            },
            Quote {
                text: "A book is like a garden carried in the pocket.".to_string(),
                source: "Arabic proverb".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::Proverbs,
                origin: "Arabic".to_string(),
            },
            Quote {
                text: "The words of the tongue should have three gatekeepers: Is it true? Is it kind? Is it necessary?".to_string(),
                source: "Arabian wisdom".to_string(),
                difficulty: QuoteDifficulty::Medium,
                category: QuoteCategory::Proverbs,
                origin: "Arabic".to_string(),
            },
            Quote {
                text: "If you chase two rabbits, you will catch neither.".to_string(),
                source: "Russian proverb".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::Proverbs,
                origin: "Russian".to_string(),
            },
            Quote {
                text: "Fireflies flash light signals, frightening frivolous frogs.".to_string(),
                source: "Alliterative rhyme".to_string(),
                difficulty: QuoteDifficulty::Hard,
                category: QuoteCategory::TongueTwisters,
                origin: "English".to_string(),
            },
            Quote {
                text: "Everything flows, nothing stands still.".to_string(),
                source: "Ancient Greek saying (Panta rhei, ouden menei)".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::Proverbs,
                origin: "Greek".to_string(),
            },
            Quote {
                text: "When spider webs unite, they can tie up a lion.".to_string(),
                source: "Ethiopian proverb".to_string(),
                difficulty: QuoteDifficulty::Medium,
                category: QuoteCategory::Proverbs,
                origin: "Ethiopian".to_string(),
            },
            Quote {
                text: "Whoever wants thorns should remember the flowers, whoever wants flowers should remember the thorns.".to_string(),
                source: "Persian poem".to_string(),
                difficulty: QuoteDifficulty::Medium,
                category: QuoteCategory::Literature,
                origin: "Persian".to_string(),
            },
            Quote {
                text: "The nail that sticks out gets hammered down.".to_string(),
                source: "Japanese proverb".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::Proverbs,
                origin: "Japanese".to_string(),
            },
            Quote {
                text: "A rich man's joke is always funny, especially when you're the rich man.".to_string(),
                source: "Russian humor".to_string(),
                difficulty: QuoteDifficulty::Medium,
                category: QuoteCategory::Humor,
                origin: "Russian".to_string(),
            },
            Quote {
                text: "Dance like the photo isn't being tagged, love like you've never been unfriended, and tweet like nobody is following.".to_string(),
                source: "Modern proverb".to_string(),
                difficulty: QuoteDifficulty::Hard,
                category: QuoteCategory::Humor,
                origin: "Internet culture".to_string(),
            },
            Quote {
                text: "Fear not the person who has practiced 10,000 kicks once, but fear the person who has practiced one kick 10,000 times.".to_string(),
                source: "Bruce Lee wisdom".to_string(),
                difficulty: QuoteDifficulty::Hard,
                category: QuoteCategory::Proverbs,
                origin: "Chinese/American".to_string(),
            },
            Quote {
                text: "Little frogs jumping high five fantastic floating fireflies flying freely.".to_string(),
                source: "Alliterative practice".to_string(),
                difficulty: QuoteDifficulty::Hard,
                category: QuoteCategory::TongueTwisters,
                origin: "English".to_string(),
            },
            Quote {
                text: "The quick brown fox jumps over the lazy dog.".to_string(),
                source: "Classic typing pangram".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::TongueTwisters,
                origin: "English".to_string(),
            },
            Quote {
                text: "To be, or not to be, that is the question.".to_string(),
                source: "William Shakespeare, Hamlet".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::Literature,
                origin: "English".to_string(),
            },
            Quote {
                text: "All that glitters is not gold.".to_string(),
                source: "William Shakespeare, The Merchant of Venice".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::Literature,
                origin: "English".to_string(),
            },
            Quote {
                text: "Talk is cheap. Show me the code.".to_string(),
                source: "Linus Torvalds".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategory::Programming,
                origin: "Finnish/American".to_string(),
            },
            Quote {
                text: "Programming isn't about what you know; it's about what you can figure out.".to_string(),
                source: "Chris Pine".to_string(),
                difficulty: QuoteDifficulty::Medium,
                category: QuoteCategory::Programming,
                origin: "English".to_string(),
            },
        ]
    }
} 