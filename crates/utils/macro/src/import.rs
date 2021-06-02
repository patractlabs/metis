use proc_macro2::TokenStream as TokenStream2;
use syn::Result;

pub fn generate(attr: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match generate_or_err(attr, input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

pub fn generate_or_err(attr: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    metis_codegen::generate_code_for_extend(attr, input)
}
