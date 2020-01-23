extern crate backtrace;
extern crate rustracing;
extern crate rustracing_jaeger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate shrinkwraprs;
#[macro_use]
extern crate log;
// #[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod channel;
mod span;
mod span_context;
mod span_wrap;
mod stack;
mod tag;
pub mod tracer_console;
pub mod tracer_network;

pub use rustracing::{sampler::*, tag::Tag};
pub use rustracing_jaeger::{reporter, Tracer, span::FinishedSpan};
pub use span::{null_tracer, test_span, HSpan as Span, noop};
pub use span_context::{EncodedSpanContext, HSpanContext as SpanContext};
pub use span_wrap::{SpanWrap, EncodedSpanWrap};
pub use stack::{is_empty, push_span, push_span_with, with_top, with_top_or_null, top_follower};
pub use tag::debug_tag;
