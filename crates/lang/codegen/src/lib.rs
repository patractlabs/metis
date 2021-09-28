mod components;
mod env;
mod event;
mod import;
mod utils;

use ink_lang_ir::Contract;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Result;

pub mod stub;

pub mod component {
    pub use super::components::{
        erc165,
        reentrancy_guard,
    };
}

pub fn generate_code(attr: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    let item_mod = syn::parse2::<syn::ItemMod>(input.clone())
        .expect("`#[contract]` marco should use for mod");

    let contract_ink = Contract::new(attr.clone(), input)?;
    let module = contract_ink.module();
    let ident = module.ident();
    let attrs = module.attrs();
    let vis = module.vis();
    let storage_ident = module.storage().ident();

    let items = match item_mod.content {
        Some((_brace, items)) => items,
        None => {
            return Err(ink_lang_ir::format_err_spanned!(
            item_mod,
            "out-of-line ink! modules are not supported, use `#[ink::contract] mod name {{ ... }}`",
        ))
        }
    };

    let envs = env::generate_code(&contract_ink, &storage_ident)?;
    let imports = import::generate_code(&contract_ink)?;
    let events = event::generate_code(&contract_ink, &storage_ident)?;

    let module_extend = quote! {
        #( #attrs )*
        #vis mod #ident {
            #( #items )*

            #envs
            #imports
            #events
        }
    };

    // For codegen in ink
    let contract = ink_lang_codegen::generate_code(&Contract::new(attr, module_extend)?);
    Ok(contract)
}
