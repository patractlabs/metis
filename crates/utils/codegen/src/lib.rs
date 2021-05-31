use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use ink_lang_ir::Contract;
use syn::{Result};

pub fn generate_code(attr: TokenStream2, input: TokenStream2) -> Result<TokenStream2>{
    let item_mod = syn::parse2::<syn::ItemMod>(input.clone()).unwrap();

    let contract_ink = Contract::new(attr.clone(), input).unwrap();
    let module = contract_ink.module();
    let ident = module.ident();
    let attrs = module.attrs();
    let vis = module.vis();
    let storage_ident = module.storage().ident();

    println!("{:#?}", attrs);
    println!("{:#?}", vis);
    println!("{:#?}", ident);
    println!("{:#?}", storage_ident);

    let items = match item_mod.content {
        Some((_brace, items)) => items,
        None => {
            return Err(ink_lang_ir::format_err_spanned!(
                item_mod,
                "out-of-line ink! modules are not supported, use `#[ink::contract] mod name {{ ... }}`",
            ))
        }
    };

    // ext the env
    let env = quote!{
        #[cfg(not(feature = "ink-as-dependency"))]
        use ::ink_lang::{EmitEvent, Env, StaticEnv};

        #[cfg(not(feature = "ink-as-dependency"))]
        impl metis_contract::Env for #storage_ident {
            type BaseEvent = <#storage_ident as ::ink_lang::BaseEvent>::Type;
            type AccountId = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::AccountId;
            type Balance = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Balance;
            type Hash = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Hash;
            type Timestamp = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Timestamp;
            type BlockNumber = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::BlockNumber;
        }

        #[cfg(not(feature = "ink-as-dependency"))]
        impl metis_contract::EnvAccess<#storage_ident > for #storage_ident  {
            fn caller() -> <#storage_ident  as metis_contract::Env>::AccountId {
                Self::env().caller()
            }
    
            fn transferred_balance() -> <#storage_ident  as metis_contract::Env>::Balance {
                Self::env().transferred_balance()
            }
        }
    };

    let module_extend = quote! {
        #( #attrs )*
        #vis mod #ident {
            #( #items )*

            #env
        }
    };

    // For codegen in ink
    let ink_contract = ink_lang_codegen::generate_code(
        &Contract::new(attr, module_extend).unwrap());
   
    Ok(ink_contract)
}