use crate::{span_map::*, Tracer};

pub use rustracing::sampler::AllSampler;
use std::collections::HashMap;

/// Create a Tracer and Reporter that reports all spans in ASCII form
pub fn new_tracer_with_console_reporter() -> (Tracer, ConsoleReporter) {
    let (span_tx, span_rx) = crossbeam_channel::bounded(1000);
    let tracer = Tracer::with_sender(AllSampler, span_tx);
    (tracer, ConsoleReporter::new(span_rx))
}

/// A Reporter that stores all spans it receives in a map,
/// with the intent to display all received spans to the console.
#[derive(Debug)]
pub struct ConsoleReporter {
    span_rx: crossbeam_channel::Receiver<FinishedSpan>,
    span_map: SpanMap,
}

impl ConsoleReporter {
    /// Constructor
    pub fn new(span_rx: crossbeam_channel::Receiver<FinishedSpan>) -> Self {
        ConsoleReporter {
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
            //println!("\t{} = {:?}", span.context().state().span_id(), span);
            self.span_map.insert(span.context().state().span_id(), span);
        }
        count
    }

    /// Print span_map to console
    pub fn print(&self, only_events: bool) {
        print_span_map(&self.span_map, only_events);
    }
}
