use std::{thread, time::Duration};

use holochain_tracing::{Span, tracer_network::new_tracer_with_network_reporter, *};

/// For manually testing if we can see reports on a jaeger client
fn main() {
    let tracer = new_tracer_with_network_reporter("report_test");
    let parent_span: Span = tracer.span("parent").start().into();
    {
        for i in 0..10 {
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
