use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
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

const LOG_CONFIG: &str = "logging_config.yaml";

pub fn init_log(app_name: &str) {
    // Hardcoded config (below) can be overridden by a config file.
    if paths::file_exists(LOG_CONFIG) {
        let log_result = log4rs::init_file(LOG_CONFIG, Default::default());
        match log_result {
            Ok(_) => {
                return;
            }
            Err(e) => {
                eprintln!(
                    "Failed to init logging from settings file, '{}', using app defaults: {:?}",
                    LOG_CONFIG, e
                );
            }
        }
    }

    // Set the default log level to info for stderr (console) output.
    let level = log::LevelFilter::Info;

    // Build a stderr (console) logger.
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
        .target(Target::Stderr)
        .build();

    // Building a log file logger.
    // Want to have 5 MB log files, that can roll over 5 times (so 30 MB of logs).
    // Boy is log4rs complicated!
    let file_path = paths::get_full_path(&paths::get_temp_dir(), &format!("{}.log", app_name));

    let logfile_pattern = "[ {d(%Y-%m-%d %H:%M:%S)(utc)} | {h({l}):5.5} ] {m}{n}";

    let trigger_size = 1024 * 1024 * 5; // 5MB log size before rolling over
    let trigger = Box::new(SizeTrigger::new(trigger_size));
    let roller_pattern = paths::get_full_path(
        &paths::get_temp_dir(),
        &format!("{}_history{{}}.log", app_name),
    );
    let roller_count = 5;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(&roller_pattern, roller_count)
            .unwrap(),
    );
    let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));
    let pattern_encoder = Box::new(PatternEncoder::new(logfile_pattern));

    let logfile = RollingFileAppender::builder()
        .encoder(pattern_encoder)
        .build(file_path, compound_policy)
        .unwrap();

    // Combine the two appenders into a single config.
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
                .build(LevelFilter::Trace), // Default log file log level to traces
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config);
}
