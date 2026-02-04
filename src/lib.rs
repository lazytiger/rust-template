use derive_more::{Display, Error};

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

#[cfg(any(target_os = "ios", target_os = "android"))]
const LOG_LEVEL: tracing::metadata::LevelFilter = tracing::metadata::LevelFilter::INFO;

#[cfg(target_os = "android")]
pub fn init_tracing() -> anyhow::Result<()> {
    fn tracing_level_filter(level: LevelFilter) -> log::LevelFilter {
        match level {
            tracing::metadata::LevelFilter::DEBUG => log::LevelFilter::Debug,
            tracing::metadata::LevelFilter::TRACE => log::LevelFilter::Trace,
            tracing::metadata::LevelFilter::INFO => log::LevelFilter::Info,
            tracing::metadata::LevelFilter::WARN => log::LevelFilter::Warn,
            tracing::metadata::LevelFilter::ERROR => log::LevelFilter::Error,
            tracing::metadata::LevelFilter::OFF => log::LevelFilter::Off,
        }
    }

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(tracing_level_filter(LOG_LEVEL))
            .with_tag("{{crate_name}}"),
    );
    Ok(())
}

#[cfg(not(target_os = "android"))]
pub fn init_tracing() -> anyhow::Result<()> {
    let builder = tracing_subscriber::fmt::Subscriber::builder();
    #[cfg(target_os = "ios")]
    let builder = builder.with_max_level(LOG_LEVEL);
    #[cfg(not(target_os = "ios"))]
    let builder = builder.with_env_filter(tracing_subscriber::EnvFilter::from_default_env());
    let subscriber = builder
        .with_file(true)
        .with_line_number(true)
        .with_writer(std::io::stdout)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
