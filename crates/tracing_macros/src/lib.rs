
extern crate crossbeam_channel;
extern crate holochain_tracing;
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::{ItemFn, Attribute};
use quote::quote;
// use quote::ToTokens;

#[proc_macro_attribute]
pub fn autotrace(_attr: TokenStream, code: TokenStream) -> TokenStream {
    let mut at = Autotrace::default();
    let output = syn::fold::fold_item(&mut at, syn::parse(code).unwrap());
    TokenStream::from(quote!{#output})
}

#[proc_macro_attribute]
pub fn no_autotrace(_attr: TokenStream, code: TokenStream) -> TokenStream {
    code
}

#[derive(Default)]
struct Autotrace {}

impl Autotrace {

    fn is_no_autotrace(&self, i: &[Attribute]) -> bool {
        i.iter().any(|ref a| if a.path.segments.len() == 1 {
            let ident = &a.path.segments.iter().next().unwrap().ident;
            ident == "autotrace" || ident == "no_autotrace"
        } else {
            false
        })
    }

}

impl syn::fold::Fold for Autotrace {
    fn fold_item_mod(&mut self, i: syn::ItemMod) -> syn::ItemMod {
        i
    }

    fn fold_item_fn(&mut self, func: ItemFn) -> ItemFn {
        if self.is_no_autotrace(&func.attrs) {
            return func
        }
        let mut func = func;
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
        func
    }
}


#[proc_macro]
pub fn start_thread_trace(input: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(input).unwrap();
    TokenStream::from(quote! {
        ::holochain_tracing::stack::start_thread_trace(#expr);
    })
}

#[proc_macro]
pub fn with_thread_span(input: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(input).unwrap();
    TokenStream::from(quote! {
        ::holochain_tracing::stack::with_thread_span(|span| {
            #expr
        })
    })
}

#[proc_macro]
pub fn trace_with_span(input: TokenStream) -> TokenStream {
    
    let mut i = input.into_iter();
    let span_expr: syn::Expr = syn::parse(i.next().unwrap().into()).unwrap();
    let delim_token = i.next().unwrap().into();
    let _: syn::Token![,] = syn::parse(delim_token).unwrap();
    let main_expr: syn::Expr = syn::parse(i.collect::<TokenStream>()).unwrap();
    // println!("{:#?} + {:#?}", span_expr, main_expr);
    TokenStream::from(quote! {
        {
            let span = #span_expr;
            ::holochain_tracing::stack::nested_root(span, || #main_expr);
        }
    })
}