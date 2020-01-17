use crate::{stack::with_top, EncodedSpanWrap, SpanWrap};
use crossbeam_channel as cb;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone, Shrinkwrap)]
pub struct SpanSender<T>(cb::Sender<SpanWrap<T>>);

#[derive(Clone, Shrinkwrap)]
pub struct EncodedSpanSender<T: Serialize + DeserializeOwned + Clone>(
    cb::Sender<EncodedSpanWrap<T>>,
);

impl<T> From<cb::Sender<SpanWrap<T>>> for SpanSender<T> {
    fn from(tx: cb::Sender<SpanWrap<T>>) -> SpanSender<T> {
        SpanSender(tx)
    }
}

impl<T: Send + DeserializeOwned + Serialize + Clone> From<cb::Sender<EncodedSpanWrap<T>>> for EncodedSpanSender<T> {
    fn from(tx: cb::Sender<EncodedSpanWrap<T>>) -> EncodedSpanSender<T> {
        EncodedSpanSender(tx)
    }
}

impl<T: Send> SpanSender<T> {
    pub fn send_wrapped(&self, v: T) -> Result<(), cb::SendError<SpanWrap<T>>> {
        let context = with_top(|top| top.and_then(|t| t.context().clone()));
        self.0.send(SpanWrap::new(v, context))
    }
}

impl<T: Send + DeserializeOwned + Serialize + Clone> EncodedSpanSender<T> {
    pub fn send_wrapped(&self, v: T) -> Result<(), cb::SendError<EncodedSpanWrap<T>>> {
        let context =
            with_top(|top| top.and_then(|t| t.context().clone()));
        self.0.send(SpanWrap::new(v, context).into())
    }
}

pub type SpanReceiver<T> = cb::Receiver<SpanWrap<T>>;
pub type EncodedSpanReceiver<T> = cb::Receiver<EncodedSpanWrap<T>>;

pub fn lax_send_wrapped<T: Send + std::fmt::Debug>(
    tx: SpanSender<T>,
    val: T,
    _failure_reason: &str,
) -> bool {
    match tx.send_wrapped(val) {
        Ok(()) => true,
        Err(_) => {
            // println!("[lax_send]\n{}\n{:?}\n", _failure_reason, val);
            false
        }
    }
}
