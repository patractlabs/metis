#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use metis_erc721_types::TokenId;

#[ink::contract]
mod stub {
    use ink_lang as ink;
    use ink_prelude::vec::Vec;

    use super::TokenId;

    #[ink::trait_definition]
    pub trait IERC721Receiver {
        #[ink(message)]
        fn on_erc721_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            token_id: TokenId,
            data: Vec<u8>,
        ) -> [u8; 4];
    }

    #[ink(storage)]
    pub struct ERC721Receiver {}

    impl ERC721Receiver {
        #[ink(constructor)]
        pub fn default() -> Self {
            unimplemented!()
        }
        #[ink(message)]
        pub fn on_erc721_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _token_id: TokenId,
            _data: Vec<u8>,
        ) -> [u8; 4] {
            unimplemented!()
        }

    }

    #[cfg(feature = "ink-as-dependency")]
    const _: () = {
        // TODO: use marco to gen code
        impl metis_lang::Env for ERC721Receiver {
            type AccountId =
                <::ink_env::DefaultEnvironment as ::ink_env::Environment>::AccountId;
            type Balance =
                <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Balance;
            type Hash = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Hash;
            type Timestamp =
                <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Timestamp;
            type BlockNumber =
                <::ink_env::DefaultEnvironment as ::ink_env::Environment>::BlockNumber;
        }

        impl<E> metis_lang::FromAccountId<E> for ERC721Receiver
        where
            E: metis_lang::Env,
        {
            /// Creates the contract instance from the account ID of the already instantiated contract.
            fn from_account_id(account_id: E::AccountId) -> Self {
                <ERC721Receiver as ::ink_env::call::FromAccountId<
                    ink_env::DefaultEnvironment,
                >>::from_account_id(account_id.into())
            }
        }
    };
}

pub use stub::{
    ERC721Receiver as ERC721ReceiverStub,
};
