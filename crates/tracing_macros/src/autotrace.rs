use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, ImplItemMethod, ItemFn};

const DEBUG_OUTPUT: bool = false;

#[derive(Default)]
pub(crate) struct Autotrace {
    deep: bool,
}

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
    pub fn deep() -> Self {
        Autotrace { deep: true }
    }
}

impl Autotrace {
    fn rewrite_block(name: String, block: syn::Block) -> syn::Block {
        syn::parse(TokenStream::from(quote! {
            {
                let __autotrace_guard = ::holochain_tracing::push_span_with(
                    |span| span.child(#name)
                );
                #block
            }
        }))
        .expect("Couldn't parse statement when rewriting block")
    }

    pub(crate) fn rewrite_deep(mut block: syn::Block) -> syn::Block {
        let mut new_statements = vec![];
        for s in block.stmts {
            let span = proc_macro2::Span::from(s.span()).unwrap();
            let name = format!(
                "Deep file: {:?}:{:?}->{:?}",
                span.source_file().path(),
                span.start(),
                span.end()
            );
            let event = syn::parse(TokenStream::from(quote! {
                ::holochain_tracing::with_top(|top|{
                top.event(#name);
                });
            }))
            .expect("Failed to create event");
            new_statements.push(event);
            new_statements.push(s);
        }
        block.stmts = new_statements;
        block
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
    //     if DE
    // BUG_OUTPUT {
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
        let span = proc_macro2::Span::from(i.span()).unwrap();
        let func_name = format!(
            "{} in {:?}:{:?} (auto:fn)",
            i.sig.ident,
            span.source_file().path(),
            span.start()
        );
        let mut i = i;
        if DEBUG_OUTPUT {
            println!("#autotrace# fold fn: {}", func_name);
        }
        if self.deep {
            i.block = Box::new(Autotrace::rewrite_deep(*i.block));
        }
        i.block = Box::new(Autotrace::rewrite_block(func_name, *i.block));
        i
    }
    
    fn fold_impl_item_method(&mut self, i: ImplItemMethod) -> ImplItemMethod {
        if i.sig.constness.is_some() || self.is_no_autotrace(&i.attrs) {
            return i;
        }
        let span = proc_macro2::Span::from(i.span()).unwrap();
        let method_name = format!(
            "{} in {:?}:{:?} (auto:method)",
            i.sig.ident,
            span.source_file().path(),
            span.start()
        );
        //let method_name = format!("{} (auto:method)", i.sig.ident);
        let mut i = i;
        if DEBUG_OUTPUT {
            println!("#autotrace# fold method: {}", method_name);
        }
        if self.deep {
            i.block = Autotrace::rewrite_deep(i.block);
        }
        i.block = Autotrace::rewrite_block(method_name, i.block);
        i
    }
}
