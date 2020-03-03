use tracing::{Event, Subscriber};
use tracing_core::field::Field;
use tracing_serde::AsSerde;
use tracing_subscriber::fmt::{time::ChronoUtc, FmtContext, FormatFields};
use tracing_subscriber::layer::Layer;
use tracing_subscriber::{field::Visit, filter::EnvFilter, registry::LookupSpan, Registry, FmtSubscriber};

use serde_json::json;
use std::str::FromStr;

pub enum Output {
    Json,
    Log,
    Compact,
    None,
}

pub type ParseError = String;

impl FromStr for Output {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "Json" => Ok(Output::Json),
            "Log" => Ok(Output::Log),
            "Compact" => Ok(Output::Compact),
            "None" => Ok(Output::None),
            _ => Err("Could not parse log output type".into()),
        }
    }
}

pub struct EventFieldVisitor {
    json: serde_json::Map<String, serde_json::Value>,
}

impl EventFieldVisitor {
    fn new() -> Self {
        let json = serde_json::Map::new();
        EventFieldVisitor { json }
    }
}

impl Visit for EventFieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.json
            .insert(field.name().into(), json!(format!("{:?}", value)));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.json.insert(field.name().into(), json!(value));
    }
}

fn format_event<S, N>(
    ctx: &FmtContext<'_, S, N>,
    writer: &mut dyn std::fmt::Write,
    event: &Event<'_>,
) -> std::fmt::Result
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    let now = chrono::offset::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let mut parents = vec![];
    ctx.visit_spans::<(), _>(|span| {
        let meta = span.metadata();
        let name = meta.name();
        let file = meta.file();
        let line = meta.line();
        let module_path = meta.module_path();
        let level = meta.level();
        let target = meta.target();
        let id = span.id();
        let json = json!({"id": id.as_serde(), "name": name, "level": level.as_serde(), "target": target, "module_path": module_path, "file": file, "line": line});
        parents.push(json);
        Ok(())
    })
    .ok();
    let meta = event.metadata();
    let name = meta.name();
    let file = meta.file();
    let line = meta.line();
    let module_path = meta.module_path();
    let level = meta.level();
    let target = meta.target();
    let mut values = EventFieldVisitor::new();
    event.record(&mut values);
    let json = json!({"time": now, "name": name, "level": level.as_serde(), "target": target, "module_path": module_path, "file": file, "line": line, "fields": values.json, "spans": parents});
    writeln!(writer, "{}", json)
}

pub fn init_fmt(output: Output, jaeger: Option<String>) -> Result<(), String> {
    let mut filter = EnvFilter::from_default_env();
    if std::env::var("CUSTOM_FILTER").is_ok() {
        EnvFilter::try_from_env("CUSTOM_FILTER")
            .map_err(|e| eprintln!("Failed to parse CUSTOM_FILTER {:?}", e))
            .map(|f| {
                filter = f;
            })
            .ok();
    }
    let fm: fn(
        ctx: &FmtContext<'_, _, _>,
        &mut dyn std::fmt::Write,
        &Event<'_>,
    ) -> std::fmt::Result = format_event;
    let subscriber = FmtSubscriber::builder().with_env_filter(filter);

    fn finish<S>(subscriber: S, jaeger: Option<String>) -> Result<(), String>
    where
        S: Subscriber + Send + Sync + for<'span> LookupSpan<'span>,
    {
        match jaeger {
            Some(name) => {
                let layer = crate::tracing::init(name)?;
                let subscriber = layer.with_subscriber(subscriber);
                tracing::subscriber::set_global_default(subscriber).map_err(|e| format!("{:?}", e))
            }
            None => {
                tracing::subscriber::set_global_default(subscriber).map_err(|e| format!("{:?}", e))
            }
        }
    };
    match output {
        Output::Json => {
            let subscriber = subscriber
                .with_timer(ChronoUtc::rfc3339())
                .json()
                .event_format(fm);
            finish(subscriber.finish(), jaeger)
        }
        Output::Log => finish(subscriber.finish(), jaeger),
        Output::Compact => {
            let subscriber = subscriber.compact();
            finish(subscriber.finish(), jaeger)
        }
        Output::None => finish(Registry::default(), jaeger),
    }
}
