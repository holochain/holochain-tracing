use tracing::{Event, Subscriber};
use tracing_serde::AsSerde;
use tracing_subscriber::layer::Context;
use tracing_subscriber::{
    filter::{EnvFilter},
    FmtSubscriber, Layer, Registry,
};

use serde_json::json;
pub struct StructuredLayer;

impl StructuredLayer {
    pub fn new() -> Self {
        StructuredLayer {}
    }
}

impl<S> Layer<S> for StructuredLayer
where
    S: Subscriber,
{
    /// Record serde logs
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let json = json!(event.as_serde());
        println!("{};", json);
    }
}

pub fn init() -> Result<(), String> {
    let subscriber = StructuredLayer::new().with_subscriber(Registry::default());
    tracing::subscriber::set_global_default(subscriber).map_err(|e| format!("{:?}", e))
}

pub fn init_fmt() -> Result<(), String> {
    let filter = EnvFilter::try_from_env("CUSTOM_FILTER")
        .map_err(|e| eprintln!("Failed to parse CUSTOM_FILER {:?}", e));
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .json();
    match filter {
        Ok(filter) => {
            let subscriber = subscriber.with_env_filter(filter).finish();
            tracing::subscriber::set_global_default(subscriber).map_err(|e| format!("{:?}", e))
        }
        Err(_) => {
            tracing::subscriber::set_global_default(subscriber.finish()).map_err(|e| format!("{:?}", e))
        }
    }
}
