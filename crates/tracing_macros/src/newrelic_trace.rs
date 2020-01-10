use proc_macro::TokenStream;
use quote::quote;
use std::env;
use syn::{ImplItemMethod, ItemFn};

#[derive(Default)]
pub(crate) struct NewRelicTrace {
    app_name: String,
    license_key: Option<String>,
}

impl NewRelicTrace {
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
                        #block
                    }
                }
            });
        syn::parse(TokenStream::from(new_block))
            .expect("Couldn't parse statement when rewriting block")
    }
}

impl syn::fold::Fold for NewRelicTrace {
    fn fold_item_mod(&mut self, i: syn::ItemMod) -> syn::ItemMod {
        #[cfg(debug_assertions)]
        println!(
            "#newrelic_trace# rewriting for module: {}",
            i.ident.to_string()
        );
        syn::fold::fold_item_mod(self, i)
    }

    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        if i.sig.constness.is_some() {
            return i;
        }
        let func_name = i.sig.ident.to_string();
        #[cfg(debug_assertions)]
        println!("#newrelic_trace# rewriting for function: {}", func_name);
        let mut i = i;
        i.block = Box::new(NewRelicTrace::rewrite_block(
            &self.app_name,
            &self.license_key,
            func_name,
            *i.block,
        ));
        i
    }

    fn fold_impl_item_method(&mut self, i: ImplItemMethod) -> ImplItemMethod {
        if i.sig.constness.is_some() {
            return i;
        }
        let method_name = i.sig.ident.to_string();
        #[cfg(debug_assertions)]
        println!(
            "#newrelic_trace# rewriting for method name: {}",
            method_name
        );
        let mut i = i;
        i.block =
            NewRelicTrace::rewrite_block(&self.app_name, &self.license_key, method_name, i.block);
        i
    }
}
