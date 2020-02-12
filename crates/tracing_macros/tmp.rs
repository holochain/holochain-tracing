#newrelic_trace# rewriting for module: mods_with_new_relic
#newrelic_trace# rewriting for function: d
#newrelic_trace# rewriting for function: e
#newrelic_trace# rewriting for function: f
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use crossbeam_channel as cc;
use holochain_tracing as ht;
use holochain_tracing_macros::*;
mod funcs {
    use holochain_tracing_macros::*;
    pub fn a(x: u32) -> u32 {
        let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
            span.child("a in crates/tracing_macros/tests/macros.rs:12 (auto:fn)")
        });
        {
            b(x) * 10
        }
    }
    pub fn b(x: u32) -> u32 {
        let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
            span.child("b in crates/tracing_macros/tests/macros.rs:16 (auto:fn)")
        });
        {
            c(x) * 2
        }
    }
    pub fn c(x: u32) -> u32 {
        let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
            span.child("c in crates/tracing_macros/tests/macros.rs:20 (auto:fn)")
        });
        {
            x + 1
        }
    }
    pub fn d(mut x: u32) -> u32 {
        let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
            span.child("d in crates/tracing_macros/tests/macros.rs:24 (auto:fn)")
        });
        {
            ::holochain_tracing::with_top(|top| {
                top.event("Deep file: crates/tracing_macros/tests/macros.rs:25->25");
            });
            let mut p = 9;
            ::holochain_tracing::with_top(|top| {
                top.event("Deep file: crates/tracing_macros/tests/macros.rs:26->29");
            });
            match Some(true) {
                Some(_) => (),
                _ => (),
            }
            ::holochain_tracing::with_top(|top| {
                top.event("Deep file: crates/tracing_macros/tests/macros.rs:30->32");
            });
            {
                x = x + 1;
            }
            ::holochain_tracing::with_top(|top| {
                top.event("Deep file: crates/tracing_macros/tests/macros.rs:33->35");
            });
            if false {
                x = x + 1;
            }
            ::holochain_tracing::with_top(|top| {
                top.event("Deep file: crates/tracing_macros/tests/macros.rs:36->36");
            });
            p = p + 1;
            ::holochain_tracing::with_top(|top| {
                top.event("Deep file: crates/tracing_macros/tests/macros.rs:37->37");
            });
            x + 1 + p
        }
    }
    pub fn e() -> String {
        String::from("crates/tracing_macros/tests/macros.rs:41")
    }
}
mod mods {
    use holochain_tracing_macros::*;
    pub fn d(x: u32) -> u32 {
        let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
            span.child("d in crates/tracing_macros/tests/macros.rs:49 (auto:fn)")
        });
        {
            e(x) * 10
        }
    }
    pub fn e(x: u32) -> u32 {
        let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
            span.child("e in crates/tracing_macros/tests/macros.rs:53 (auto:fn)")
        });
        {
            f(x) * 2
        }
    }
    pub fn f(x: u32) -> u32 {
        let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
            span.child("f in crates/tracing_macros/tests/macros.rs:56 (auto:fn)")
        });
        {
            x + 1
        }
    }
}
mod mods_with_new_relic {
    use lazy_static::lazy_static;
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct NEW_RELIC_LICENSE_KEY {
        __private_field: (),
    }
    #[doc(hidden)]
    static NEW_RELIC_LICENSE_KEY: NEW_RELIC_LICENSE_KEY = NEW_RELIC_LICENSE_KEY {
        __private_field: (),
    };
    impl ::lazy_static::__Deref for NEW_RELIC_LICENSE_KEY {
        type Target = Option<String>;
        fn deref(&self) -> &Option<String> {
            #[inline(always)]
            fn __static_ref_initialize() -> Option<String> {
                Some("1234".to_string())
            }
            #[inline(always)]
            fn __stability() -> &'static Option<String> {
                static LAZY: ::lazy_static::lazy::Lazy<Option<String>> =
                    ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for NEW_RELIC_LICENSE_KEY {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    pub fn d(x: u32) -> u32 {
        let _transaction = if let Some(license_key) = &*NEW_RELIC_LICENSE_KEY {
            if let Ok(live_app) = newrelic::App::new("TEST", &license_key) {
                if let Ok(_transaction) = live_app.non_web_transaction("d") {
                    Some(_transaction)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        {
            e(x) * 10
        }
    }
    pub fn e(x: u32) -> u32 {
        let _transaction = if let Some(license_key) = &*NEW_RELIC_LICENSE_KEY {
            if let Ok(live_app) = newrelic::App::new("TEST", &license_key) {
                if let Ok(_transaction) = live_app.non_web_transaction("e") {
                    Some(_transaction)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        {
            f(x) * 2
        }
    }
    pub fn f(x: u32) -> u32 {
        let _transaction = if let Some(license_key) = &*NEW_RELIC_LICENSE_KEY {
            if let Ok(live_app) = newrelic::App::new("TEST", &license_key) {
                if let Ok(_transaction) = live_app.non_web_transaction("f") {
                    Some(_transaction)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        {
            x + 1
        }
    }
}
mod methods {
    use holochain_tracing_macros::*;
    pub struct S {}
    impl S {
        pub fn g(&self, x: u32) -> u32 {
            let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
                span.child("g in crates/tracing_macros/tests/macros.rs:86 (auto:method)")
            });
            {
                self.h(x) * 10
            }
        }
        pub fn h(&self, x: u32) -> u32 {
            let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
                span.child("h in crates/tracing_macros/tests/macros.rs:91 (auto:fn)")
            });
            {
                self.i(x) * 2
            }
        }
        pub fn i(&self, x: u32) -> u32 {
            let __autotrace_guard = ::holochain_tracing::push_span_with(|span| {
                span.child("i in crates/tracing_macros/tests/macros.rs:95 (auto:method)")
            });
            {
                {
                    ::holochain_tracing::with_top(|top| {
                        top.event("Deep file: crates/tracing_macros/tests/macros.rs:97->97");
                    });
                    if false {}
                    ::holochain_tracing::with_top(|top| {
                        top.event("Deep file: crates/tracing_macros/tests/macros.rs:98->98");
                    });
                    x + 1
                }
            }
        }
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const function_attr: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("function_attr"),
        ignore: false,
        allow_fail: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(function_attr())),
};
fn function_attr() {
    let (tx, rx) = cc::unbounded();
    let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
    let x = {
        let root_span = tracer.span("root").start().into();
        let _guard = ht::push_span(root_span);
        funcs::a(0)
    };
    {
        match (&x, &20) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        }
    };
    let num = rx.len();
    {
        match (&num, &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        }
    };
    let names: Vec<_> = rx
        .iter()
        .take(num)
        .map(|s| s.operation_name().to_owned())
        .collect();
    {
        match ( & names , & < [ _ ] > :: into_vec ( box [ "c in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 20, column: 4 } (auto:fn)" , "b in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 16, column: 4 } (auto:fn)" , "a in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 12, column: 4 } (auto:fn)" , "root" ] ) ) { ( left_val , right_val ) => { if ! ( * left_val == * right_val ) { { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "assertion failed: `(left == right)`\n  left: `" , "`,\n right: `" , "`" ] , & match ( & & * left_val , & & * right_val ) { ( arg0 , arg1 ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) , :: core :: fmt :: ArgumentV1 :: new ( arg1 , :: core :: fmt :: Debug :: fmt ) ] , } ) ) } } } }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const here_test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("here_test"),
        ignore: false,
        allow_fail: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(here_test())),
};
fn here_test() {
    {
        match (&funcs::e(), &"") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const module_attr: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("module_attr"),
        ignore: false,
        allow_fail: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(module_attr())),
};
fn module_attr() {
    let (tx, rx) = cc::unbounded();
    let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
    let x = {
        let root_span = tracer.span("root").start().into();
        let _guard = ht::push_span(root_span);
        mods::d(0)
    };
    {
        match (&x, &20) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        }
    };
    let num = rx.len();
    {
        match (&num, &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        }
    };
    let names: Vec<_> = rx
        .iter()
        .take(num)
        .map(|s| s.operation_name().to_owned())
        .collect();
    {
        match ( & names , & < [ _ ] > :: into_vec ( box [ "f in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 52, column: 4 } (auto:fn)" , "e in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 49, column: 4 } (auto:fn)" , "d in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 45, column: 4 } (auto:fn)" , "root" ] ) ) { ( left_val , right_val ) => { if ! ( * left_val == * right_val ) { { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "assertion failed: `(left == right)`\n  left: `" , "`,\n right: `" , "`" ] , & match ( & & * left_val , & & * right_val ) { ( arg0 , arg1 ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) , :: core :: fmt :: ArgumentV1 :: new ( arg1 , :: core :: fmt :: Debug :: fmt ) ] , } ) ) } } } }
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const method_attr: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("method_attr"),
        ignore: false,
        allow_fail: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(method_attr())),
};
fn method_attr() {
    let (tx, rx) = cc::unbounded();
    let tracer = ht::Tracer::with_sender(ht::AllSampler, tx);
    let x = {
        let root_span = tracer.span("root").start().into();
        let _guard = ht::push_span(root_span);
        let s = methods::S {};
        s.g(0)
    };
    {
        match (&x, &20) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        }
    };
    let num = rx.len();
    {
        match (&num, &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    {
                        ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                            &[
                                "assertion failed: `(left == right)`\n  left: `",
                                "`,\n right: `",
                                "`",
                            ],
                            &match (&&*left_val, &&*right_val) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ))
                    }
                }
            }
        }
    };
    let names: Vec<_> = rx
        .iter()
        .take(num)
        .map(|s| s.operation_name().to_owned())
        .collect();
    {
        match ( & names , & < [ _ ] > :: into_vec ( box [ "i in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 91, column: 8 } (auto:method)" , "h in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 87, column: 8 } (auto:fn)" , "g in \"crates/tracing_macros/tests/macros.rs\":LineColumn { line: 82, column: 8 } (auto:method)" , "root" ] ) ) { ( left_val , right_val ) => { if ! ( * left_val == * right_val ) { { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "assertion failed: `(left == right)`\n  left: `" , "`,\n right: `" , "`" ] , & match ( & & * left_val , & & * right_val ) { ( arg0 , arg1 ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Debug :: fmt ) , :: core :: fmt :: ArgumentV1 :: new ( arg1 , :: core :: fmt :: Debug :: fmt ) ] , } ) ) } } } }
    };
}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&function_attr, &here_test, &module_attr, &method_attr])
}
