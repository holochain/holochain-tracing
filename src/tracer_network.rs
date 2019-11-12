use crate::{Reporter, Tracer};
use std::thread;

pub use rustracing::{sampler::*, span::SpanReceiver};
pub use rustracing_jaeger::{span::SpanContextState as RjSpanContextState, Span as RjSpan};

fn run_reporter_thread(service_name: &'static str, span_rx: SpanReceiver<RjSpanContextState>) {
    thread::spawn(move || {
        let reporter = Reporter::new(service_name).unwrap();
        for span in span_rx {
            reporter.report(&[span]).expect("could not report");
        }
    });
}

/// Create a Tracer that sends all spans automatically to the default jaeger reporter
pub fn new_tracer_with_network_reporter(service_name: &'static str) -> Tracer {
    let (span_tx, span_rx) = crossbeam_channel::bounded(50);
    run_reporter_thread(service_name, span_rx);
    let tracer = Tracer::with_sender(AllSampler, span_tx);
    tracer
}
