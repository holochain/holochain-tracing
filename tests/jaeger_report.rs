use holochain_tracing::*;

extern crate crossbeam_channel;
extern crate rustracing;
extern crate rustracing_jaeger;

use std::{thread, time::Duration};

use holochain_tracing::tracer_jaeger::new_jaeger_tracer;
pub use rustracing::sampler::*;
pub use rustracing_jaeger::{Result, Span as RtSpan, *};

#[test]
fn report_test() {
    let tracer = new_jaeger_tracer("report_test_33");
    let parent_span: HSpan = tracer.span("parent").start().into();
    {
        for i in 0..30 {
            thread::sleep(Duration::from_millis(100));
            // Starts "child" span
            let mut child_span = parent_span.child("child_span");
            child_span.set_tag(|| Tag::new("iteration", format!("{}", i)));
            child_span.event("a log message");
            // Starts "follower" span
            let mut _follower_span = child_span.follower("child_follower_span");
        } // The "child" span dropped and will be sent to `span_rx`
    }
}