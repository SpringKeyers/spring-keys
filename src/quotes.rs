use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io;

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
    /// Typewriter and printing technology quotes
    Typewriters,
    /// Typing lessons and exercises
    Lessons,
    /// Basic number typing exercises
    LessonsNumbersBasic,
    /// Alternate hand typing exercises
    LessonsAlternateHands,
    /// Home row typing exercises
    LessonsHomeRow,
    /// Top row typing exercises
    LessonsTopRow,
    /// Bottom row typing exercises
    LessonsBottomRow,
    /// Symbol typing exercises
    LessonsSymbols,
    /// Speed typing exercises
    LessonsSpeed,
    /// Accuracy typing exercises
    LessonsAccuracy,
    /// Song lyrics for typing practice
    SongLyrics,
    /// Tautological statements for typing practice
    Tautologies,
    /// Sacred texts from various traditions
    SacredTexts,
    /// Sacred texts from the Bible
    SacredTextsBible,
    /// Sacred texts from the Torah
    SacredTextsTorah,
    /// Sacred texts from the Quran
    SacredTextsQuran,
    /// Sacred texts from the Vedas
    SacredTextsVedas,
    /// Sacred texts from the Tripitaka
    SacredTextsTripitaka,
    /// Sacred texts from the Analects
    SacredTextsAnalects,
    /// Sacred texts from the Avesta
    SacredTextsAvesta,
    /// Sacred texts from the Guru Granth Sahib
    SacredTextsGuruGranthSahib,
    /// Sacred texts from the Kitáb-i-Aqdas
    SacredTextsKitabIAqdas,
    /// Sacred texts from the Tao Te Ching
    SacredTextsTaoTeChing,
    /// Sacred texts from the Popol Vuh
    SacredTextsPopolVuh,
    /// Sacred texts from the Florentine Codex
    SacredTextsFlorentineCodex,
    /// Sacred texts from the Iroquois Constitution
    SacredTextsIroquoisConstitution,
    /// Sacred texts from the Odù Ifá
    SacredTextsOduIfa,
    /// Sacred texts from the Kebra Nagast
    SacredTextsKebraNagast,
    /// Sacred texts from the Kojiki
    SacredTextsKojiki,
    /// Sacred texts from the Darangen
    SacredTextsDarangen,
    /// Sacred texts from the Kumulipo
    SacredTextsKumulipo,
    /// Sacred texts from Alberta traditions
    SacredTextsAlberta,
    /// Sacred texts from CIRI traditions
    SacredTextsCIRI,
    /// Sacred texts from Inuit oral traditions
    SacredTextsInuitOral,
    /// Sacred texts from Kenaitze traditions
    SacredTextsKenaitze,
    /// Sacred texts from river valley traditions
    SacredTextsRiverValleys,
    /// Sacred texts from Yukon traditions
    SacredTextsYukon,
    /// Sacred texts from Yupik oral traditions
    SacredTextsYupikOral,
    /// Sacred texts from Jamaican traditions
    SacredTextsJamaica,
    /// Holiday quotes
    Holiday,
    /// Purpose-related quotes
    Purpose,
    /// Mortality-related quotes
    Mortality,
    /// Emotion-related quotes
    Emotions,
    /// Self-mastery quotes
    SelfMastery,
    /// Value-related quotes
    Values,
    /// Progress-related quotes
    Progress,
    /// Hope-related quotes
    Hope,
    /// Intuition-related quotes
    Intuition,
    /// Growth-related quotes
    Growth,
    /// Resilience-related quotes
    Resilience,
    /// Action-related quotes
    Action,
    /// Worth-related quotes
    Worth,
    /// Perspective-related quotes
    Perspective,
    /// Effort-related quotes
    Effort,
    /// Happiness-related quotes
    Happiness,
    /// Problem-solving quotes
    ProblemSolving,
    /// Kindness-related quotes
    Kindness,
    /// Courage-related quotes
    Courage,
    /// Perseverance-related quotes
    Perseverance,
    /// Self-confidence quotes
    SelfConfidence,
    /// Human nature quotes
    HumanNature,
    /// Connection-related quotes
    Connection,
    /// Existentialism quotes
    Existentialism,
    /// History and growth quotes
    HistoryAndGrowth,
    /// Potential-related quotes
    Potential,
}

/// F-key category groups for cycling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CategoryCycle {
    Typewriter,
    Programming,
    Literature,
}

impl CategoryCycle {
    /// Get the categories associated with this cycle group
    fn categories(&self) -> Vec<QuoteCategory> {
        match self {
            CategoryCycle::Typewriter => vec![
                QuoteCategory::Typewriters,
                QuoteCategory::TongueTwisters,
                QuoteCategory::Multilingual,
            ],
            CategoryCycle::Programming => vec![
                QuoteCategory::Programming,
                QuoteCategory::Humor,
                QuoteCategory::Proverbs,
            ],
            CategoryCycle::Literature => vec![
                QuoteCategory::Literature,
                QuoteCategory::Proverbs,
                QuoteCategory::Multilingual,
            ],
        }
    }
}

/// Category type for quotes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuoteCategoryType {
    /// Predefined category from the enum
    Predefined(QuoteCategory),
    /// Custom category as a string
    Custom(String),
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
    pub category: QuoteCategoryType,
    /// Language of origin
    pub origin: String,
}

/// Quote database for typing practice
#[derive(Debug)]
pub struct QuoteDatabase {
    /// All available quotes
    quotes: Vec<Quote>,
    /// Quotes by category
    quotes_by_category: HashMap<QuoteCategoryType, Vec<usize>>,
    /// Quotes by difficulty
    quotes_by_difficulty: HashMap<QuoteDifficulty, Vec<usize>>,
    /// Current quote index
    current_index: usize,
    /// Starting offset for quote cycling
    starting_offset: usize,
    /// Random number generator
    rng: rand::rngs::ThreadRng,
    /// Current category index for each cycle group
    category_cycle_indices: HashMap<CategoryCycle, usize>,
}

impl QuoteDatabase {
    /// Create a new quote database with quotes loaded from JSON files
    pub fn new() -> Self {
        let quotes = Self::load_quotes_from_files().unwrap_or_else(|_| {
            eprintln!("Error loading quotes from files, using default quotes");
            Self::default_quotes()
        });
        
        let mut quotes_by_category = HashMap::new();
        let mut quotes_by_difficulty = HashMap::new();
        let mut rng = rand::thread_rng();
        
        // Organize quotes by category and difficulty
        for (i, quote) in quotes.iter().enumerate() {
            quotes_by_category
                .entry(quote.category.clone())
                .or_insert_with(Vec::new)
                .push(i);
                
            quotes_by_difficulty
                .entry(quote.difficulty)
                .or_insert_with(Vec::new)
                .push(i);
        }
        
        // Generate random starting offset
        let starting_offset = if !quotes.is_empty() {
            rng.gen::<usize>() % quotes.len()
        } else {
            0
        };

        // Initialize category cycle indices
        let mut category_cycle_indices = HashMap::new();
        category_cycle_indices.insert(CategoryCycle::Typewriter, 0);
        category_cycle_indices.insert(CategoryCycle::Programming, 0);
        category_cycle_indices.insert(CategoryCycle::Literature, 0);
        
        Self {
            quotes,
            quotes_by_category,
            quotes_by_difficulty,
            current_index: starting_offset,
            starting_offset,
            rng,
            category_cycle_indices,
        }
    }

    /// Create a new quote database with quotes loaded from JSON files, without printing loading messages
    pub fn new_silent() -> Self {
        let quotes = Self::load_quotes_from_files_silent().unwrap_or_else(|_| {
            Self::default_quotes()
        });
        
        let mut quotes_by_category = HashMap::new();
        let mut quotes_by_difficulty = HashMap::new();
        let mut rng = rand::thread_rng();
        
        // Organize quotes by category and difficulty
        for (i, quote) in quotes.iter().enumerate() {
            quotes_by_category
                .entry(quote.category.clone())
                .or_insert_with(Vec::new)
                .push(i);
                
            quotes_by_difficulty
                .entry(quote.difficulty)
                .or_insert_with(Vec::new)
                .push(i);
        }
        
        // Generate random starting offset
        let starting_offset = if !quotes.is_empty() {
            rng.gen::<usize>() % quotes.len()
        } else {
            0
        };

        // Initialize category cycle indices
        let mut category_cycle_indices = HashMap::new();
        category_cycle_indices.insert(CategoryCycle::Typewriter, 0);
        category_cycle_indices.insert(CategoryCycle::Programming, 0);
        category_cycle_indices.insert(CategoryCycle::Literature, 0);
        
        Self {
            quotes,
            quotes_by_category,
            quotes_by_difficulty,
            current_index: starting_offset,
            starting_offset,
            rng,
            category_cycle_indices,
        }
    }

    /// Load quotes from JSON files in the quotes directory
    fn load_quotes_from_files() -> io::Result<Vec<Quote>> {
        let mut all_quotes = Vec::new();
        let categories_dir = Path::new("quotes/categories");
        
        // Check if the directory exists
        if !categories_dir.exists() {
            eprintln!("Quotes directory not found, using default quotes");
            return Ok(Self::default_quotes());
        }
        
        // Get all JSON files and sort them alphabetically
        let mut entries: Vec<_> = fs::read_dir(categories_dir)?.collect();
        entries.sort_by(|a, b| {
            let a_path = a.as_ref().unwrap().path();
            let b_path = b.as_ref().unwrap().path();
            a_path.file_name().unwrap().cmp(b_path.file_name().unwrap())
        });
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file_content = fs::read_to_string(&path)?;
                match serde_json::from_str::<Vec<Quote>>(&file_content) {
                    Ok(quotes) => {
                        println!("Loaded {:3} quotes from {:?}", quotes.len(), path);
                        all_quotes.extend(quotes);
                    },
                    Err(e) => {
                        eprintln!("Error parsing quotes from {:?}: {}", path, e);
                    }
                }
            }
        }
        
        // If no quotes were loaded, return default quotes
        if all_quotes.is_empty() {
            eprintln!("No quotes found in JSON files, using default quotes");
            Ok(Self::default_quotes())
        } else {
            println!("Successfully loaded {:3} quotes from JSON files", all_quotes.len());
            Ok(all_quotes)
        }
    }

    /// Load quotes from JSON files without printing loading messages
    fn load_quotes_from_files_silent() -> io::Result<Vec<Quote>> {
        let mut all_quotes = Vec::new();
        let categories_dir = Path::new("quotes/categories");
        
        // Check if the directory exists
        if !categories_dir.exists() {
            return Ok(Self::default_quotes());
        }
        
        // Get all JSON files and sort them alphabetically
        let mut entries: Vec<_> = fs::read_dir(categories_dir)?.collect();
        entries.sort_by(|a, b| {
            let a_path = a.as_ref().unwrap().path();
            let b_path = b.as_ref().unwrap().path();
            a_path.file_name().unwrap().cmp(b_path.file_name().unwrap())
        });
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file_content = fs::read_to_string(&path)?;
                if let Ok(quotes) = serde_json::from_str::<Vec<Quote>>(&file_content) {
                    all_quotes.extend(quotes);
                }
            }
        }
        
        // If no quotes were loaded, return default quotes
        if all_quotes.is_empty() {
            Ok(Self::default_quotes())
        } else {
            Ok(all_quotes)
        }
    }
    
    /// Get the next quote in sequence
    pub fn next_sequential(&mut self) -> &Quote {
        // TODO: investigate sequential quote progression
        if self.quotes.is_empty() {
            panic!("No quotes available");
        }
        
        self.current_index = (self.current_index + 1) % self.quotes.len();
        &self.quotes[self.current_index]
    }
    
    /// Jump to a new random starting point
    pub fn jump_random(&mut self) -> &Quote {
        // TODO: investigate random quote selection
        if self.quotes.is_empty() {
            panic!("No quotes available");
        }
        
        // Generate new random offset
        self.starting_offset = self.rng.gen::<usize>() % self.quotes.len();
        self.current_index = self.starting_offset;
        &self.quotes[self.current_index]
    }
    
    /// Get the next random quote (completely random, not sequential)
    pub fn next_random(&mut self) -> &Quote {
        // TODO: investigate random quote selection
        let index = (0..self.quotes.len())
            .choose(&mut self.rng)
            .unwrap_or(0);
        self.current_index = index;
        &self.quotes[index]
    }
    
    /// Get the next random quote of a specific difficulty
    pub fn next_by_difficulty(&mut self, difficulty: QuoteDifficulty) -> Option<&Quote> {
        // TODO: investigate difficulty-based quote selection
        if let Some(indices) = self.quotes_by_difficulty.get(&difficulty) {
            if let Some(&index) = indices.choose(&mut self.rng) {
                self.current_index = index;
                return Some(&self.quotes[index]);
            }
        }
        None
    }
    
    /// Get the next random quote of a specific category
    pub fn next_by_category(&mut self, category: QuoteCategoryType) -> Option<&Quote> {
        // TODO: investigate category-based quote selection
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
            .entry(quote.category.clone())
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
    /// This is used as a fallback if JSON files fail to load
    fn default_quotes() -> Vec<Quote> {
        vec![
            Quote {
                text: "The early bird might get the worm, but the second mouse gets the cheese.".to_string(),
                source: "English wisdom".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategoryType::Predefined(QuoteCategory::Proverbs),
                origin: "English".to_string(),
            },
            Quote {
                text: "Six sitting scientists sorted sixty slippery snakes successfully.".to_string(),
                source: "English tongue twister".to_string(),
                difficulty: QuoteDifficulty::Hard,
                category: QuoteCategoryType::Predefined(QuoteCategory::TongueTwisters),
                origin: "English".to_string(),
            },
            Quote {
                text: "Talk is cheap. Show me the code.".to_string(),
                source: "Linus Torvalds".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategoryType::Predefined(QuoteCategory::Programming),
                origin: "Finnish/American".to_string(),
            },
            Quote {
                text: "To be, or not to be, that is the question.".to_string(),
                source: "William Shakespeare, Hamlet".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategoryType::Predefined(QuoteCategory::Literature),
                origin: "English".to_string(),
            },
            Quote {
                text: "A bird in the hand is worth two in the bush.".to_string(),
                source: "English proverb".to_string(),
                difficulty: QuoteDifficulty::Easy,
                category: QuoteCategoryType::Predefined(QuoteCategory::Proverbs),
                origin: "English".to_string(),
            },
        ]
    }

    /// Cycle to the next category for the given F-key group
    pub fn cycle_category(&mut self, cycle_group: CategoryCycle) -> QuoteCategory {
        let categories = cycle_group.categories();
        let current_index = self.category_cycle_indices.entry(cycle_group).or_insert(0);
        *current_index = (*current_index + 1) % categories.len();
        categories[*current_index]
    }

    /// Get the currently active category for a cycle group
    pub fn get_active_category(&self, cycle_group: CategoryCycle) -> QuoteCategory {
        let categories = cycle_group.categories();
        let current_index = self.category_cycle_indices.get(&cycle_group).unwrap_or(&0);
        categories[*current_index]
    }

    /// Get the next quote from the active category of a cycle group
    pub fn next_from_cycle_group(&mut self, cycle_group: CategoryCycle) -> Option<&Quote> {
        let active_category = QuoteCategoryType::Predefined(self.get_active_category(cycle_group));
        self.next_by_category(active_category)
    }

    pub fn set_active_category(&mut self, category: CategoryCycle) {
        // Convert CategoryCycle to QuoteCategory
        let quote_category = QuoteCategoryType::Predefined(match category {
            CategoryCycle::Typewriter => QuoteCategory::Typewriters,
            CategoryCycle::Programming => QuoteCategory::Programming,
            CategoryCycle::Literature => QuoteCategory::Literature,
        });
        
        // Update the current index to point to a quote from the desired category
        if let Some(quotes) = self.quotes_by_category.get(&quote_category) {
            if !quotes.is_empty() {
                let current_cycle_index = self.category_cycle_indices.entry(category).or_insert(0);
                self.current_index = quotes[*current_cycle_index];
                *current_cycle_index = (*current_cycle_index + 1) % quotes.len();
            }
        }
    }
} 