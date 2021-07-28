extern crate proc_macro;
use proc_macro2::{
    TokenStream as TokenStream2,
};
use quote::{
    quote,
};
use syn::{
    punctuated::Punctuated,
    Result,
};

pub fn generate_hash_string_or_err(input: TokenStream2) -> Result<TokenStream2> {
    let bytes = blake2b_256_str(input.to_string()); // is \"string\", should delete \"

    println!("{}", input.to_string());

    let mut segments = Punctuated::new();
    bytes
        .iter()
        .for_each(|item| {
             segments.push_value(quote! { #item });
             segments.push_punct(<syn::Token![,]>::default());
        });

    Ok(quote! { [#segments] })
}

fn blake2b_256(input: &[u8], output: &mut [u8]) {
    use ::blake2::digest::{
        Update as _,
        VariableOutput as _,
    };
    let mut blake2 = blake2::VarBlake2b::new_keyed(&[], 32);
    blake2.update(input);
    blake2.finalize_variable(|result| output.copy_from_slice(result));
}

fn blake2b_256_str(input: String) -> [u8; 32] {
    let mut output = [0_u8; 32];

    blake2b_256(&input.into_bytes(), &mut output);

    output
}
