extern crate proc_macro;

mod contract;
mod erc165;
mod reentrancy_guard;
mod stub;
mod utils;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::generate(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn import(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn metis(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// The macro to generate _supports_interface for impl erc165
/// Use like
///    #[metis::supports(interface(new, default), interface(flip, get))]
///    impl Flipper {}
/// This will generate this two interface:
///   - Selector(new) ^ Selector(default)
///   - Selector(flip) ^ Selector(get)
#[proc_macro_attribute]
pub fn supports(attr: TokenStream, item: TokenStream) -> TokenStream {
    erc165::generate(attr.into(), item.into()).into()
}

/// The macro to generate reentrancy_guard check for message which need nonreentrancy
#[proc_macro_attribute]
pub fn reentrancy_guard(attr: TokenStream, item: TokenStream) -> TokenStream {
    reentrancy_guard::generate(attr.into(), item.into()).into()
}

/// The macro to generate stub code in metis
#[proc_macro_attribute]
pub fn stub(attr: TokenStream, item: TokenStream) -> TokenStream {
    stub::generate(attr.into(), item.into()).into()
}

/// The macro to generate hash by input.
#[proc_macro]
pub fn hash(input: TokenStream) -> TokenStream {
    match utils::generate_hash_string_or_err(input.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

/// Gen selector id form input message name.
#[proc_macro]
pub fn selector_id(input: TokenStream) -> TokenStream {
    match utils::generate_msg_selector_id_or_err(input.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
    .into()
}
