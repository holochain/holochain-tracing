#![feature(rustc_private)]

pub mod span_map;
pub mod tracer_console;
pub mod tracer_network;

extern crate crossbeam_channel;
extern crate rustracing;
extern crate rustracing_jaeger;
#[macro_use]
extern crate shrinkwraprs;

use crate::rustracing::carrier::{ExtractFromBinary, InjectToBinary};
use rustracing::span::StartSpanOptions;
use rustracing_jaeger::span::SpanContextState;
use std::{borrow::Cow, io::Cursor};

pub use rustracing::sampler::*;
pub use rustracing_jaeger::{Result, Span as RjSpan, *};

pub type SpanContext = rustracing_jaeger::span::SpanContext;
pub type Tracer = rustracing_jaeger::Tracer;
pub type Reporter = rustracing_jaeger::reporter::JaegerCompactReporter;
pub type Tag = rustracing::tag::Tag;
pub type Span = HSpan;

/// A wrapper around a simple rustracing_jaeger::RtSpan, providing some
/// convenience functions.
/// It overshadows the lower-level `child` and `follower` methods
/// with simpler versions. To access the lower-level methods, use `.0`.
#[derive(Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct HSpan(pub RjSpan);

impl From<RjSpan> for HSpan {
    fn from(span: RjSpan) -> HSpan {
        HSpan(span)
    }
}

impl HSpan {
    pub fn event<S: Into<Cow<'static, str>>>(&mut self, msg: S) {
        self.0.log(|l| {
            l.std().event(msg);
        })
    }

    pub fn error<S: Into<Cow<'static, str>>>(&mut self, kind: S, msg: S) {
        self.0.log(|l| {
            l.error().kind(kind).message(msg);
        })
    }

    pub fn context(&self) -> Option<HSpanContext> {
        self.0.context().map(|ctx| HSpanContext(ctx.to_owned()))
    }

    /// Renaming of underlying `child` method
    pub fn child_<'a, N: Into<Cow<'static, str>>, F, C, T>(
        &'a self,
        operation_name: N,
        f: F,
    ) -> RjSpan
    where
        F: FnOnce(StartSpanOptions<'_, AllSampler, SpanContextState>) -> RjSpan,
    {
        self.0.child(operation_name, f)
    }

    /// Renaming of underlying `follow` method
    pub fn follower_<'a, N: Into<Cow<'static, str>>, F, C, T>(
        &'a self,
        operation_name: N,
        f: F,
    ) -> RjSpan
    where
        F: FnOnce(StartSpanOptions<'_, AllSampler, SpanContextState>) -> RjSpan,
    {
        self.0.follower(operation_name, f)
    }

    /// Call underlying `child` method with only a simple operation name
    pub fn child<S: Into<Cow<'static, str>>>(&self, operation_name: S) -> Self {
        self.0.child(operation_name, |o| o.start()).into()
    }

    /// Call underlying `follower` method with only a simple operation name
    pub fn follower<S: Into<Cow<'static, str>>>(&self, operation_name: S) -> Self {
        self.0.follower(operation_name, |o| o.start()).into()
    }

    /// Wrap this span in a SpanWrap along with some user data
    pub fn wrap<T>(self, data: T) -> SpanWrap<T> {
        SpanWrap::new(data, self)
    }

    /// e.g. for times when a function requires a RtSpan but we don't desire to actually
    /// instrument that function call.
    pub fn noop() -> Self {
        noop("no-op, intentionally disconnected RtSpan".into())
    }

    /// Useful for retrofitting existing codebases with traces. This is a noop,
    /// but signals intent to eventually hook this up in a meaningful way,
    /// perhaps requiring some restructuring of the underlying code to make
    /// hookup possible.
    pub fn todo(reason: &'static str) -> Self {
        noop(format!("TODO: {}", reason))
    }

    /// Like todo(), but lazier. There is no reason why this span can't be
    /// hooked up other than lack of programmer time. This signals that
    /// it'll be simple to hook up whenever you get around it.
    pub fn fixme() -> Self {
        noop("not yet hooked up".into())
    }
}

/// SpanWrap is a simple way to couple some data along with a struct It is
/// common to send some data on a channel which will be used as arguments
/// to a function on the receiving side, where we also want to continue the
/// trace on the receiving side. This struct helps keep that data together
/// with minimal boilerplate.
///
/// The use of shrinkwrap allows the entire struct to be used as if it were
/// a bare T (in most situations), but the RtSpan can also be extracted.
#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct SpanWrap<T> {
    #[shrinkwrap(main_field)]
    pub data: T,
    pub span: HSpan,
}

impl<T> SpanWrap<T> {
    pub fn new(data: T, span: HSpan) -> Self {
        Self { data, span }
    }
}

/// Binary representation is exactly 37 bytes, so ideally
/// we would use a [u8; 37], but this is easier...
pub type EncodedSpanContext = Vec<u8>;

/// An OpenTracing SpanContext is used to send span info across a process
/// boundary. This is a simple wrapper around that, again with some helper
/// functions.
pub struct HSpanContext(pub SpanContext);

impl HSpanContext {
    /// Create a follower RtSpan from this SpanContext
    /// NB: there is intentionally no method to create a child span from a context,
    /// since it's assumed that all inter-process points of a trace are async and
    /// the parent span will have ended before this one does
    pub fn follower<S: Into<Cow<'static, str>>>(
        &self,
        tracer: &Tracer,
        operation_name: S,
    ) -> HSpan {
        tracer
            .span(operation_name)
            .follows_from(&self.0)
            .start()
            .into()
    }

    /// Serialize to binary format for packing into a IPC message
    pub fn encode(&self) -> Result<EncodedSpanContext> {
        let mut enc: Vec<u8> = [0; 37].to_vec(); // OpenTracing binary format is 37 bytes
        let mut slice = &mut enc[..];
        SpanContextState::inject_to_binary(&self.0, &mut slice)?;
        Ok(enc)
    }

    /// Deserialize from binary format
    pub fn decode(enc: &EncodedSpanContext) -> Result<Self> {
        let mut cursor = Cursor::new(enc);
        SpanContextState::extract_from_binary(&mut cursor).map(|x| HSpanContext(x.unwrap()))
    }
}

/// Tracer placeholder (use only as last resort)
pub fn null_tracer() -> Tracer {
    Tracer::new(NullSampler).0
}

/// TODO: use lazy_static / thread_local singleton Tracer
fn noop(name: String) -> HSpan {
    null_tracer().span(name).start().into()
}

/// Dummy span, useful for tests that don't test tracing
pub fn test_span(name: &str) -> HSpan {
    noop(name.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_test() {
        // Creates a tracer
        let (tracer, mut reporter) = tracer_console::new_tracer_with_console_reporter();
        {
            // Starts "parent" span
            let parent_span: HSpan = tracer.span("parent").start().into();
            {
                // Starts "child" span
                let mut child_span = parent_span.child("child_span");
                child_span.set_tag(|| Tag::new("id", "A"));
                child_span.event("a log message");
                std::thread::sleep(std::time::Duration::from_millis(10));
                let mut child_b_span = parent_span.child("child_b_span");
                child_b_span.set_tag(|| Tag::new("id", "B"));
                let mut _grand_child_span = child_span.child("grand_child_span");
                // Starts "follower" span
                let mut _follower_span = child_span.follower("child_follower_span");
            } // The "child" span dropped and will be sent to `span_rx`
            let parent_follower_span = parent_span.follower("parent_follower_span");
            // std::thread::sleep(std::time::Duration::from_millis(10));
            let mut _parent_follower_2_span =
                parent_follower_span.follower("parent_follower_2_span");
        } // The "parent" span dropped and will be sent to `span_rx`

        // Outputs finished spans to the standard output
        let count = reporter.drain();
        assert_eq!(7, count);
        reporter.print(false);
        println!("Debug output:\n {:?}", reporter);
    }
}
