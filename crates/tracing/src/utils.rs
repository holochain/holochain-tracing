use crate::{
    push_span, stack::SpanStackGuard, with_top_or_null, EncodedSpanContext, Span, SpanContext,
    SpanWrap, Tag, Tracer,
};
/// Add a span to the stack that follows from a SpanWrap
/// # Example
/// ```
/// # use holochain_tracing::{null_tracer, Span, follow};
/// # let tracer = null_tracer();
/// # let data = 1;
/// # let span = Span::from(tracer.span("Following from some incoming span wrap").start());
/// # let tracer = Some(tracer);
/// # let span_wrap = span.wrap(data);
/// let _spanguard = follow(&tracer, &span_wrap, "func_name".into());
/// ```
pub fn follow<T>(
    tracer: &Option<Tracer>,
    span_wrap: &SpanWrap<T>,
    name: String,
) -> Option<SpanStackGuard> {
    tracer
        .as_ref()
        .map(|t| {
            let root_span = span_wrap
                .span_context
                .as_ref()
                .map(|c| c.follower(&t, name));
            root_span.map(|span| push_span(span))
        })
        .flatten()
}

pub fn follow_encoded(
    tracer: &Option<Tracer>,
    span_context: &EncodedSpanContext,
    name: String,
) -> Option<SpanStackGuard> {
    follow_encoded_tag_inner(tracer, span_context, name, None)
}

pub fn follow_encoded_tag(
    tracer: &Option<Tracer>,
    span_context: &EncodedSpanContext,
    name: String,
    tag: Tag,
) -> Option<SpanStackGuard> {
    follow_encoded_tag_inner(tracer, span_context, name, Some(tag))
}

fn follow_encoded_tag_inner(
    tracer: &Option<Tracer>,
    span_context: &EncodedSpanContext,
    name: String,
    tag: Option<Tag>,
) -> Option<SpanStackGuard> {
    tracer.as_ref().and_then(|t| {
        let root_span = SpanContext::decode(span_context.clone()).ok().map(|c| {
            c.follower_(&t, name, |options| {
                if let Some(tag) = tag {
                    options.tag(tag).start()
                } else {
                    options.start()
                }
            })
        });
        root_span.map(|span| push_span(span))
    })
}

pub fn wrap<T>(data: T, name: String) -> SpanWrap<T> {
    wrap_with_tag_inner(data, name, None)
}

pub fn wrap_with_tag<T>(data: T, name: String, tag: Tag) -> SpanWrap<T> {
    wrap_with_tag_inner(data, name, Some(tag))
}

fn wrap_with_tag_inner<T>(data: T, name: String, tag: Option<Tag>) -> SpanWrap<T> {
    with_top_or_null(|top| {
        let child: Span = top
            .follower_(name, |options| {
                if let Some(tag) = tag {
                    options.tag(tag).start()
                } else {
                    options.start()
                }
            })
            .into();
        child.wrap(data)
    })
}
