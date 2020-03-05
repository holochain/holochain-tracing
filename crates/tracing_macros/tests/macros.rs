#![feature(proc_macro_hygiene)]

use crossbeam_channel as cc;
use holochain_tracing as ht;
use holochain_tracing_macros::*;

// mod submod;
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
    #[autotrace_deep]
    pub fn d(mut x: u32) -> u32 {
        let mut p = 9;
        match Some(true) {
            Some(_) => (),
            _ => (),
        }
        {
            x = x + 1;
        }
        if false {
            x = x + 1;
        }
        p = p + 1;
        x + 1 + p
    }

    pub fn e() -> String {
        here!(())
    }
}

#[autotrace]
mod mods {
    use holochain_tracing_macros::*;

    pub fn d(x: u32) -> u32 {
        e(x) * 10
    }
    #[autotrace]
    pub fn e(x: u32) -> u32 {
        f(x) * 2
    }
    pub fn f(x: u32) -> u32 {
        x + 1
    }
}

//new relic trace for optimization wont work until new_relic_license_key is in scope
use lazy_static::lazy_static;
lazy_static! {
    static ref NEW_RELIC_LICENSE_KEY: Option<String> = Some("1234".to_string());
}
#[allow(dead_code)]
#[newrelic_autotrace(TEST)]
mod mods_with_new_relic {
    pub fn d(x: u32) -> u32 {
        e(x) * 10
    }
    pub fn e(x: u32) -> u32 {
        f(x) * 2
    }
    pub fn f(x: u32) -> u32 {
        x + 1
    }
}

#[autotrace]
mod methods {
    use holochain_tracing_macros::*;

    pub struct S {}

    impl S {
        pub fn g(&self, x: u32) -> u32 {
            self.h(x) * 10
        }

        #[autotrace]
        pub fn h(&self, x: u32) -> u32 {
            self.i(x) * 2
        }

        pub fn i(&self, x: u32) -> u32 {
            autotrace_deep_block!({
                if false {}
                x + 1
            })
        }
    }
}

#[test]
fn function_attr() {
    let (tx, rx) = cc::unbounded();
    let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
    let x = {
        let root_span = tracer.span("root").start().into();
        let _guard = ht::push_span(root_span);
        funcs::a(0)
    };
    assert_eq!(x, 20);
    let num = rx.len();
    assert_eq!(num, 4);
    let names: Vec<_> = rx
        .iter()
        .take(num)
        .map(|s| s.operation_name().to_owned())
        .collect();
    assert_eq!(
        names,
        vec![
            "c in crates/tracing_macros/tests/macros.rs:20 (auto:fn)",
            "b in crates/tracing_macros/tests/macros.rs:16 (auto:fn)",
            "a in crates/tracing_macros/tests/macros.rs:12 (auto:fn)",
            "root"
        ]
    );
}

#[test]
fn here_test() {
    assert_eq!(funcs::e(), "crates/tracing_macros/tests/macros.rs:41");
}

#[test]
fn module_attr() {
    let (tx, rx) = cc::unbounded();
    let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
    let x = {
        let root_span = tracer.span("root").start().into();
        let _guard = ht::push_span(root_span);
        mods::d(0)
    };
    assert_eq!(x, 20);
    let num = rx.len();
    assert_eq!(num, 4);
    let names: Vec<_> = rx
        .iter()
        .take(num)
        .map(|s| s.operation_name().to_owned())
        .collect();
    assert_eq!(
        names,
        vec![
            "f in crates/tracing_macros/tests/macros.rs:56 (auto:fn)",
            "e in crates/tracing_macros/tests/macros.rs:53 (auto:fn)",
            "d in crates/tracing_macros/tests/macros.rs:49 (auto:fn)",
            "root"
        ]
    );
}

#[test]
fn method_attr() {
    let (tx, rx) = cc::unbounded();
    let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
    let x = {
        let root_span = tracer.span("root").start().into();
        let _guard = ht::push_span(root_span);
        let s = methods::S {};
        s.g(0)
    };
    assert_eq!(x, 20);
    let num = rx.len();
    assert_eq!(num, 4);
    let names: Vec<_> = rx
        .iter()
        .take(num)
        .map(|s| s.operation_name().to_owned())
        .collect();
    assert_eq!(
        names,
        vec![
            "i in crates/tracing_macros/tests/macros.rs:96 (auto:method)",
            "h in crates/tracing_macros/tests/macros.rs:92 (auto:fn)",
            "g in crates/tracing_macros/tests/macros.rs:87 (auto:method)",
            "root"
        ]
    );
}

// #[test]
// fn submodule_attr() {
//     let (tx, rx) = cc::unbounded();
//     let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
//     let x = {
//         let root_span = tracer.span("root").start().into();
//         ht::start_thread_trace(root_span, || submod::submod::j(0))
//     };
//     assert_eq!(x, 20);
//     let num = rx.len();
//     assert_eq!(num, 4);
//     let names: Vec<_> = rx
//         .iter()
//         .take(num)
//         .map(|s| s.operation_name().to_owned())
//         .collect();
//     assert_eq!(names, vec!["f", "e", "d", "root"]);
// }
