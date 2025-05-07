use std::time::{SystemTime, Instant};
use serde::{Serialize, Deserialize};

pub mod state;
pub mod metrics;
pub mod histogram;
pub mod stats;

use metrics::TypingMetrics;

#[derive(Debug, Clone)]
pub struct TypingSession {
    pub start_time: Instant,
    pub metrics: TypingMetrics,
    pub quote_text: String,
    pub current_position: usize,
    pub is_complete: bool,
}

impl Serialize for TypingSession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TypingSession", 4)?;
        state.serialize_field("metrics", &self.metrics)?;
        state.serialize_field("quote_text", &self.quote_text)?;
        state.serialize_field("current_position", &self.current_position)?;
        state.serialize_field("is_complete", &self.is_complete)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TypingSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Metrics,
            QuoteText,
            CurrentPosition,
            IsComplete,
        }

        struct TypingSessionVisitor;

        impl<'de> serde::de::Visitor<'de> for TypingSessionVisitor {
            type Value = TypingSession;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct TypingSession")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TypingSession, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut metrics = None;
                let mut quote_text = None;
                let mut current_position = None;
                let mut is_complete = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Metrics => metrics = Some(map.next_value()?),
                        Field::QuoteText => quote_text = Some(map.next_value()?),
                        Field::CurrentPosition => current_position = Some(map.next_value()?),
                        Field::IsComplete => is_complete = Some(map.next_value()?),
                    }
                }

                Ok(TypingSession {
                    start_time: Instant::now(),
                    metrics: metrics.ok_or_else(|| serde::de::Error::missing_field("metrics"))?,
                    quote_text: quote_text.ok_or_else(|| serde::de::Error::missing_field("quote_text"))?,
                    current_position: current_position.ok_or_else(|| serde::de::Error::missing_field("current_position"))?,
                    is_complete: is_complete.ok_or_else(|| serde::de::Error::missing_field("is_complete"))?,
                })
            }
        }

        const FIELDS: &[&str] = &["metrics", "quote_text", "current_position", "is_complete"];
        deserializer.deserialize_struct("TypingSession", FIELDS, TypingSessionVisitor)
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub last_update: SystemTime,
    pub current_quote: Option<String>,
    pub current_position: usize,
    pub is_complete: bool,
}

impl Serialize for GameState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("GameState", 3)?;
        state.serialize_field("current_quote", &self.current_quote)?;
        state.serialize_field("current_position", &self.current_position)?;
        state.serialize_field("is_complete", &self.is_complete)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for GameState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            CurrentQuote,
            CurrentPosition,
            IsComplete,
        }

        struct GameStateVisitor;

        impl<'de> serde::de::Visitor<'de> for GameStateVisitor {
            type Value = GameState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GameState")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GameState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut current_quote = None;
                let mut current_position = None;
                let mut is_complete = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::CurrentQuote => current_quote = Some(map.next_value()?),
                        Field::CurrentPosition => current_position = Some(map.next_value()?),
                        Field::IsComplete => is_complete = Some(map.next_value()?),
                    }
                }

                Ok(GameState {
                    last_update: SystemTime::now(),
                    current_quote: current_quote.ok_or_else(|| serde::de::Error::missing_field("current_quote"))?,
                    current_position: current_position.ok_or_else(|| serde::de::Error::missing_field("current_position"))?,
                    is_complete: is_complete.ok_or_else(|| serde::de::Error::missing_field("is_complete"))?,
                })
            }
        }

        const FIELDS: &[&str] = &["current_quote", "current_position", "is_complete"];
        deserializer.deserialize_struct("GameState", FIELDS, GameStateVisitor)
    }
}

#[derive(Debug, Clone)]
pub struct TypingError {
    pub expected: char,
    pub received: char,
    pub position: usize,
    pub timestamp: SystemTime,
}

impl Serialize for TypingError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TypingError", 3)?;
        state.serialize_field("expected", &self.expected)?;
        state.serialize_field("received", &self.received)?;
        state.serialize_field("position", &self.position)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TypingError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Expected,
            Received,
            Position,
        }

        struct TypingErrorVisitor;

        impl<'de> serde::de::Visitor<'de> for TypingErrorVisitor {
            type Value = TypingError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct TypingError")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TypingError, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut expected = None;
                let mut received = None;
                let mut position = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Expected => expected = Some(map.next_value()?),
                        Field::Received => received = Some(map.next_value()?),
                        Field::Position => position = Some(map.next_value()?),
                    }
                }

                Ok(TypingError {
                    expected: expected.ok_or_else(|| serde::de::Error::missing_field("expected"))?,
                    received: received.ok_or_else(|| serde::de::Error::missing_field("received"))?,
                    position: position.ok_or_else(|| serde::de::Error::missing_field("position"))?,
                    timestamp: SystemTime::now(),
                })
            }
        }

        const FIELDS: &[&str] = &["expected", "received", "position"];
        deserializer.deserialize_struct("TypingError", FIELDS, TypingErrorVisitor)
    }
}

impl TypingSession {
    pub fn new(text: String) -> Self {
        let first_sentence_end = find_next_sentence_end(&text, 0);
        Self {
            quote_text: text.clone(),
            start_time: Instant::now(),
            metrics: TypingMetrics::new(),
            current_position: 0,
            is_complete: false,
        }
    }

    pub fn load_new_quote(&mut self, text: String) {
        // Save current stats before loading new quote if we have any text
        if !self.quote_text.is_empty() {
            if let Err(e) = self.metrics.save_to_json(&self.quote_text) {
                eprintln!("Failed to save stats: {}", e);
            }
        }

        // Update text and reset timing/positions
        self.quote_text = text;
        self.reset();
    }
    
    pub fn record_keystroke(&mut self, c: char) {
        let expected_char = self.quote_text.chars().nth(self.current_position).unwrap_or(' ');
        self.metrics.record_keystroke(c, expected_char, self.current_position);
        
        // Increment position if the character matches and we're not past the end
        if c == expected_char && self.current_position < self.quote_text.len() {
            self.current_position += 1;
        }
    }

    pub fn get_averages(&self) -> (f64, f64) {
        if self.metrics.keystrokes == 0 {
            (0.0, 0.0)
        } else {
            (self.metrics.wpm, self.metrics.accuracy)
        }
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.current_position = 0;
        self.metrics = TypingMetrics::new();
    }

    pub fn calculate_metrics(&mut self) {
        self.metrics.current_time = Instant::now();
        self.metrics.calculate_overall_metrics();
    }

    pub fn advance_sentence(&mut self) {
        if self.current_position < self.quote_text.len() {
            let next_end = find_next_sentence_end(&self.quote_text, self.current_position);
            self.current_position = next_end;
        }
    }
}

fn find_next_sentence_end(text: &str, start: usize) -> usize {
    let mut in_quote = false;
    let mut last_char = None;

    for (i, c) in text[start..].chars().enumerate() {
        match c {
            '"' | '"' | '"' => in_quote = !in_quote,
            '.' | '!' | '?' => {
                // Check if this is part of an ellipsis
                if c == '.' && last_char == Some('.') {
                    last_char = Some(c);
                    continue;
                }
                // If we're not in a quote and this is a sentence end, return position
                if !in_quote {
                    return start + i + 1;
                }
            }
            _ => {}
        }
        last_char = Some(c);
    }
    
    // If no sentence end found, return end of text
    text.len()
} 