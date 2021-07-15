#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use metis_erc721_types::TokenId;

#[ink::contract]
mod stub {
    use ink_prelude::vec::Vec;

    use super::TokenId;

    #[ink(storage)]
    pub struct ERC1155Receiver {}

    impl ERC1155Receiver {
        #[ink(constructor)]
        pub fn default() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn on_erc1155_received(
            &mut self,
            _operator: AccountId,
            _from: Option<AccountId>,
            _id: TokenId,
            _value: Balance,
            _data: Vec<u8>,
        ) -> [u8; 4] {
            // [194u8, 238u8, 217u8, 152u8]
            unimplemented!()
        }

        #[ink(message)]
        pub fn on_erc1155_batch_received(
            &mut self,
            _operator: AccountId,
            _from: Option<AccountId>,
            _ids: Vec<TokenId>,
            _values: Vec<Balance>,
            _data: Vec<u8>,
        ) -> [u8; 4] {
            // [22u8, 32u8, 73u8, 133u8]
            unimplemented!()
        }
    }

    #[cfg(feature = "ink-as-dependency")]
    const _: () = {
        // TODO: use marco to gen code
        impl metis_lang::Env for ERC1155Receiver {
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

        impl<E> metis_lang::FromAccountId<E> for ERC1155Receiver
        where
            E: metis_lang::Env,
        {
            /// Creates the contract instance from the account ID of the already instantiated contract.
            fn from_account_id(account_id: E::AccountId) -> Self {
                <ERC1155Receiver as ::ink_env::call::FromAccountId<
                    ink_env::DefaultEnvironment,
                >>::from_account_id(account_id.into())
            }
        }
    };
}

pub use stub::{
    ERC1155Receiver as ERC1155ReceiverStub,
};
