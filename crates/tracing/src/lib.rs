#![feature(rustc_private)]

extern crate rustracing;
extern crate rustracing_jaeger;
#[macro_use]
extern crate shrinkwraprs;

mod span;
pub mod span_map;
pub mod stack;
pub mod tracer_console;
pub mod tracer_network;

pub use rustracing::{tag::Tag, sampler::*};
pub use rustracing_jaeger::{Tracer, reporter::JaegerCompactReporter as Reporter};

pub use crate::span::{
    HSpanContext as SpanContext,
    HSpan as Span,
    SpanWrap,
    noop,
    null_tracer,
    test_span,
    EncodedSpanContext
};
