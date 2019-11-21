
extern crate crossbeam_channel;
extern crate holochain_tracing;
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::mem;

#[proc_macro_attribute]
pub fn traced(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func: syn::ItemFn =
        syn::parse(item.clone().into()).expect("#[traced] can only be used on a function");
    let func_name = func.sig.ident.to_string();
    let block = func.block;
    let new_tokens: TokenStream = TokenStream::from(quote! {
        {
            ::holochain_tracing::stack::nested(
                |span| span.child(#func_name),
                || #block
            )
        }
    });
    func.block = syn::parse(new_tokens).expect("Couldn't parse new tokens");
    func.into_token_stream().into()
}


#[proc_macro]
pub fn start_thread_trace(input: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(input).unwrap();
    TokenStream::from(quote! {
        ::holochain_tracing::stack::start_thread_trace(#expr);
    })
}

// #[proc_macro]
// pub fn trace_with_span(input: TokenStream) -> TokenStream {
    
//     let mut i = input.into_iter();
//     let span_expr: syn::Expr = syn::parse(i.next().unwrap().into()).unwrap();
//     let delim_token = i.next().unwrap().into();
//     let _: syn::Token![,] = syn::parse(delim_token).unwrap();
//     let main_expr: syn::Expr = syn::parse(i.collect::<TokenStream>()).unwrap();
//     println!("{:#?} + {:#?}", span_expr, main_expr);
//     TokenStream::from(quote! {
//         {
//             let span = #span_expr;
//             ::holochain_tracing::stack::start_thread_trace(span);
//             let result = #main_expr;
//             result
//         }
//     })
// }