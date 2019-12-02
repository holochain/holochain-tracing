use crate::rustracing::carrier::{ExtractFromBinary, InjectToBinary};
use crate::span::HSpan;
use crate::span_wrap::SpanWrap;
use rustracing::sampler::*;
use rustracing::span::StartSpanOptions;
use rustracing_jaeger::{
    span::{SpanContext, SpanContextState},
    Tracer,
};
use rustracing_jaeger::{Result, Span as RjSpan};
use std::{borrow::Cow, io::Cursor};

/// Binary representation is exactly 37 bytes, so ideally
/// we would use a [u8; 37], but this is easier...
pub type EncodedSpanContext = Vec<u8>;

/// An OpenTracing SpanContext is used to send span info across a process
/// boundary. This is a simple wrapper around that, again with some helper
/// functions.
#[derive(Clone, Debug)]
pub struct HSpanContext(pub SpanContext);

impl HSpanContext {
    /// Create a follower RjSpan from this SpanContext
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

    pub fn follower_<'a, N: Into<Cow<'static, str>>, F>(
        &'a self,
        tracer: &Tracer,
        operation_name: N,
        f: F,
    ) -> HSpan
    where
        F: FnOnce(StartSpanOptions<'_, BoxSampler<SpanContextState>, SpanContextState>) -> RjSpan,
    {
        f(tracer.span(operation_name).follows_from(&self.0)).into()
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

    /// Wrap this context in a SpanWrap along with some user data
    pub fn wrap<T>(self, data: T) -> SpanWrap<T> {
        SpanWrap::new(data, Some(self))
    }
}
