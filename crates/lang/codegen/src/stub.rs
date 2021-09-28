use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Result;

pub fn generate_code(_attr: TokenStream2, input: TokenStream2) -> Result<TokenStream2> {
    let typ = syn::parse2::<syn::ItemStruct>(input.clone())?;
    let ident = typ.ident.clone();

    Ok(quote! {
        #input

        #[cfg(feature = "ink-as-dependency")]
        const _: () = {
            impl metis_lang::Env for #ident {
                type AccountId = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::AccountId;
                type Balance = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Balance;
                type Hash = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Hash;
                type Timestamp = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Timestamp;
                type BlockNumber = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::BlockNumber;
            }

            impl<E> metis_lang::FromAccountId<E> for #ident
            where
                E: metis_lang::Env,
            {
                /// Creates the contract instance from the account ID of the already instantiated contract.
                fn from_account_id(account_id: E::AccountId) -> Self {
                    <#ident as ::ink_env::call::FromAccountId<
                        ink_env::DefaultEnvironment,
                    >>::from_account_id(account_id.into())
                }
            }
        };
    })
}
