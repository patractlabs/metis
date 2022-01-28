use proc_macro2::{
    Group,
    Ident,
    Span,
    TokenStream as TokenStream2,
};
use quote::quote;
use syn::{
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    punctuated::Punctuated,
    Token,
};

use super::super::utils::Args;

// Parses a func with param name like Name(a, b, c)
pub struct FuncParam {
    pub name: Ident,
    pub attrs: Vec<Ident>,
}

impl Parse for FuncParam {
    fn parse(input: ParseStream) -> Result<Self> {
        // interface(new, default)
        // println!("FuncParam input {:#?}", &input);

        let name = input.parse::<Ident>()?;
        // println!("FuncParam name {:#?}", &name);

        let attrs_group = input.parse::<Group>()?;
        // println!("FuncParam attrs_group {:#?}", &attrs_group);

        let attrs = syn::parse2::<Args>(attrs_group.stream())?;

        Ok(Self {
            name,
            attrs: attrs.vars.into_iter().collect(),
        })
    }
}

// Parses a func with param name like 'Name1(a, b, c), Name2(a, b, v)'
pub struct FuncParams {
    pub funcs: Vec<FuncParam>,
}

impl Parse for FuncParams {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<FuncParam, Token![,]>::parse_terminated(input)?;
        Ok(FuncParams {
            funcs: vars.into_iter().collect(),
        })
    }
}

fn calculate_interface_id(idents: &Vec<Ident>) -> u32 {
    // return hash(a) ^ hash(b) ^ ...

    let mut interface_id: usize = 0;

    for i in idents {
        let callable_ident = i.to_string().into_bytes();
        let selector = ink_lang_ir::Selector::compute(&callable_ident);
        let selector_id = u32::from_be_bytes(selector.to_bytes().clone()) as usize;

        interface_id = interface_id ^ selector_id;
    }

    // println!("interface_id {:#0x}", &interface_id);

    interface_id as u32
}

fn generate_supports_message(func_params: FuncParams) -> Result<TokenStream2> {
    let mut interface_ids = func_params
        .funcs
        .iter()
        .filter(|f| f.name == "interface")
        .map(|f| calculate_interface_id(&f.attrs))
        .into_iter()
        .collect::<Vec<_>>();

    // for supports_interface
    interface_ids.push(calculate_interface_id(&vec![Ident::new(
        "supports_interface",
        Span::call_site(),
    )]));

    let match_id_trues = interface_ids
        .iter()
        .map(|interface_id| {
            quote! {
                #interface_id => true,
            }
        })
        .collect::<Vec<_>>();

    Ok(quote! {
        impl Flipper {
            fn _supports_interface(&self, interface_id: u32) -> bool {
                match interface_id {
                    #( #match_id_trues )*
                    _ => false,
                }
            }
        }
    })
}

pub fn generate_code(attr: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    // #[metis::supports(interface(new, default), interface(flip, get))]
    // attr : interface(new, default), interface(flip, get)

    let params = syn::parse2::<FuncParams>(attr)?;
    let impl_codes = generate_supports_message(params)?;

    Ok(quote! {
        #input
        #impl_codes
    })
}
