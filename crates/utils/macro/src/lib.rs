extern crate proc_macro;

mod contract;
mod import;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::generate(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn import(attr: TokenStream, item: TokenStream) -> TokenStream {
    import::generate(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn metis(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}
