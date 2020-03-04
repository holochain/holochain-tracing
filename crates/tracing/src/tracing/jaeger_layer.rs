use opentelemetry::api::{Span, SpanContext, Tracer};
use opentelemetry::sdk;
use tracing::{
    field::{Field, Visit},
    span, Event, Subscriber,
};
use tracing_subscriber::{layer::Context, registry::SpanRef, Layer};
pub(crate) struct JaegerLayer {
    tracer: sdk::Tracer,
}

#[derive(Clone, Debug)]
struct JaegerTrace;

struct ContextVisitor {
    context: Option<crate::EncodedSpanContext>,
}

impl ContextVisitor {
    fn _new() -> Self {
        ContextVisitor { context: None }
    }
}

impl Visit for ContextVisitor {
    fn record_debug(&mut self, _: &Field, _: &dyn std::fmt::Debug) {}
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "jaeger_follower" {
            let context: Result<crate::EncodedSpanContext, _> = serde_json::from_str(value);
            if context.is_err() {
                dbg!("Failed to deserialize");
            }
            self.context = context.ok();
        }
    }
}

impl JaegerLayer {
    pub(crate) fn new(tracer: sdk::Tracer) -> Self {
        JaegerLayer { tracer }
    }
}

impl JaegerLayer {
    pub(crate) fn start<S>(&self, name: &str, context: Option<SpanContext>, span: SpanRef<S>)
    where
        S: Subscriber,
        S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
    {
        let s = self.tracer.start(name, context);
        span.extensions_mut().replace(s.get_context()).map(|_| {
            dbg!("Context was already in span");
        });
        span.extensions_mut().replace(s).map(|_| {
            dbg!("Jaeger span was already in span");
        });
    }
}

impl<S> Layer<S> for JaegerLayer
where
    S: Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    // Submit all spans except for `jaeger_follower` that have a marked with a `SpanContext` parent.
    fn new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        /*
        if let Some(_) = attrs.metadata().fields().field("jaeger_follower") {
            let mut visitor = ContextVisitor::new();
            attrs.record(&mut visitor);
            visitor
                .context
                .and_then(|context| crate::SpanContext::decode(context).ok())
                .and_then(|context| crate::tracing::tracing_context(&context))
                .map(|context| self.start(attrs.metadata().name(), Some(context), this_span));
        } else {
            */
        if let None = attrs.metadata().fields().field("jaeger_follower") {
            let this_span = ctx
                .span(id)
                .expect("Should always be able to find self span");
            if let Some(_) = attrs.metadata().fields().field("jaeger_root") {
                self.start(attrs.metadata().name(), None, this_span);
            } else if let Some(context) = check_parents(attrs, &ctx) {
                self.start(attrs.metadata().name(), Some(context), this_span);
            } else {
                //tracing::trace!("Not in jaeger trace {}", attrs.metadata().name());
            }
        }
    }
    // Submit custom start event because this will show in jaeger when this enters the executor
    fn on_enter(&self, _id: &span::Id, _ctx: Context<'_, S>) {}
    // Submit custom start event because this will show in jaeger when this exits the executor
    fn on_exit(&self, _id: &span::Id, _ctx: Context<'_, S>) {}
    fn on_event(&self, _event: &Event<'_>, _ctx: Context<'_, S>) {}
    fn on_record(&self, _span: &span::Id, _values: &span::Record<'_>, _ctx: Context<'_, S>) {}
    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = ctx
            .span(&id)
            .expect("Should always be able to find self span");
        let extensions = span.extensions();
        if let Some(_) = extensions.get::<SpanContext>() {
            span.extensions_mut()
                .get_mut::<sdk::Span>()
                .map(|span| span.end())
                .unwrap_or_else(|| {
                    dbg!("Jaeger span was already in span");
                });
        }
    }
}

fn check_parents<S>(attrs: &span::Attributes, ctx: &Context<'_, S>) -> Option<SpanContext>
where
    S: Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    let current = ctx.current_span();
    attrs
        .parent()
        .or_else(|| current.id())
        .and_then(|parent| ctx.span(parent))
        .and_then(|span| span.extensions().get::<SpanContext>().cloned())
}
