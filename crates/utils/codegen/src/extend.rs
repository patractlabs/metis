use proc_macro2::TokenStream as TokenStream2;
use quote::{
    quote,format_ident};
use syn::{Result};
use proc_macro2::Ident;

use super::utils::Args;

pub fn generate_code_for_extend(attr: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    let mods = syn::parse2::<Args>(attr)?;
    let storage_struct = syn::parse2::<syn::ItemStruct>(input.clone())?;

    let code_for_mods = mods.vars.iter().map(|ext_mod| {
        generate_extand_mod(&storage_struct.ident, ext_mod)
    });

    let gen = quote!{
        #input

        #(#code_for_mods)*
    };

    Ok(gen)
}

fn ext_mod_data_ident(ext_mod: &Ident) -> Ident {
    format_ident!("data_{}", ext_mod)
}

fn generate_extand_mod(storage_ident: &Ident, ext_mod: &Ident) -> TokenStream2{
    let data_ident = ext_mod_data_ident(ext_mod);

    quote!{
        #[cfg(not(feature = "ink-as-dependency"))]
        const _: () = {
            use #ext_mod;

            impl #ext_mod::Storage<#storage_ident> for #storage_ident {
                fn get(&self) -> &#ext_mod::Data<#storage_ident> {
                    &self.#data_ident
                }
        
                fn get_mut(&mut self) -> &mut #ext_mod::Data<#storage_ident> {
                    &mut self.#data_ident
                }
            }
        };
    }
}