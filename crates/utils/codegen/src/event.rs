use convert_case::{Case, Casing};
use ink_lang_ir::{Contract, Event};
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::Result;

use super::utils::{get_metis_item_attr, is_metis_item};

pub fn generate_code(contract: &Contract, storage_ident: &Ident) -> Result<TokenStream2> {
    Ok(contract
        .module()
        .events()
        .filter(|evt| is_metis_item(evt.attrs()))
        .flat_map(|evt| {
            get_metis_item_attr(evt.attrs())
                .iter()
                .map(|mod_ident| gen_code_for_event(&storage_ident, &mod_ident, evt))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .fold(quote! {}, |codes, new| quote! {#codes #new}))
}

fn gen_code_for_event(storage_ident: &Ident, mod_ident: &Ident, evt: &Event) -> TokenStream2 {
    let evt_impl_func = generate_event_emit_impl(evt);
    quote! {
        #[cfg(not(feature = "ink-as-dependency"))]
        const _: () = {
            impl #mod_ident::EventEmit<#storage_ident> for #storage_ident {
                #evt_impl_func
            }
        };
    }
}

fn generate_event_emit_impl(evt: &Event) -> TokenStream2 {
    let span = evt.span();
    let ident = evt.ident();
    let params_call = evt.fields().map(|evt_field| {
        let span = evt_field.span();
        let ident = evt_field.ident();
        let ty = evt_field.ty();
        quote_spanned!(span=>
            #ident : #ty
        )
    });

    let params_evt = evt.fields().map(|evt_field| {
        let span = evt_field.span();
        let ident = evt_field.ident();
        quote_spanned!(span=>
            #ident
        )
    });

    let evt_name_snake = ident
        .to_string()
        .from_case(Case::Camel)
        .to_case(Case::Snake);
    let impl_func_name = format_ident!("emit_event_{}", evt_name_snake);

    quote_spanned!(span =>
        fn #impl_func_name(
            &mut self,
            #( #params_call ),*
        ) {
            self.env().emit_event(#ident {
                #( #params_evt ),*
            });
        }
    )
}
