extern crate crossbeam_channel;
extern crate holochain_tracing;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
// use quote::ToTokens;

mod autotrace;
// mod start_trace;

use autotrace::Autotrace;

#[proc_macro_attribute]
pub fn autotrace(_attr: TokenStream, code: TokenStream) -> TokenStream {
    let mut at = Autotrace::default();
    let output = syn::fold::fold_item(&mut at, syn::parse(code).unwrap());
    TokenStream::from(quote! {#output})
}

#[proc_macro_attribute]
pub fn no_autotrace(_attr: TokenStream, code: TokenStream) -> TokenStream {
    code
}

// #[proc_macro]
// pub fn start_thread_trace(input: TokenStream) -> TokenStream {
//     let expr: syn::Expr = syn::parse(input).unwrap();
//     TokenStream::from(quote! {
//         ::holochain_tracing::stack::start_thread_trace(#expr);
//     })
// }

// #[proc_macro]
// pub fn with_thread_span(input: TokenStream) -> TokenStream {
//     let expr: syn::Expr = syn::parse(input).unwrap();
//     TokenStream::from(quote! {
//         ::holochain_tracing::stack::with_thread_span(|span| {
//             #expr
//         })
//     })
// }
