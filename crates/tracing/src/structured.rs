use tracing::{Event, Subscriber};
use tracing_serde::AsSerde;
use tracing_subscriber::fmt::{FmtContext, FormatFields, time::ChronoUtc};
use tracing_subscriber::{filter::EnvFilter, registry::LookupSpan, FmtSubscriber};

use serde_json::json;
use std::str::FromStr;

pub enum Output {
    Json,
    Log,
    Compact,
}

pub type ParseError = String;

impl FromStr for Output {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "Json" => Ok(Output::Json),
            "Log" => Ok(Output::Log),
            "Compact" => Ok(Output::Compact),
            _ => Err("Could not parse log output type".into()),
        }
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
        let id = span.id();
        parents.push(json!({"id": id.as_serde(), "metadata": meta.as_serde()}));
        Ok(())
    })
    .ok();
    let json = json!({"time": now, "event": event.as_serde(), "parents": parents});
    /*
    let values: Option<serde_json::Map<String, serde_json::Value>> = json.as_object()
        .and_then(|o| o.get("event").and_then(|e| e.as_object()))
    .map(|e: &serde_json::Map<String, serde_json::Value>| {
        e.iter()
            .filter(|(key, _)| key.as_str() != "metadata")
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    });
    let ref module = json["event"]["metadata"]["module_path"];
    let ref file = json["event"]["metadata"]["file"];
    let ref line  = json["event"]["metadata"]["line"];
    json!({"module": module});
    */
    let ser = serde_value_flatten::to_flatten_maptree("-", None, &json).unwrap();
    writeln!(writer, "{}", json!(ser))
}

pub fn init_fmt(output: Output) -> Result<(), String> {
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
    match output {
        Output::Json => {
            let subscriber = subscriber.with_timer(ChronoUtc::rfc3339()).json().event_format(fm);
            tracing::subscriber::set_global_default(subscriber.finish())
                .map_err(|e| format!("{:?}", e))
        }
        Output::Log => tracing::subscriber::set_global_default(subscriber.finish())
            .map_err(|e| format!("{:?}", e)),
        Output::Compact => {
            let subscriber = subscriber.compact();
            tracing::subscriber::set_global_default(subscriber.finish())
                .map_err(|e| format!("{:?}", e))
        }
    }
}
