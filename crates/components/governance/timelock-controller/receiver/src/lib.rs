#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod stub {
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    #[metis_lang::stub]
    pub struct Receiver {}

    impl Receiver {
        #[ink(constructor)]
        pub fn default() -> Self {
            unimplemented!()
        }

        #[ink(message, payable)]
        pub fn on_call(&mut self, _operator: AccountId, _data: Vec<u8>) -> bool {
            unimplemented!()
        }
    }
}

pub use stub::Receiver;
