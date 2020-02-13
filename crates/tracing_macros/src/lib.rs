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

#[proc_macro_attribute]
pub fn autotrace(_attr: TokenStream, code: TokenStream) -> TokenStream {
    let mut at = Autotrace::default();
    let output = syn::fold::fold_item(&mut at, syn::parse(code).unwrap());
    TokenStream::from(quote! {#output})
}

#[proc_macro_attribute]
pub fn autotrace_deep(_attr: TokenStream, code: TokenStream) -> TokenStream {
    let mut at = Autotrace::deep();
    let output = syn::fold::fold_item(&mut at, syn::parse(code).unwrap());
    TokenStream::from(quote! {#output})
}

#[proc_macro]
pub fn autotrace_deep_block(code: TokenStream) -> TokenStream {
    let output = Autotrace::rewrite_deep(syn::parse(code).unwrap());
    TokenStream::from(quote! {#output})
}

#[proc_macro_attribute]
pub fn newrelic_autotrace(attr: TokenStream, code: TokenStream) -> TokenStream {
    let mut new_relic = NewRelicTrace::new(attr);
    let output = syn::fold::fold_item(&mut new_relic, syn::parse(code).unwrap());
    TokenStream::from(quote! {#output})
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
