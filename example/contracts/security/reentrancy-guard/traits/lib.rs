#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod stub {
    // no use trait by test not support
    // #[ink::trait_definition]
    // pub trait Flip {
    // #[ink(message)]
    // fn flip(&mut self);
    //
    // #[ink(message)]
    // fn get(&self) -> bool;
    //
    // #[ink(message)]
    // fn call_flip(&mut self, caller: AccountId);
    // }

    #[ink(storage)]
    pub struct Flipper {}

    impl Flipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            unimplemented!()
        }

        #[ink(message)]
        pub fn call_flip(&mut self, _caller: AccountId) {
            unimplemented!()
        }
    }
}
