use crate::Result;
use crate::CONFIG;
use time::format_description::BorrowedFormatItem;
use time::{macros::format_description, UtcOffset};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

pub fn register_log() -> Result<WorkerGuard> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&CONFIG.log.level));
    let log_time = get_log_time();
    let formatting_layer = fmt::layer()
        .with_file(true)
        .with_timer(log_time.clone())
        .pretty()
        .with_writer(std::io::stdout);

    // file format
    let file_appender = rolling::daily("logs", "app.log");

    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_file(true)
        .with_timer(log_time)
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(file_layer)
        .init();
    Ok(guard)
}

#[inline]
fn get_log_time() -> OffsetTime<&'static [BorrowedFormatItem<'static>]> {
    let offset = UtcOffset::from_hms(8, 0, 0).unwrap_or(UtcOffset::UTC);
    let log_time = OffsetTime::new(
        offset,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );
    log_time
}