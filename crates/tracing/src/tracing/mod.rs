use opentelemetry::{
    api::{self, Provider},
    global::{self},
    sdk,
};
use tracing::Subscriber;
use tracing_subscriber::{registry::LookupSpan, Layer, Registry};

mod jaeger_layer;

use jaeger_layer::JaegerLayer;

/// Allows following from a cross boundary context

pub(crate) fn init<S>(service_name: String) -> Result<impl Layer<S>, String>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
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
    let tracer = provider.get_tracer("tracing");
    global::set_provider(provider);
    let jaeger_layer = JaegerLayer::new(tracer);
    Ok(jaeger_layer)
}

pub fn span_context(span: &tracing::Span) -> Option<crate::SpanContext> {
    tracing::dispatcher::get_default(|dispatch| {
        dispatch
            .downcast_ref::<Registry>()
            .or_else(|| {
                dbg!("Failed to downcast dispatch");
                None
            })
            .and_then(|s| {
                s.span(&span.id().expect("Failed to get span id"))
                    .and_then(|span_ref| {
                        span_ref
                            .extensions()
                            .get::<api::SpanContext>()
                            .or_else(|| {
                                dbg!("Context was already in span");
                                None
                            })
                            .cloned()
                    })
            })
    })
    .or_else(|| {
        dbg!("Failed to get dispatch");
        None
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

/*
pub fn span_context(id: &Id) -> Option<crate::SpanContext> {
    tracing::dispatcher::get_default(|d| {
        d.is::<Layered<JaegerLayer, Registry>>();
        d.downcast_ref::<Layered<JaegerLayer, Registry>>()
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
*/

pub(crate) fn tracing_context(context: &crate::SpanContext) -> Option<api::SpanContext> {
    let context = context.0.state();
    u128::from_str_radix(&context.trace_id().to_string(), 16)
        .map_err(|e| {
            dbg!(&e, "Failed to create follower from context");
            e
        })
        .ok()
        .map(|trace_id| api::SpanContext::new(trace_id, context.span_id(), 1, false))
}

#[doc(hidden)]
pub fn __follow(span: &tracing::Span, context: &crate::SpanContext) {
    let context = tracing_context(context);
    tracing::dispatcher::get_default(|dispatch| {
        dispatch
            .downcast_ref::<Registry>()
            .or_else(|| {
                dbg!("Failed to downcast dispatch");
                None
            })
            .map(|s| {
                context.as_ref().map(|context| {
                    s.span(&span.id().expect("Failed to get id"))
                        .map(|span_ref| {
                            span_ref.extensions_mut().replace(context.clone()).map(|_| {
                                dbg!("Context was already in span");
                            });
                        });
                });
            });
    });
}

/*
pub fn __follow(id: &Id, context: &api::SpanContext, dispatch: &Dispatch) -> Option<()> {
    dispatch
        .downcast_ref::<Registry>()
        .or_else(|| {
            dbg!("Failed to downcast dispatch");
            None
        })
        .and_then(|s| {
            let span_ref = s.span(&id);
            span_ref
                .map(|s| start_follower(context, dispatch, s))
                .or_else(|| {
                    dbg!("Failed to find follow span");
                    None
                })
        })
}

fn start_follower(context: &api::SpanContext, dispatch: &Dispatch, span: SpanRef<Registry>) {
    dispatch
        .downcast_ref::<JaegerLayer>()
        .or_else(|| {
            dbg!("Failed to downcast to jaeger layer");
            None
        })
        .map(|s| s.start("follower", Some(context.clone()), span));
}
*/
