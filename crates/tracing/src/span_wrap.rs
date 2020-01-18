use crate::span::{HSpan, NOOP_SPAN};
use crate::span_context::EncodedSpanContext;
use crate::span_context::HSpanContext;
use rustracing::sampler::*;
use rustracing::span::StartSpanOptions;
use rustracing_jaeger::Span as RjSpan;
use rustracing_jaeger::{span::SpanContextState, Tracer};
use serde::de::DeserializeOwned;
// use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

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

    pub fn follower_or_null<S: Into<Cow<'static, str>>>(
        &self,
        tracer: &Tracer,
        operation_name: S,
    ) -> HSpan {
        self.follower(tracer, operation_name)
            .unwrap_or_else(|| NOOP_SPAN.follower("noop"))
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

    /// Map the data field to a new value while keeping the same span_context
    pub fn map<F, U>(self, f: F) -> SpanWrap<U>
    where
        F: FnOnce(T) -> U,
    {
        SpanWrap {
            data: f(self.data),
            span_context: self.span_context,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SpanWrap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpanWrap({:?}, {:?})", self.data, self.span_context)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EncodedSpanWrap<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    pub data: T,
    pub span_context: Option<EncodedSpanContext>,
}

impl<T> EncodedSpanWrap<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    /// Map the data field to a new value while keeping the same span_context
    pub fn map<F, U>(self, f: F) -> EncodedSpanWrap<U>
    where
        F: FnOnce(T) -> U,
        U: Serialize + DeserializeOwned + Clone,
    {
        EncodedSpanWrap {
            data: f(self.data),
            span_context: self.span_context,
        }
    }

    /// Return new struct with new inner data and cloned context
    pub fn swapped<U>(&self, data: U) -> EncodedSpanWrap<U>
    where
        U: Serialize + DeserializeOwned + Clone,
    {
        EncodedSpanWrap {
            data,
            span_context: self.span_context.clone(),
        }
    }
}

impl<'a, T> From<SpanWrap<T>> for EncodedSpanWrap<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn from(sw: SpanWrap<T>) -> Self {
        Self {
            data: sw.data,
            span_context: match sw.span_context {
                Some(c) => c.encode().ok().or_else(|| {
                    warn!("Failed to decode SpanContext, throwing it away!");
                    None
                }),
                None => None,
            },
        }
    }
}

impl<'a, T> From<EncodedSpanWrap<T>> for SpanWrap<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn from(swe: EncodedSpanWrap<T>) -> Self {
        Self {
            data: swe.data,
            span_context: match swe.span_context {
                Some(c) => HSpanContext::decode(c).ok().or_else(|| {
                    warn!("Failed to decode SpanContext, throwing it away!");
                    None
                }),
                None => None,
            },
        }
    }
}

impl<'a, T> std::fmt::Debug for EncodedSpanWrap<T>
where
    T: Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpanWrap({:?}, {:?})", self.data, self.span_context)
    }
}

// impl<T> TryFrom<SpanWrap<T>> for EncodedSpanWrap<T>
// where T: Serialize + Deserialize + Clone {
//     type Error = rustracing_jaeger::Error;

//     fn try_from(sw: SpanWrap<T>) -> Result<Self> {
//         Ok(Self {
//             data: sw.data,
//             span_context: match sw.span_context {
//                 Some(c) => Some(c.encode()?),
//                 None => None,
//             },
//         })
//     }
// }

// impl<T> TryFrom<EncodedSpanWrap<T>> for SpanWrap<T>
// where T: Serialize + Deserialize + Clone {
//     type Error = rustracing_jaeger::Error;

//     fn try_from(swe: EncodedSpanWrap<T>) -> Result<Self> {
//         Ok(Self {
//             data: swe.data,
//             span_context: match swe.span_context {
//                 Some(c) => Some(HSpanContext::decode(&c)?),
//                 None => None,
//             },
//         })
//     }
// }
