use crate::{span_map::*, Reporter, Tracer};
use rustracing::span::SpanReceiver;
use rustracing_jaeger::span::SpanContextState;
use std::{borrow::Cow, io::Cursor};
use std::{thread, time::Duration};

pub use rustracing::sampler::*;
pub use rustracing_jaeger::{Result, Span as RtSpan, *};
use std::collections::HashMap;

fn run_reporter_thread(service_name: &'static str, span_rx: SpanReceiver<SpanContextState>) {
    thread::spawn(move || {
        let reporter = Reporter::new(service_name).unwrap();
        for span in span_rx {
            reporter.report(&[span]).expect("could not report");
        }
    });
}

/// Create a Tracer that sends all spans to a jaeger reporter thread
pub fn new_jaeger_tracer(service_name: &'static str) -> Tracer {
    let (span_tx, span_rx) = crossbeam_channel::bounded(50);
    run_reporter_thread(service_name, span_rx);
    let tracer = Tracer::with_sender(AllSampler, span_tx);
    tracer
}
