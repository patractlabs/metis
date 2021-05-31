use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use ink_lang_ir::Contract;
use syn::{Result};

pub fn generate_code_for_env(_contract: &Contract, storage_ident: &syn::Ident) -> Result<TokenStream2>{
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

    Ok(env)
}