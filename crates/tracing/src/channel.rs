use crate::stack::with_top;
use crate::SpanWrap;
use crossbeam_channel as cb;

#[derive(Clone, Shrinkwrap)]
pub struct SpanSender<T>(cb::Sender<SpanWrap<T>>);

impl<T> From<cb::Sender<SpanWrap<T>>> for SpanSender<T> {
    fn from(tx: cb::Sender<SpanWrap<T>>) -> SpanSender<T> {
        SpanSender(tx)
    }
}

impl<T: Send> SpanSender<T> {
    pub fn send_wrapped(&self, v: T) -> Result<(), cb::SendError<SpanWrap<T>>> {
        let context = with_top(|top| top.and_then(|t| t.context().clone()));
        self.0.send(SpanWrap::new(v, context))
    }
}

pub type SpanReceiver<T> = cb::Receiver<SpanWrap<T>>;
