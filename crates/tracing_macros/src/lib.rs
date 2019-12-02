extern crate crossbeam_channel;
extern crate holochain_tracing;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use autotrace::Autotrace;
use proc_macro::TokenStream;
use quote::quote;

mod autotrace;

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
