#![feature(rustc_private)]

extern crate backtrace;
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
mod span_context;
mod span_wrap;
mod stack;
pub mod tracer_console;
pub mod tracer_network;

pub use rustracing::{sampler::*, tag::Tag};
pub use rustracing_jaeger::{reporter::JaegerCompactReporter as Reporter, Tracer};
pub use stack::{push_span, push_span_with, with_top};
pub use channel::{SpanSender, SpanReceiver};
pub use crate::span::{ HSpan as Span, noop, null_tracer, test_span };
pub use crate::span_context::{EncodedSpanContext, HSpanContext as SpanContext};
pub use crate::span_wrap::{SpanWrap};
