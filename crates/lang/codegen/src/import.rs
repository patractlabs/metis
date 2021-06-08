use super::utils::{
    gen_cross_calling_conflict_cfg,
    get_item_attr,
};
use ink_lang_ir::Contract;
use proc_macro2::{
    Ident,
    TokenStream as TokenStream2,
};
use quote::{
    format_ident,
    quote,
};
use syn::Result;

pub fn generate_code(contract: &Contract) -> Result<TokenStream2> {
    let storage = contract.module().storage();
    let storage_ident = storage.ident();
    let attrs = storage.attrs();

    let import_mods = get_item_attr(attrs, "import");

    let import_mods_codes = import_mods
        .iter()
        .map(|ext_mod| generate_import_mod(contract, storage_ident, ext_mod).unwrap());

    let code = quote! {
        #(#import_mods_codes)*
    };

    Ok(code)
}

fn ext_mod_data_ident(ext_mod: &Ident) -> Ident {
    format_ident!("{}", ext_mod)
}

fn generate_import_mod(
    contract: &Contract,
    storage_ident: &Ident,
    ext_mod: &Ident,
) -> Result<TokenStream2> {
    let data_ident = ext_mod_data_ident(ext_mod);
    let no_cross_calling_cfg = gen_cross_calling_conflict_cfg(contract);
    let ext_mod_data_typ = get_storage_mod_type(contract, ext_mod)?;

    Ok(quote! {
        #no_cross_calling_cfg
        const _: () = {
            use #ext_mod;

            impl metis_lang::Storage<#storage_ident, #ext_mod_data_typ> for #storage_ident {
                fn get(&self) -> &#ext_mod_data_typ {
                    &self.#data_ident
                }
                fn get_mut(&mut self) -> &mut #ext_mod_data_typ {
                    &mut self.#data_ident
                }
            }
        };
    })
}

fn get_storage_mod_type(contract: &Contract, ext_mod: &Ident) -> Result<TokenStream2> {
    let storage = contract.module().storage();
    let mod_to_get = Some(ext_mod.clone());

    for f in storage.fields() {
        if f.ident == mod_to_get {
            if let syn::Type::Path(path_fields) = f.ty.clone() {
                return Ok(quote! {#path_fields})
            }
        }
    }

    Err(syn::Error::new_spanned(
        ext_mod,
        "no found storage mod item which imported",
    ))
}
