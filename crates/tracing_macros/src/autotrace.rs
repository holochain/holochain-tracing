extern crate crossbeam_channel;
extern crate holochain_tracing;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, ImplItemMethod, ItemFn};

#[derive(Default)]
pub(crate) struct Autotrace {}

impl Autotrace {
    fn is_no_autotrace(&self, i: &[Attribute]) -> bool {
        i.iter().any(|ref a| {
            if a.path.segments.len() == 1 {
                let ident = &a.path.segments.iter().next().unwrap().ident;
                ident == "autotrace" || ident == "no_autotrace"
            } else {
                false
            }
        })
    }
}

impl Autotrace {
    fn rewrite_block(name: String, block: syn::Block) -> syn::Block {
        let new_tokens: TokenStream = TokenStream::from(quote! {
            {
                ::holochain_tracing::nested(
                    |span| span.child(#name),
                    || #block
                )
            }
        });
        syn::parse(new_tokens).expect("Couldn't parse new tokens")
    }
}

impl syn::fold::Fold for Autotrace {
    // fn fold_item_mod(&mut self, i: syn::ItemMod) -> syn::ItemMod {
    //     i
    // }

    fn fold_item_fn(&mut self, func: ItemFn) -> ItemFn {
        if self.is_no_autotrace(&func.attrs) {
            return func;
        }
        let mut func = func;
        let func_name = func.sig.ident.to_string();
        func.block = Box::new(Autotrace::rewrite_block(func_name, *func.block));
        func
    }

    fn fold_impl_item_method(&mut self, i: ImplItemMethod) -> ImplItemMethod {
        if i.sig.constness.is_some() || self.is_no_autotrace(&i.attrs) {
            return i;
        }
        let mut method = i;
        let method_name = method.sig.ident.to_string();
        method.block = Autotrace::rewrite_block(method_name, method.block);
        method
    }
}
