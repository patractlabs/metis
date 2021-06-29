#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod stub {
    #[ink(storage)]
    pub struct EscrowStub {}

    // impl
    impl EscrowStub {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn deposits_of(&self, _payee: AccountId) -> Balance {
            unimplemented!()
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self, _payee: AccountId) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn withdraw(&mut self, _payee: AccountId) {
            unimplemented!()
        }
    }
}

pub use stub::EscrowStub;