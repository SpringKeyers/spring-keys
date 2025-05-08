use std::path::Path;
use std::fs;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryCycle {
    Typewriter,
    Programming,
    Literature,
    TongueTwisters,
    AncientPhilosophy,
    AnimeWisdom,
    ArabicWisdom,
    AsianWisdom,
    Buddhism,
    BuddhistWisdom,
    ChineseWisdom,
    ChristianScripture,
    FolkWisdom,
    GreekWisdom,
    HawaiianWisdom,
    HinduScripture,
    HinduWisdom,
    InuitWisdom,
    IslamicScripture,
    IslamicWisdom,
    JapaneseWisdom,
    JewishScripture,
    JewishWisdom,
    KoreanWisdom,
    PirateWisdom,
    ProgrammingWisdom,
    TypewriterHistory,
    AncientLiterature,
    EasyDatetimeFormats,
    EasyHomeRow,
    HardStockReports,
    Holiday,
    KoreanAnimeWisdom,
    MediumAlternateHands,
    MediumNumbersBasic,
    MediumSongLyrics,
    MediumTautologies,
    SacredTexts,
}

impl CategoryCycle {
    pub fn from_quote_category(category: &QuoteCategory) -> Self {
        use QuoteCategory::*;
        match category {
            Literature => CategoryCycle::Literature,
            Programming => CategoryCycle::Programming,
            TongueTwisters => CategoryCycle::TongueTwisters,
            AncientPhilosophy => CategoryCycle::AncientPhilosophy,
            AnimeWisdom => CategoryCycle::AnimeWisdom,
            ArabicWisdom => CategoryCycle::ArabicWisdom,
            AsianWisdom => CategoryCycle::AsianWisdom,
            Buddhism => CategoryCycle::Buddhism,
            BuddhistWisdom => CategoryCycle::BuddhistWisdom,
            ChineseWisdom => CategoryCycle::ChineseWisdom,
            ChristianScripture => CategoryCycle::ChristianScripture,
            FolkWisdom => CategoryCycle::FolkWisdom,
            GreekWisdom => CategoryCycle::GreekWisdom,
            HawaiianWisdom => CategoryCycle::HawaiianWisdom,
            HinduScripture => CategoryCycle::HinduScripture,
            HinduWisdom => CategoryCycle::HinduWisdom,
            InuitWisdom => CategoryCycle::InuitWisdom,
            IslamicScripture => CategoryCycle::IslamicScripture,
            IslamicWisdom => CategoryCycle::IslamicWisdom,
            JapaneseWisdom => CategoryCycle::JapaneseWisdom,
            JewishScripture => CategoryCycle::JewishScripture,
            JewishWisdom => CategoryCycle::JewishWisdom,
            KoreanWisdom => CategoryCycle::KoreanWisdom,
            PirateWisdom => CategoryCycle::PirateWisdom,
            ProgrammingWisdom => CategoryCycle::ProgrammingWisdom,
            TypewriterHistory => CategoryCycle::TypewriterHistory,
            AncientLiterature => CategoryCycle::AncientLiterature,
            EasyDatetimeFormats => CategoryCycle::EasyDatetimeFormats,
            EasyHomeRow => CategoryCycle::EasyHomeRow,
            HardStockReports => CategoryCycle::HardStockReports,
            HolidayChineseNewYear | HolidayChristmas | HolidayColumbusDay | 
            HolidayEaster | HolidayIndependenceDay | HolidayJuneteenth | 
            HolidayLaborDay | HolidayMemorialDay | HolidayMlkDay | 
            HolidayNewYear | HolidayPresidentsDay | HolidayThanksgiving | 
            HolidayVeteransDay => CategoryCycle::Holiday,
            KoreanAnimeWisdom => CategoryCycle::KoreanAnimeWisdom,
            MediumAlternateHands => CategoryCycle::MediumAlternateHands,
            MediumNumbersBasic => CategoryCycle::MediumNumbersBasic,
            MediumSongLyrics => CategoryCycle::MediumSongLyrics,
            MediumTautologies => CategoryCycle::MediumTautologies,
            SacredTextsAfrican | SacredTextsAlberta | SacredTextsAnalects | 
            SacredTextsAvesta | SacredTextsBible | SacredTextsCelticNorse | 
            SacredTextsCiri | SacredTextsDarangen | SacredTextsEgyptian | 
            SacredTextsFlorentineCodex | SacredTextsGuruGranthSahib | 
            SacredTextsInuitOral | SacredTextsIroquoisConstitution | 
            SacredTextsJamaica | SacredTextsKebraNagast | SacredTextsKenaitze | 
            SacredTextsKitabIAqdas | SacredTextsKojiki | SacredTextsKumulipo | 
            SacredTextsMartialArts | SacredTextsModern | SacredTextsNativeAmerican | 
            SacredTextsOduIfa | SacredTextsPopulVuh | SacredTextsProverbs | 
            SacredTextsQuran | SacredTextsReligious | SacredTextsRiverValleys | 
            SacredTextsRoman | SacredTextsSacred | SacredTextsSikhism | 
            SacredTextsSikhism2 | SacredTextsStoicPhilosophy | SacredTextsStoicism | 
            SacredTextsStoicism2 | SacredTextsTaoTeChing | SacredTextsTaoism | 
            SacredTextsTauTaoism | SacredTextsTorah | SacredTextsTripitaka | 
            SacredTextsUniversal | SacredTextsUniversalWisdom | SacredTextsVedas | 
            SacredTextsYukon | SacredTextsYupikOral | SacredTextsZen | 
            SacredTextsZenBuddhism | SacredTexts => CategoryCycle::SacredTexts,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub text: String,
    pub source: String,
    pub difficulty: QuoteDifficulty,
    pub category: QuoteCategory,
    pub origin: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteCategory {
    Literature,
    Programming,
    TongueTwisters,
    AncientPhilosophy,
    AnimeWisdom,
    ArabicWisdom,
    AsianWisdom,
    Buddhism,
    BuddhistWisdom,
    ChineseWisdom,
    ChristianScripture,
    FolkWisdom,
    GreekWisdom,
    HawaiianWisdom,
    HinduScripture,
    HinduWisdom,
    InuitWisdom,
    IslamicScripture,
    IslamicWisdom,
    JapaneseWisdom,
    JewishScripture,
    JewishWisdom,
    KoreanWisdom,
    PirateWisdom,
    ProgrammingWisdom,
    TypewriterHistory,
    AncientLiterature,
    EasyDatetimeFormats,
    EasyHomeRow,
    HardStockReports,
    HolidayChineseNewYear,
    HolidayChristmas,
    HolidayColumbusDay,
    HolidayEaster,
    HolidayIndependenceDay,
    HolidayJuneteenth,
    HolidayLaborDay,
    HolidayMemorialDay,
    HolidayMlkDay,
    HolidayNewYear,
    HolidayPresidentsDay,
    HolidayThanksgiving,
    HolidayVeteransDay,
    KoreanAnimeWisdom,
    MediumAlternateHands,
    MediumNumbersBasic,
    MediumSongLyrics,
    MediumTautologies,
    SacredTextsAfrican,
    SacredTextsAlberta,
    SacredTextsAnalects,
    SacredTextsAvesta,
    SacredTextsBible,
    SacredTextsCelticNorse,
    SacredTextsCiri,
    SacredTextsDarangen,
    SacredTextsEgyptian,
    SacredTextsFlorentineCodex,
    SacredTextsGuruGranthSahib,
    SacredTextsInuitOral,
    SacredTextsIroquoisConstitution,
    SacredTextsJamaica,
    SacredTextsKebraNagast,
    SacredTextsKenaitze,
    SacredTextsKitabIAqdas,
    SacredTextsKojiki,
    SacredTextsKumulipo,
    SacredTextsMartialArts,
    SacredTextsModern,
    SacredTextsNativeAmerican,
    SacredTextsOduIfa,
    SacredTextsPopulVuh,
    SacredTextsProverbs,
    SacredTextsQuran,
    SacredTextsReligious,
    SacredTextsRiverValleys,
    SacredTextsRoman,
    SacredTextsSacred,
    SacredTextsSikhism,
    SacredTextsSikhism2,
    SacredTextsStoicPhilosophy,
    SacredTextsStoicism,
    SacredTextsStoicism2,
    SacredTextsTaoTeChing,
    SacredTextsTaoism,
    SacredTextsTauTaoism,
    SacredTextsTorah,
    SacredTextsTripitaka,
    SacredTextsUniversal,
    SacredTextsUniversalWisdom,
    SacredTextsVedas,
    SacredTextsYukon,
    SacredTextsYupikOral,
    SacredTextsZen,
    SacredTextsZenBuddhism,
    SacredTexts,
}

#[derive(Debug)]
pub struct QuoteDatabase {
    quotes: Vec<Quote>,
    rng: rand::rngs::ThreadRng,
    active_categories: Vec<CategoryCycle>,
}

impl QuoteDatabase {
    pub fn new() -> Self {
        let mut db = Self {
            quotes: Vec::new(),
            rng: rand::thread_rng(),
            active_categories: vec![
                CategoryCycle::Typewriter,
                CategoryCycle::Programming,
                CategoryCycle::Literature,
                CategoryCycle::TongueTwisters,
                CategoryCycle::SacredTexts,
                CategoryCycle::Holiday,
            ],
        };
        db.load_quotes(false);
        db
    }

    pub fn new_silent() -> Self {
        let mut db = Self {
            quotes: Vec::new(),
            rng: rand::thread_rng(),
            active_categories: vec![
                CategoryCycle::Typewriter,
                CategoryCycle::Programming,
                CategoryCycle::Literature,
                CategoryCycle::TongueTwisters,
                CategoryCycle::SacredTexts,
                CategoryCycle::Holiday,
            ],
        };
        db.load_quotes(true);
        db
    }

    pub fn total_quotes(&self) -> usize {
        self.quotes.len()
    }

    fn load_quotes(&mut self, silent: bool) {
        let quote_dir = Path::new("quotes/categories");
        let mut total_quotes = 0;

        if let Ok(entries) = fs::read_dir(quote_dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "json" {
                        if let Ok(file_content) = fs::read_to_string(entry.path()) {
                            if let Ok(mut quotes) = serde_json::from_str::<Vec<Quote>>(&file_content) {
                                let count = quotes.len();
                                total_quotes += count;
                                if !silent {
                                    println!("Loaded {:>3} quotes from {:?}", count, entry.path().display());
                                }
                                self.quotes.append(&mut quotes);
                            }
                        }
                    }
                }
            }
        }

        if !silent {
            println!("Successfully loaded {} quotes from JSON files", total_quotes);
        }
    }

    pub fn next_random(&mut self) -> &Quote {
        let index = self.rng.gen_range(0..self.quotes.len());
        &self.quotes[index]
    }

    pub fn next_by_difficulty(&mut self, difficulty: QuoteDifficulty) -> Option<&Quote> {
        let matching_quotes: Vec<_> = self.quotes.iter()
            .filter(|q| q.difficulty == difficulty)
            .collect();

        if matching_quotes.is_empty() {
            None
        } else {
            let index = self.rng.gen_range(0..matching_quotes.len());
            Some(matching_quotes[index])
        }
    }

    pub fn all(&self) -> &[Quote] {
        &self.quotes
    }

    pub fn next_by_category(&mut self, category: CategoryCycle) -> Option<&Quote> {
        let matching_quotes: Vec<_> = self.quotes.iter()
            .filter(|q| CategoryCycle::from_quote_category(&q.category) == category)
            .collect();

        if matching_quotes.is_empty() {
            None
        } else {
            let index = self.rng.gen_range(0..matching_quotes.len());
            Some(matching_quotes[index])
        }
    }

    pub fn cycle_category(&mut self) -> CategoryCycle {
        if let Some(category) = self.active_categories.pop() {
            self.active_categories.insert(0, category);
            category
        } else {
            CategoryCycle::Literature
        }
    }

    pub fn get_current_category(&self) -> Option<CategoryCycle> {
        self.active_categories.first().copied()
    }

    pub fn set_active_category(&mut self, category: CategoryCycle) {
        if !self.active_categories.contains(&category) {
            self.active_categories.push(category);
        }
        if let Some(pos) = self.active_categories.iter().position(|&c| c == category) {
            self.active_categories.remove(pos);
            self.active_categories.insert(0, category);
        }
    }

    pub fn get_active_category(&self, category: CategoryCycle) -> bool {
        self.active_categories.contains(&category)
    }

    pub fn get_all_categories(&self) -> Vec<CategoryCycle> {
        self.active_categories.clone()
    }
} 