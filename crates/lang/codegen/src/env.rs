use super::utils::gen_cross_calling_conflict_cfg;
use ink_lang_ir::Contract;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Result;

pub fn generate_code(
    contract: &Contract,
    storage_ident: &syn::Ident,
) -> Result<TokenStream2> {
    let no_cross_calling_cfg = gen_cross_calling_conflict_cfg(contract);
    let env = quote! {
        #no_cross_calling_cfg
        use ::ink_lang::{EmitEvent, Env, StaticEnv};

        #no_cross_calling_cfg
        impl metis_lang::Env for #storage_ident {
            type AccountId = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::AccountId;
            type Balance = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Balance;
            type Hash = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Hash;
            type Timestamp = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Timestamp;
            type BlockNumber = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::BlockNumber;
        }

        #no_cross_calling_cfg
        impl metis_lang::EnvAccess<#storage_ident > for #storage_ident  {
            fn caller() -> <#storage_ident as metis_lang::Env>::AccountId {
                Self::env().caller()
            }
            fn transferred_balance() -> <#storage_ident as metis_lang::Env>::Balance {
                Self::env().transferred_balance()
            }
            fn account_id() -> <#storage_ident as metis_lang::Env>::AccountId {
                Self::env().account_id()
            }
            fn balance() -> <#storage_ident as metis_lang::Env>::Balance  {
                Self::env().balance()
            }
            fn transfer(destination: <#storage_ident as metis_lang::Env>::AccountId, value: <#storage_ident as metis_lang::Env>::Balance) -> ink_env::Result<()>{
                Self::env().transfer(destination, value)
            }
            fn block_timestamp() -> <#storage_ident as metis_lang::Env>::Timestamp{
                Self::env().block_timestamp()
            }
            fn hash_bytes<H>(input: &[u8]) -> <H as ink_env::hash::HashOutput>::Type where H : ink_env::hash::CryptoHash{
                Self::env().hash_bytes::<H>(input)
            }
        }
    };

    Ok(env)
}
