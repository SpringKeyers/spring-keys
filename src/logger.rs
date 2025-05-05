use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
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
    let mut config_builder = Config::builder();
    let mut root_builder = Root::builder();

    // Only add file logging, no stdout
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
    Ok(())
} 