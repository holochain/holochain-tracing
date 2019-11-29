
use rustracing::sampler::*;
use rustracing::span::StartSpanOptions;
use rustracing_jaeger::{
    span::{SpanContextState},
    Tracer,
};
use std::{borrow::Cow};
use rustracing_jaeger::{Span as RjSpan};
use crate::span::HSpan;
use crate::span_context::HSpanContext;

/// SpanWrap is a simple way to couple some data along with a struct. It is
/// common to send some data on a channel which will be used as arguments
/// to a function on the receiving side, where we also want to continue the
/// trace on the receiving side. This struct helps keep that data together
/// with minimal boilerplate.
///
/// The use of shrinkwrap allows the entire struct to be used as if it were
/// a bare T (in most situations), but the RjSpan can also be extracted.
#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct SpanWrap<T> {
    #[shrinkwrap(main_field)]
    pub data: T,
    pub span_context: Option<HSpanContext>,
}

impl<T> SpanWrap<T> {
    pub fn new(data: T, span_context: Option<HSpanContext>) -> Self {
        Self { data, span_context }
    }

    pub fn follower<S: Into<Cow<'static, str>>>(
        &self,
        tracer: &Tracer,
        operation_name: S,
    ) -> Option<HSpan> {
        self.span_context
            .as_ref()
            .map(|context| context.follower(tracer, operation_name))
    }

    pub fn follower_<'a, N: Into<Cow<'static, str>>, F>(
        &'a self,
        tracer: &Tracer,
        operation_name: N,
        f: F,
    ) -> Option<HSpan>
    where
        F: FnOnce(StartSpanOptions<'_, BoxSampler<SpanContextState>, SpanContextState>) -> RjSpan,
    {
        self.span_context
            .as_ref()
            .map(|context| context.follower_(tracer, operation_name, f))
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SpanWrap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpanWrap({:?}, {:?})", self.data, self.span_context)
    }
}
