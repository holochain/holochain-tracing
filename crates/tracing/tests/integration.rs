use std::env;
use std::sync::Once;
use holochain_tracing as ht;
use ht::structured::Output;
use std::io::Read;
use gag::BufferRedirect;
use tracing::*;

static INIT: Once = Once::new();

fn setup() {
    env::set_var("RUST_LOG", "debug");
    INIT.call_once(|| assert!(ht::structured::init_fmt(Output::Log, None).is_ok()));
}

fn some_work() -> i32 {
    let a = 1;
    info!(a);
    let b = 2;
    trace!(a);
    a + b + some_more_work()
}

fn some_more_work() -> i32 {
    let c = 3;
    debug!(c);
    c
}

#[test]
fn test_filter() {
    setup();
    let mut buf = BufferRedirect::stdout().expect("Failed to start redirect");
    some_work();
    let mut output = String::new();
    buf.read_to_string(&mut output).expect("Failed to read redirect");
    assert!(!output.contains("TRACE"));
    assert!(output.contains("DEBUG"));
}