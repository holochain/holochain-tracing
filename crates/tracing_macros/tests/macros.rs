#![feature(proc_macro_hygiene)]

use holochain_tracing as ht;
use holochain_tracing_macros::*;
use crossbeam_channel as cc;

mod funcs {
    use holochain_tracing_macros::*;

    #[autotrace]
    pub fn a(x: u32) -> u32 {
        b(x) * 10
    }
    
    #[autotrace]
    pub fn b(x: u32) -> u32 {
        c(x) * 2
    }
    
    #[autotrace]
    pub fn c(x: u32) -> u32 {
        x + 1
    }
}

#[autotrace]
mod mods {
    use holochain_tracing_macros::*;

    pub fn a(x: u32) -> u32 {
        b(x) * 10
    }
    
    #[autotrace]
    pub fn b(x: u32) -> u32 {
        c(x) * 2
    }
    
    pub fn c(x: u32) -> u32 {
        x + 1
    }
}

// fn oh_hi() -> u32 {
//     println!("oh, hello there!");
//     2
// }

#[test]
fn function_decoration() {
    let (tx, rx) = cc::unbounded();
    let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
    start_thread_trace!(tracer.span("root").start().into());
    let x = funcs::a(0);
    assert_eq!(x, 20);
    let spans: Vec<_> = rx.iter().take(3).map(|s| s.operation_name().to_owned()).collect();
    assert_eq!(spans, vec!["c", "b", "a"]);
}

#[test]
fn module_decoration() {
    let (tx, rx) = cc::unbounded();
    let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
    start_thread_trace!(tracer.span("root").start().into());
    let x = mods::a(0);
    assert_eq!(x, 20);
    let spans: Vec<_> = rx.iter().take(3).map(|s| s.operation_name().to_owned()).collect();
    assert_eq!(spans, vec!["c", "b", "a"]);
}

// #[test]
// fn function_style() {
//     let x = trace_with_span!(span, oh_hi());
//     assert_eq!(x, 2);
// }
