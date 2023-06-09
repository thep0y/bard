use std::{collections::HashMap, env};

use log::{Level, LevelFilter};
use simplelog::{Color, Config, ConfigBuilder};
use time::macros::format_description;

#[cfg(target_os = "linux")]
use time::macros::offset;

pub fn logger_config(is_term: bool) -> Config {
    let mut config = &mut ConfigBuilder::new();

    if is_term {
        config = config
            .set_level_color(Level::Error, Some(Color::Rgb(191, 0, 0)))
            .set_level_color(Level::Warn, Some(Color::Rgb(255, 127, 0)))
            .set_level_color(Level::Info, Some(Color::Rgb(19, 161, 14)))
            .set_level_color(Level::Debug, Some(Color::Rgb(136, 23, 152)))
            .set_level_color(Level::Trace, Some(Color::Rgb(127, 127, 255)))
            .set_time_format_custom(format_description!(
                "[hour]:[minute]:[second].[subsecond digits:3]"
            ));
    } else {
        config = config.set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ));
    }

    // time::OffsetDateTime::now_local() 方法在 POSIX 上线程不安全：https://github.com/time-rs/time/issues/457
    // 在 linux 上直接指定时区为 +8。
    #[cfg(target_os = "linux")]
    config.set_time_offset(offset!(+8));

    #[cfg(not(target_os = "linux"))]
    config.set_time_offset_to_local().unwrap();

    config
        .set_location_level(LevelFilter::Error)
        .set_thread_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .add_filter_ignore_str("tao")
        .add_filter_ignore_str("mio")
        .build()
}

pub fn log_level() -> LevelFilter {
    let level_strings: HashMap<&str, LevelFilter> = HashMap::from([
        ("0", LevelFilter::Off),
        ("1", LevelFilter::Error),
        ("2", LevelFilter::Warn),
        ("3", LevelFilter::Info),
        ("4", LevelFilter::Debug),
        ("5", LevelFilter::Trace),
    ]);

    match env::var("LOG_LEVEL") {
        Ok(v) => match level_strings.get(&v as &str) {
            Some(l) => *l,
            None => {
                if cfg!(debug_assertions) {
                    LevelFilter::Trace
                } else {
                    LevelFilter::Debug
                }
            }
        },
        Err(_) => LevelFilter::Debug,
    }
}
