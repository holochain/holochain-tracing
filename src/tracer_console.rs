use crate::{Reporter, Tracer, span_map::*};
use rustracing::span::SpanReceiver;
use rustracing_jaeger::span::SpanContextState;
use std::{borrow::Cow, io::Cursor};
use std::{thread, time::Duration};

pub use rustracing::sampler::*;
pub use rustracing_jaeger::{Result, Span as RtSpan, *};
use std::collections::HashMap;

#[derive(Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
#[shrinkwrap(unsafe_ignore_visibility)]
pub struct ConsoleTracer {
    #[shrinkwrap(main_field)]
    inner: Tracer,
    span_rx: crossbeam_channel::Receiver<FinishedSpan>,
    span_map: SpanMap,
}

impl ConsoleTracer {
    /// Create a Tracer that sends all spans to a jaeger reporter thread
    pub fn new() -> Self {
        let (span_tx, span_rx) = crossbeam_channel::bounded(1000);
        let tracer = Tracer::with_sender(AllSampler, span_tx);
        ConsoleTracer {inner: tracer, span_rx, span_map: HashMap::new()}
    }

    // Delete all stored spans
    pub fn clear(&mut self) {
        let _ = self.drain();
        self.span_map.clear();
    }

    // Drain recv spans and add to map
    pub fn drain(&mut self) -> u32 {
        let mut count = 0;
        while let Ok(span) = self.span_rx.try_recv() {
            count += 1;
            self.span_map.insert(span.context().state().span_id(), span);
        }
        count
    }

    // Print span_map to console
    pub fn print(&self) {
        print_span_map(&self.span_map);
    }
}