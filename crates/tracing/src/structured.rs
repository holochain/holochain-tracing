use tracing::{Event, Subscriber};
use tracing_serde::AsSerde;
use tracing_subscriber::layer::Context;
use tracing_subscriber::{Layer, Registry};

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
        println!("<SL< {}; >SL>", json);
    }
}

pub fn init() -> Result<(), String> {
    let subscriber = StructuredLayer::new()
        .with_subscriber(Registry::default());
    tracing::subscriber::set_global_default(subscriber).map_err(|e | format!("{:?}", e))
}
