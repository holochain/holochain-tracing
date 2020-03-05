use proc_macro::TokenStream;
use quote::quote;
use syn::{ImplItemMethod, ItemFn};

pub(crate) struct NewRelicTrace {
    app_name: String,
}

impl NewRelicTrace {
    pub fn new(attr: TokenStream) -> Self {
        let app_name = attr
            .into_iter()
            .next()
            .map(|token| token.to_string())
            .unwrap_or_else(|| "UNDEFINED".to_string());

        Self { app_name }
    }

    fn rewrite_block(app_name: &String, name: String, block: syn::Block) -> syn::Block {
        let new_block = quote! {
        {
            use crate::NEW_RELIC_LICENSE_KEY;
            //if new relic is somehow down or the daemon is not running, the program should continue normally
            //stated away from combinators because of closure ownership
            let _transaction = if let Some(license_key) = &*NEW_RELIC_LICENSE_KEY
            {
                if let Ok(live_app) = newrelic::App::new(#app_name, &license_key)
                {
                    if let Ok(_transaction) = live_app.non_web_transaction(#name)
                    {
                        Some(_transaction)
                    }
                    else
                    {
                        None
                    }
                }
                else
                {
                    None
                }
            }
            else
            {
                None
            };
            #block

        }};
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
        i.block = NewRelicTrace::rewrite_block(&self.app_name, method_name, i.block);
        i
    }
}
