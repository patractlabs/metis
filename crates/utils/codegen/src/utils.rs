use std::collections::HashSet as Set;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Ident, Token};

/// Parses a list of variable names separated by commas.
///
///     a, b, c
///
/// This is how the compiler passes in arguments to our attribute -- it is
/// everything inside the delimiters after the attribute name.
///
///     #[trace_var(a, b, c)]
///                 ^^^^^^^
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

pub fn get_metis_item_attr<'a, I>(attrs: I) -> Set<Ident>
where
    I: IntoIterator<Item = &'a syn::Attribute>,
{
    for attr in attrs.into_iter() {
        if attr.path.is_ident("metis") {
            let vars = syn::parse2::<proc_macro2::Group>(attr.tokens.clone()).unwrap();
            let tags = syn::parse2::<Args>(vars.stream()).unwrap();

            return tags.vars;
        }
    }

    Set::default()
}
