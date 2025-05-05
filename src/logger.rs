use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::path::Path;

/// Initialize the application logger
pub fn init_logger<P: AsRef<Path>>(
    log_level: LevelFilter,
    log_file: Option<P>,
) -> Result<(), SetLoggerError> {
    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}] [{l}] - {m}\n",
        )))
        .build();

    let mut config_builder = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log_level)))
                .build("stdout", Box::new(stdout)),
        );

    let mut root_builder = Root::builder().appender("stdout");

    // Add file logging if requested
    if let Some(log_file_path) = log_file {
        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "[{d(%Y-%m-%d %H:%M:%S)}] [{l}] - {m}\n",
            )))
            .build(log_file_path)
            .expect("Failed to create log file appender");

        config_builder = config_builder.appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log_level)))
                .build("file", Box::new(file)),
        );

        root_builder = root_builder.appender("file");
    }

    let config = config_builder
        .build(root_builder.build(log_level))
        .expect("Failed to create logging configuration");

    log4rs::init_config(config)?;

    log::info!("Logging system initialized at level: {}", log_level);
    Ok(())
}

/// Get the appropriate log level from a string
pub fn get_log_level(level: &str) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info, // Default
    }
} 