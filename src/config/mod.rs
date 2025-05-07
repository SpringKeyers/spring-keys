use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const DEFAULT_CONFIG_FILE: &str = "springkeys.toml";

/// Main configuration structure for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Application version
    pub version: String,
    /// User preferences
    pub preferences: Preferences,
    /// UI settings
    pub ui: UiSettings,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    /// User name
    pub username: String,
    /// Difficulty level
    pub difficulty: DifficultyLevel,
    /// Sound effects enable/disable
    pub sound_enabled: bool,
}

/// UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    /// UI theme
    pub theme: String,
    /// Font size
    pub font_size: u8,
    /// Show WPM counter
    pub show_wpm: bool,
    /// Show accuracy meter
    pub show_accuracy: bool,
    /// Show error highlighting
    pub highlight_errors: bool,
}

/// Difficulty levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DifficultyLevel {
    /// Beginner level
    Beginner,
    /// Intermediate level
    Intermediate,
    /// Advanced level
    Advanced,
    /// Expert level
    Expert,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            preferences: Preferences::default(),
            ui: UiSettings::default(),
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            username: "Captain Typebeard".to_string(),
            difficulty: DifficultyLevel::Beginner,
            sound_enabled: true,
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: "classic".to_string(),
            font_size: 14,
            show_wpm: true,
            show_accuracy: true,
            highlight_errors: true,
        }
    }
}

impl Config {
    /// Load configuration from the specified file path
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(&path)
            .map_err(|e| ConfigError::IoError(e, path.as_ref().to_path_buf()))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e))?;
        
        Ok(config)
    }
    
    /// Save configuration to the specified file path
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(&self)
            .map_err(|e| ConfigError::SerializeError(e))?;
        
        fs::write(&path, content)
            .map_err(|e| ConfigError::IoError(e, path.as_ref().to_path_buf()))?;
        
        Ok(())
    }
    
    /// Load configuration or create default if not found
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::load(&path).unwrap_or_else(|_| {
            let config = Config::default();
            let _ = config.save(&path);
            config
        })
    }
}

/// Configuration error types
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// IO error
    #[error("IO error: {0}, path: {1}")]
    IoError(io::Error, PathBuf),
    
    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(#[from] toml::de::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializeError(#[from] toml::ser::Error),
} 