#![feature(rustc_private)]

extern crate rustracing;
extern crate rustracing_jaeger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate shrinkwraprs;
#[macro_use]
extern crate log;

mod channel;
mod span;
pub mod span_map;
mod stack;
pub mod tracer_console;
pub mod tracer_network;

pub use rustracing::{sampler::*, tag::Tag};
pub use rustracing_jaeger::{reporter::JaegerCompactReporter as Reporter, Tracer};
pub use stack::{push_root_span, push_span_with};
pub use channel::{SpanSender, SpanReceiver};
pub use crate::span::{
    noop, null_tracer, test_span, EncodedSpanContext, HSpan as Span, HSpanContext as SpanContext,
    SpanWrap,
};
