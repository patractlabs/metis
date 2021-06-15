use ink_lang_ir::Contract;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Result;

pub fn generate_code(attr: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    println!("attr {:#?}", &attr);

    // #[metis::supports(interface(new, default), interface(flip, get))]

    Ok(input)
}