use tracing::Level;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::{EnvFilter, Registry};

pub fn init(level: String) -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    let format = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_target(true);

    #[cfg(not(debug_assertions))]
    let format = tracing_subscriber::fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_target(true);

    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .parse(level)?;

    let subscriber = Registry::default().with(format).with(env_filter);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
