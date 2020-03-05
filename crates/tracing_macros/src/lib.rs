#![feature(proc_macro_span)]
extern crate crossbeam_channel;
extern crate holochain_tracing;
extern crate proc_macro;
extern crate quote;
extern crate syn;

mod autotrace;
mod newrelic_trace;

use autotrace::Autotrace;
use newrelic_trace::NewRelicTrace;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn autotrace(_attr: TokenStream, code: TokenStream) -> TokenStream {
    if cfg!(feature = "tracing-on") {
        let mut at = Autotrace::default();
        let output = syn::fold::fold_item(&mut at, syn::parse(code).unwrap());
        let span = output.span();
        TokenStream::from(quote::quote_spanned! {span=>
            #output
        })
    } else {
        code
    }
}

#[proc_macro_attribute]
pub fn autotrace_deep(_attr: TokenStream, code: TokenStream) -> TokenStream {
    if cfg!(feature = "tracing-on") {
        let mut at = Autotrace::deep();
        let output = syn::fold::fold_item(&mut at, syn::parse(code).unwrap());
        TokenStream::from(quote! {#output})
    } else {
        code
    }
}

#[proc_macro]
pub fn autotrace_deep_block(code: TokenStream) -> TokenStream {
    if cfg!(feature = "tracing-on") {
        let output = Autotrace::rewrite_deep(syn::parse(code).unwrap());
        TokenStream::from(quote! {#output})
    } else {
        code
    }
}

#[proc_macro_attribute]
pub fn newrelic_autotrace(attr: TokenStream, code: TokenStream) -> TokenStream {
    if cfg!(feature = "newrelic-on") {
        let mut new_relic = NewRelicTrace::new(attr);
        let output = syn::fold::fold_item(&mut new_relic, syn::parse(code).unwrap());
        let span = output.span();
        TokenStream::from(quote::quote_spanned! {span=>
            #output
        })
    } else {
        code
    }
}

#[proc_macro_attribute]
pub fn no_autotrace(_attr: TokenStream, code: TokenStream) -> TokenStream {
    code
}

#[proc_macro]
pub fn here(code: TokenStream) -> TokenStream {
    let span = code
        .into_iter()
        .next()
        .expect("Failed to unwrap code, pass in a ()")
        .span();
    let position = format!(
        "{}:{}",
        span.source_file().path().display(),
        span.start().line
    );
    TokenStream::from(quote! {String::from(#position)})
}
