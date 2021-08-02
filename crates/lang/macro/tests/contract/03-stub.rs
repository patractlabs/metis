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
        #[ink(message)]
        pub fn do_sth(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _data: Vec<u8>,
        ) -> [u8; 4] {
            unimplemented!()
        }

    }
}

fn main() {}
