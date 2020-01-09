extern crate newrelic;
use proc_macro::TokenStream;
use quote::quote;
use std::env;
use syn::{Attribute, ImplItemMethod, ItemFn};

const DEBUG_OUTPUT: bool = false;

#[derive(Default)]
pub(crate) struct Autotrace {
    app_name: String,
    license_key: Option<String>,
}

impl Autotrace {
    pub fn new(attr: TokenStream) -> Self {
        let app_name = attr
            .clone()
            .into_iter()
            .nth(0)
            .map(|token| token.to_string())
            .unwrap_or("UNDEFINED".to_string());
        let license_key = env::var("NEW_RELIC_LICENSE_KEY")
            .map(|license_key| Some(license_key))
            .unwrap_or_else(|_| {
                attr.clone()
                    .into_iter()
                    .nth(1)
                    .map(|token| Some(token.to_string()))
                    .unwrap_or(None)
            });
        Self {
            app_name,
            license_key,
        }
    }
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
    fn rewrite_block(
        app_name: &String,
        license_key: &Option<String>,
        name: String,
        block: syn::Block,
    ) -> syn::Block {
        let new_block = license_key
            .as_ref()
            .map(|license_key| {
                //structure of func created will replce old function but have new relic recording capabilities
                quote! {
                {
                    let __autotrace_guard = ::holochain_tracing::push_span_with(
                        |span| span.child(#name)
                    );

                    //if new relic is somehow down or the daemon is not running, the program should continue normally
                    //stated away from combinators because of closure ownership
                    if let Ok(live_app) = newrelic::App::new(#app_name, #license_key)
                    {
                        if let Ok(transaction) = live_app.non_web_transaction(#name)
                        {
                            #block
                        }
                        else
                        {
                            #block
                        }
                    }
                    else
                    {
                        #block
                    }
                }}
            })
            .unwrap_or_else(|| {
                quote! {
                    {
                        let __autotrace_guard = ::holochain_tracing::push_span_with(
                            |span| span.child(#name)
                        );
                        #block
                    }
                }
            });
        syn::parse(TokenStream::from(new_block))
            .expect("Couldn't parse statement when rewriting block")
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

    // fn fold_item_impl(&mut self, i: syn::ItemImpl) -> syn::ItemImpl {
    //     if self.is_no_autotrace(&i.attrs) {
    //         return i;
    //     }
    //     if DEBUG_OUTPUT {
    //         println!("#autotrace# fold impl: {:?}", i.self_ty);
    //     }
    //     syn::fold::fold_item_impl(self, i)
    // }

    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        if i.sig.constness.is_some() || self.is_no_autotrace(&i.attrs) {
            return i;
        }
        let func_name = i.sig.ident.to_string();
        let mut i = i;
        if DEBUG_OUTPUT {
            println!("#autotrace# fold fn: {}", func_name);
        }
        i.block = Box::new(Autotrace::rewrite_block(
            &self.app_name,
            &self.license_key,
            func_name,
            *i.block,
        ));
        i
    }

    fn fold_impl_item_method(&mut self, i: ImplItemMethod) -> ImplItemMethod {
        if i.sig.constness.is_some() || self.is_no_autotrace(&i.attrs) {
            return i;
        }
        let method_name = i.sig.ident.to_string();
        let mut i = i;
        if DEBUG_OUTPUT {
            println!("#autotrace# fold method: {}", method_name);
        }
        i.block = Autotrace::rewrite_block(&self.app_name, &self.license_key, method_name, i.block);
        i
    }
}
