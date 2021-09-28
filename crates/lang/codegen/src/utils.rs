use ink_lang_ir::Contract;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashSet as Set;
use syn::{
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    punctuated::Punctuated,
    Ident,
    Token,
};

/// Parses a list of variable names separated by commas like a, b, c
pub struct Args {
    pub vars: Set<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

pub fn is_metis_item<'a, I>(attrs: I) -> bool
where
    I: IntoIterator<Item = &'a syn::Attribute>,
{
    attrs.into_iter().any(|attr| attr.path.is_ident("metis"))
}

#[allow(dead_code)]
pub fn is_metis_item_has_attr<'a, I>(attrs: I, expect_attr: &Ident) -> bool
where
    I: IntoIterator<Item = &'a syn::Attribute>,
{
    get_metis_item_attr(attrs)
        .iter()
        .any(|attr| attr == expect_attr)
}

pub fn get_metis_item_attr<'a, I>(attrs: I) -> Set<Ident>
where
    I: IntoIterator<Item = &'a syn::Attribute>,
{
    for attr in attrs.into_iter() {
        if attr.path.is_ident("metis") {
            let vars = syn::parse2::<proc_macro2::Group>(attr.tokens.clone())
                .expect("metis item attr parse err");
            let tags = syn::parse2::<Args>(vars.stream())
                .expect("metis attr item should be a,b,c");

            return tags.vars
        }
    }

    Set::default()
}

pub fn get_item_attr<'a, I>(attrs: I, name: &str) -> Set<Ident>
where
    I: IntoIterator<Item = &'a syn::Attribute>,
{
    for attr in attrs.into_iter() {
        if attr.path.is_ident(name) {
            let vars = syn::parse2::<proc_macro2::Group>(attr.tokens.clone())
                .expect("get key item attr parse err");
            let tags = syn::parse2::<Args>(vars.stream())
                .expect("get key attr item should be a,b,c");

            return tags.vars
        }
    }

    Set::default()
}

/// Generates `#[cfg(..)]` code to guard against compilation under `ink-as-dependency`.
/// From ink! code
pub fn gen_cross_calling_conflict_cfg(contract: &Contract) -> TokenStream2 {
    if contract.config().is_compile_as_dependency_enabled() {
        return quote! { #[cfg(feature = "__ink_DO_NOT_COMPILE")] }
    }
    quote! { #[cfg(not(feature = "ink-as-dependency"))] }
}
