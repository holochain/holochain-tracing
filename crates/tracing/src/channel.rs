use crossbeam_channel as cb;
use crate::{Span, SpanWrap};
use crate::stack::with_top;

#[derive(Shrinkwrap)]
pub struct SpanSender<T>(cb::Sender<SpanWrap<T>>);

impl<T> From<cb::Sender<SpanWrap<T>>> for SpanSender<T> {
    fn from(tx: cb::Sender<SpanWrap<T>>) -> SpanSender<T> {
        SpanSender(tx)
    }
}

impl<T: Send> SpanSender<T> {
    pub fn send_wrapped(&self, v: T) -> Result<(), cb::SendError<SpanWrap<T>>> {
        let span = with_top(|top| top.child("send_with")).unwrap_or_else(|| {
            warn!("Using noop span in send_with");
            Span::noop()
        });
        // .and_then(|span| {
            self.0.send(span.wrap(v))
        // })
    }
}

pub type SpanReceiver<T> = cb::Receiver<SpanWrap<T>>;