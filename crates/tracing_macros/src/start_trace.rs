
extern crate crossbeam_channel;
extern crate holochain_tracing;
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::{ItemFn, Attribute};
use quote::quote;
// use quote::ToTokens;


// #[proc_macro]
// pub fn trace_with_span(input: TokenStream) -> TokenStream {
    
//     let mut i = input.into_iter();
//     let span_expr: syn::Expr = syn::parse(i.next().unwrap().into()).unwrap();
//     let delim_token = i.next().unwrap().into();
//     let _: syn::Token![,] = syn::parse(delim_token).unwrap();
//     let main_expr: syn::Expr = syn::parse(i.collect::<TokenStream>()).unwrap();
//     // println!("{:#?} + {:#?}", span_expr, main_expr);
//     TokenStream::from(quote! {
//         {
//             let span = #span_expr;
//             ::holochain_tracing::stack::nested_root(span, || #main_expr);
//         }
//     })
// }

#[derive(Default)]
struct StartTrace {}

impl StartTrace {

}

impl syn::fold::Fold for Autotrace {
    fn fold_expr(&mut self, i: syn::Expr) -> syn::Expr {
        i
    }
}



// #[proc_macro]
// pub fn trace_with_span(input: TokenStream) -> TokenStream {
    
//     let mut i = input.into_iter();
//     let span_expr: syn::Expr = syn::parse(i.next().unwrap().into()).unwrap();
//     let delim_token = i.next().unwrap().into();
//     let _: syn::Token![,] = syn::parse(delim_token).unwrap();
//     let main_expr: syn::Expr = syn::parse(i.collect::<TokenStream>()).unwrap();
//     // println!("{:#?} + {:#?}", span_expr, main_expr);
//     TokenStream::from(quote! {
//         {
//             let span = #span_expr;
//             ::holochain_tracing::stack::nested_root(span, || #main_expr);
//         }
//     })
// }