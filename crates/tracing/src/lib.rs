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
#[macro_use]
pub mod tracing;
mod utils;

pub use rustracing::{sampler::*, tag::Tag};
pub use rustracing_jaeger::{reporter, span::FinishedSpan, Tracer};
pub use span::{noop, null_tracer, test_span, HSpan as Span};
pub use span_context::{EncodedSpanContext, HSpanContext as SpanContext};
pub use span_wrap::{test_wrap, test_wrap_enc, EncodedSpanWrap, SpanWrap};
pub use stack::{is_empty, push_span, push_span_with, top_follower, with_top, with_top_or_null};
pub use tag::debug_tag;
pub use utils::{follow, follow_encoded, follow_encoded_tag, wrap, wrap_with_tag};
