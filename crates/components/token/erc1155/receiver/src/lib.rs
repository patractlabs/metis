#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use metis_erc721_types::TokenId;

#[ink::contract]
mod stub {
    use ink_prelude::vec::Vec;

    use super::TokenId;

    #[ink(storage)]
    #[metis_lang::stub]
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
}

pub use stub::ERC1155Receiver as ERC1155ReceiverStub;
