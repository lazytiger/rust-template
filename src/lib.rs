use derive_more::{Display, Error};
use tracing::metadata::LevelFilter;

#[derive(Error, Debug, Display)]
pub struct OptionIsNone;

pub trait OptionToResult<T> {
    fn to_ok(self) -> Result<T, anyhow::Error>;
}

impl<T> OptionToResult<T> for Option<T> {
    fn to_ok(self) -> Result<T, anyhow::Error> {
        self.ok_or_else(|| OptionIsNone.into())
    }
}

const LOG_LEVEL: LevelFilter = LevelFilter::INFO;

#[cfg(target_os = "android")]
const APP_NAME: &str = "guardns";

#[cfg(target_os = "android")]
pub fn init_tracing() -> anyhow::Result<()> {
    fn tracing_level_filter(level: LevelFilter) -> log::LevelFilter {
        match level {
            LevelFilter::DEBUG => log::LevelFilter::Debug,
            LevelFilter::TRACE => log::LevelFilter::Trace,
            LevelFilter::INFO => log::LevelFilter::Info,
            LevelFilter::WARN => log::LevelFilter::Warn,
            LevelFilter::ERROR => log::LevelFilter::Error,
            LevelFilter::OFF => log::LevelFilter::Off,
        }
    }

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(tracing_level_filter(LOG_LEVEL))
            .with_tag(APP_NAME),
    );
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn init_tracing() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(LOG_LEVEL)
        .with_file(true)
        .with_line_number(true)
        .with_writer(std::io::stdout)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
