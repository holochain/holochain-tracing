use crate::{span_map::*, Tracer};

pub use rustracing::sampler::*;
pub use rustracing_jaeger::{Result, Span as RtSpan, *};
use std::collections::HashMap;

/// A Tracer wrapper that stores all spans it receives in a map,
/// with the intent to display all received spans in the console.
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
    /// Create a Tracer that self-consumes all spans and reports them in ASCII form
    pub fn new() -> Self {
        let (span_tx, span_rx) = crossbeam_channel::bounded(1000);
        let tracer = Tracer::with_sender(AllSampler, span_tx);
        ConsoleTracer {
            inner: tracer,
            span_rx,
            span_map: HashMap::new(),
        }
    }

    /// Delete all stored spans
    pub fn clear(&mut self) {
        let _ = self.drain();
        self.span_map.clear();
    }

    /// Drain `span_rx` and add to map
    /// TODO: Could be done periodically in a separate thread
    pub fn drain(&mut self) -> u32 {
        let mut count = 0;
        while let Ok(span) = self.span_rx.try_recv() {
            count += 1;
            self.span_map.insert(span.context().state().span_id(), span);
        }
        count
    }

    /// Print span_map to console
    pub fn print(&self, only_events: bool) {
        print_span_map(&self.span_map, only_events);
    }
}
