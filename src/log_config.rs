use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use crate::paths;

//const LOG_CONFIG: &str = "logging_config.yaml";

/// Based on https://github.com/estk/log4rs/blob/master/examples/log_to_file.rs
// fn main() -> Result<(), SetLoggerError> {
pub fn init_log() {
    /*
    if paths::file_exists(LOG_CONFIG) {
        let log_result = log4rs::init_file(LOG_CONFIG, Default::default());
        match log_result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to init logging: {:?}", e);
            }
        }
    }
    */

    let level = log::LevelFilter::Info;
    let file_path = paths::get_full_path(&paths::get_temp_dir(), "app-lemonator.log");
    // let file_path = "/tmp/foo.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h([ {({l}):5.5} ])} {m}{n}")))
        .target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        //.encoder(Box::new(PatternEncoder::new("{h({l})} - {m}\n")))
        .encoder(Box::new(PatternEncoder::new("[ {d(%Y-%m-%d %H:%M:%S)(utc)} | {h({l}):5.5} ] {m}{n}")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config);

    /*
    error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only");

    Ok(())
    */
}
