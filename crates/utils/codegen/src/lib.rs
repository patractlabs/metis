use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use ink_lang_ir::Contract;

pub fn generate_code(contract: &Contract) -> TokenStream2{
    let ink_contract = ink_lang_codegen::generate_code(contract);

    quote! {
        #ink_contract
    }
}