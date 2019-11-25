use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, ImplItemMethod, ItemFn};

const DEBUG_OUTPUT: bool = false;

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
    fn rewrite_block(name: String, block: syn::Block, is_async: bool) -> syn::Block {
        let new_tokens: TokenStream = if is_async {            
            TokenStream::from(quote! {
                {
                    ::holochain_tracing::nested_async(
                        |span| span.child(#name),
                        async move || #block
                    ).await
                }
            })
        } else {
            TokenStream::from(quote! {
                {
                    ::holochain_tracing::nested(
                        |span| span.child(#name),
                        || #block
                    )
                }
            })
        };
        syn::parse(new_tokens).expect("Couldn't parse new tokens")
    }
}


impl syn::fold::Fold for Autotrace {
    fn fold_item_mod(&mut self, i: syn::ItemMod) -> syn::ItemMod {
        if DEBUG_OUTPUT {
            println!("#autotrace# fold module: {}", i.ident.to_string());
        }
        syn::fold::fold_item_mod(self, i)
    }

    // fn fold_item_trait(&mut self, i: syn::ItemTrait) -> syn::ItemTrait {
    //     if DEBUG_OUTPUT {
    //         println!("#autotrace# fold trait: {}", i.ident.to_string());
    //     }
    //     syn::fold::fold_item_trait(self, i)
    // }

    // fn fold_trait_item_method(&mut self, i: syn::TraitItemMethod) -> syn::TraitItemMethod {
    //     if DEBUG_OUTPUT {
    //         println!(
    //             "#autotrace# fold trait item method: {}",
    //             i.sig.ident.to_string()
    //         );
    //     }
    //     syn::fold::fold_trait_item_method(self, i)
    // }

    fn fold_item_impl(&mut self, i: syn::ItemImpl) -> syn::ItemImpl {
        if self.is_no_autotrace(&i.attrs) {
            return i;
        }
        if DEBUG_OUTPUT {
            println!("#autotrace# fold impl: {:?}", i.self_ty);
        }
        syn::fold::fold_item_impl(self, i)
    }

    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        if i.sig.constness.is_some() || self.is_no_autotrace(&i.attrs) {
            return i;
        }
        let func_name = i.sig.ident.to_string();
        let is_async = i.sig.asyncness.is_some();
        let mut i = i;
        if DEBUG_OUTPUT {
            println!("#autotrace# fold fn: {}", i.sig.ident.to_string());
        }
        i.block = Box::new(Autotrace::rewrite_block(func_name, *i.block, is_async));
        i
    }

    fn fold_impl_item_method(&mut self, i: ImplItemMethod) -> ImplItemMethod {
        if i.sig.constness.is_some() || self.is_no_autotrace(&i.attrs) {
            return i;
        }
        let method_name = i.sig.ident.to_string();
        let is_async = i.sig.asyncness.is_some();
        let mut i = i;
        if DEBUG_OUTPUT {
            println!("#autotrace# fold method: {}", i.sig.ident.to_string());
        }
        i.block = Autotrace::rewrite_block(method_name, i.block, is_async);
        i
    }
}
