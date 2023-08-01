use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
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

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
        .target(Target::Stderr)
        .build();

    // Logging to log file.
    let file_path = paths::get_full_path(&paths::get_temp_dir(), "app-lemonator{}.log");
    // let file_path = "/tmp/foo.log";

    let logfile_pattern = "[ {d(%Y-%m-%d %H:%M:%S)(utc)} | {h({l}):5.5} ] {m}{n}";

    /*
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(logfile_pattern)))
        .build(file_path)
        .unwrap();
    */

    // Rolling log file
    // https://stackoverflow.com/a/56347061/17461
    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = FixedWindowRoller::builder()
        //.build("log{}", window_size)
        // .build(&file_path, window_size)
        .build("app-lemonator{}.log", window_size)
        .unwrap();
    // let size_limit = 5 * 1024; // 5KB as max log file size to roll
    let size_limit = 512 * 1024; // 512KB as max log file size to roll
    //let size_limit = 5 * 1024 * 1024; // 512KB as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);
    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));
    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(logfile_pattern)))
        .build("logfile", Box::new(compound_policy))
        .unwrap();
    /*
    // let trigger_size = byte_unit::n_mb_bytes!(30) as u64;
    let trigger_size = 1024 * 1024 * 30;
    let trigger = Box::new(SizeTrigger::new(trigger_size));

    let roller_pattern = "logs/step/step_{}.gz";
    let roller_count = 5;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .unwrap(),
    );

    let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

    let logfile = RollingFileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        //.encoder(Box::new(PatternEncoder::new("{h({l})} - {m}\n")))
        .encoder(Box::new(PatternEncoder::new(
            "[ {d(%Y-%m-%d %H:%M:%S)(utc)} | {h({l}):5.5} ] {m}{n}",
        )))
        .build(file_path, compound_policy)
        .unwrap();
    */
    */

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
