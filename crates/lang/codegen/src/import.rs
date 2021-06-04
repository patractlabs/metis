use super::utils::{get_item_attr, gen_cross_calling_conflict_cfg};
use ink_lang_ir::{Contract};
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Result;

pub fn generate_code(contract: &Contract) -> Result<TokenStream2> {
    let storage = contract.module().storage();
    let storage_ident = storage.ident();
    let attrs = storage.attrs();

    let import_mods = get_item_attr(attrs, "import");

    let import_mods_codes = import_mods.iter()
        .map(|ext_mod| generate_import_mod(contract, storage_ident, ext_mod));

    let code = quote! {
        #(#import_mods_codes)*
    };

    Ok(code)
}

fn ext_mod_data_ident(ext_mod: &Ident) -> Ident {
    format_ident!("{}", ext_mod)
}

fn generate_import_mod(contract: &Contract, storage_ident: &Ident, ext_mod: &Ident) -> TokenStream2 {
    let data_ident = ext_mod_data_ident(ext_mod);
    let no_cross_calling_cfg = gen_cross_calling_conflict_cfg(contract);

    quote! {
        #no_cross_calling_cfg
        const _: () = {
            use #ext_mod;

            impl metis_lang::Storage<#storage_ident, #ext_mod::Data<#storage_ident>> for #storage_ident {
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
