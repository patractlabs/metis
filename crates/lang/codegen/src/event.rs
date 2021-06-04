use convert_case::{Case, Casing};
use ink_lang_ir::{Contract, Event};
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use std::collections::HashSet as Set;
use syn::spanned::Spanned;
use syn::Result;

use super::utils::{get_metis_item_attr, is_metis_item};

pub fn generate_code(contract: &Contract, storage_ident: &Ident) -> Result<TokenStream2> {
    let mods = contract
        .module()
        .events()
        .filter(|evt| is_metis_item(evt.attrs()))
        .flat_map(|evt| {
            get_metis_item_attr(evt.attrs())
                .iter()
                .map(|mod_ident| mod_ident.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Set<_>>();

    let evt2mods = &contract
        .module()
        .events()
        .filter(|evt| is_metis_item(evt.attrs()))
        .flat_map(|evt| {
            get_metis_item_attr(evt.attrs())
                .iter()
                .map(|mod_ident| (mod_ident.clone(), evt.clone()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(mods
        .iter()
        .flat_map(|m| {
            let mod_evts = evt2mods
                .iter()
                .filter(|evt_pair| evt_pair.0 == *m)
                .map(|evt| evt.1.clone())
                .collect::<Vec<_>>();

            generate_code_for_mod_evts(storage_ident, &m, &mod_evts)
        })
        .collect::<Vec<_>>()
        .iter()
        .fold(quote! {}, |codes, new| quote! {#codes #new}))
}

fn generate_code_for_mod_evts(
    storage_ident: &Ident,
    mod_ident: &Ident,
    evts: &Vec<&Event>,
) -> TokenStream2 {
    let evt_impl_funcs = evts
        .iter()
        .flat_map(|evt| generate_event_emit_impl(evt))
        .collect::<Vec<_>>()
        .iter()
        .fold(quote! {}, |codes, new| quote! {#codes #new});

    quote! {
        #[cfg(not(feature = "ink-as-dependency"))]
        const _: () = {
            impl #mod_ident::EventEmit<#storage_ident> for #storage_ident {
                #evt_impl_funcs
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
