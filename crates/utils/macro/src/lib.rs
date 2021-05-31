extern crate proc_macro;

mod contract;
mod extend;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::generate(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn extend(attr: TokenStream, item: TokenStream) -> TokenStream {
    extend::generate(attr.into(), item.into()).into()
}