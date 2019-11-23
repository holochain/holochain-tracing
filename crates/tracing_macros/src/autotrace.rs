
extern crate crossbeam_channel;
extern crate holochain_tracing;
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::{ItemFn, Attribute};
use quote::quote;

#[derive(Default)]
pub(crate) struct Autotrace {}

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
    // fn fold_item_mod(&mut self, i: syn::ItemMod) -> syn::ItemMod {
    //     i
    // }

    fn fold_item_fn(&mut self, func: ItemFn) -> ItemFn {
        if self.is_no_autotrace(&func.attrs) {
            return func
        }
        let mut func = func;
        let func_name = func.sig.ident.to_string();
        let block = func.block;
        let new_tokens: TokenStream = TokenStream::from(quote! {
            {
                ::holochain_tracing::nested(
                    |span| span.child(#func_name),
                    || #block
                )
            }
        });
        func.block = syn::parse(new_tokens).expect("Couldn't parse new tokens");
        func
    }
}
