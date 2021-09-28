#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use metis_erc721_types::TokenId;

#[ink::contract]
mod stub {
    use ink_prelude::vec::Vec;

    use super::TokenId;

    #[ink(storage)]
    #[metis_lang::stub]
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
}

pub use stub::ERC721Receiver as ERC721ReceiverStub;
