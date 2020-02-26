use crate::structured::StructuredLayer;
use opentelemetry::{
    api::{self, Provider, Span},
    global::{self, BoxedSpan, BoxedTracer},
    sdk,
};
use tracing::Id;
use tracing_opentelemetry::OpentelemetryLayer;
use tracing_subscriber::{layer::Layered, registry::LookupSpan, Layer, Registry};

#[macro_export]
macro_rules! follow_span {
    ($name:expr, $context:expr) => {
        if let Some(context) = $context {
            // This isn't an Error but we need to make sure it always goes on the stack
            // It won't be reported to jaeger
            let follow_span = ::tracing::error_span!(
                target: module_path!(),
                parent: None,
                $name,
                follower = true
            );
            let id = follow_span.id();
            id.map(|id| $crate::tracing::__follow(&context, id));
            follow_span
        } else {
            tracing::Span::none()
        }
    };
}

// TODO figure out how to create many spans and give the resulting IDs back to the dev
#[macro_export]
macro_rules! follow_spans {
    () => {};
}

pub fn init(service_name: String, structured: bool) -> Result<(), String> {
    let exporter = opentelemetry_jaeger::Exporter::builder()
        .with_agent_endpoint("127.0.0.1:6831".parse().map_err(|e| format!("{:?}", e))?)
        .with_process(opentelemetry_jaeger::Process {
            service_name,
            tags: Vec::new(),
        })
        .init()
        .map_err(|e| format!("{:?}", e))?;
    let provider = sdk::Provider::builder()
        .with_simple_exporter(exporter)
        .with_config(sdk::Config {
            default_sampler: Box::new(sdk::Sampler::Always),
            ..Default::default()
        })
        .build();
    global::set_provider(provider);
    let tracer = global::trace_provider().get_tracer("tracing");
    let opentelemetry = OpentelemetryLayer::with_tracer(tracer);
    if structured {
        let subscriber = opentelemetry
            .and_then(StructuredLayer::new())
            .with_subscriber(Registry::default());
        tracing::subscriber::set_global_default(subscriber).map_err(|e| format!("{:?}", e))?;
    } else {
        let subscriber = opentelemetry.with_subscriber(Registry::default());
        tracing::subscriber::set_global_default(subscriber).map_err(|e| format!("{:?}", e))?;
    }
    Ok(())
}

pub fn span_context(id: &Id) -> Option<crate::SpanContext> {
    tracing::dispatcher::get_default(|d| {
        d.is::<Layered<OpentelemetryLayer<opentelemetry::global::BoxedTracer>, Registry>>();
        d.downcast_ref::<Layered<OpentelemetryLayer<BoxedTracer>, Registry>>()
            .and_then(|s| s.span(&id))
            .and_then(|s| s.extensions().get::<BoxedSpan>().map(|s| s.get_context()))
    })
    .and_then(|context| {
        let builder = rustracing_jaeger::span::SpanContextStateBuilder::new();
        let c = context.trace_id();
        format!("{:x}", c)
            .parse::<rustracing_jaeger::span::TraceId>()
            .ok()
            .map(|trace_id| {
                builder
                    .trace_id(trace_id)
                    .span_id(context.span_id())
                    .finish()
            })
    })
    .map(|context| {
        let context = rustracing::span::SpanContext::new(context, vec![]);
        crate::span_context::HSpanContext(context)
    })
}

fn __tracing_context(context: &crate::SpanContext) -> Option<api::SpanContext> {
    let context = context.0.state();
    u128::from_str_radix(&context.trace_id().to_string(), 16)
        .ok()
        .map(|trace_id| api::SpanContext::new(trace_id, context.span_id(), 1, false))
}

// This is a bit of a hack to get follow to work with opentelemetry
pub fn __follow(context: &crate::SpanContext, id: Id) -> () {
    __tracing_context(context).map(|context| {
        tracing::dispatcher::get_default(|d| {
            d.downcast_ref::<Layered<OpentelemetryLayer<BoxedTracer>, Registry>>()
                .map(|s| {
                    let span_ref = s.span(&id);
                    span_ref.map(|s| {
                        s.extensions_mut().insert(context.clone());
                    });
                })
        });
    });
}
