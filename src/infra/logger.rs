mod layer;

use crate::infra::logger::layer::JsonLogLayer;
use std::{io, sync::Mutex};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{self, EnvFilter};

pub fn init() {
    tracing_subscriber::registry()
        .with(JsonLogLayer::new(Mutex::new(io::stdout())))
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init()
}
