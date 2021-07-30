#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod flipper_caller {
    use ink_env::call::FromAccountId;
    use reentrancy_guard_traits::{
        stub::Flipper,
    };

    #[ink(storage)]
    pub struct Caller {
        flipper_account: AccountId,
        call_type: u8,
        called: bool,
    }

    pub const CALL_SAME: u8 = 1;
    pub const CALL_OTHER: u8 = 2;

    impl Caller {
        #[ink(constructor)]
        pub fn new(flipper: AccountId) -> Self {
            let instance = Self {
                flipper_account: flipper,
                call_type: CALL_SAME,
                called: false,
            };

            instance
        }

        #[ink(message)]
        pub fn clear_account(&mut self, flipper: AccountId) {
            self.flipper_account = flipper;
            self.called = false;
        }

        #[ink(message)]
        pub fn set_call_type(&mut self, typ: u8) {
            self.call_type = typ;
        }

        #[ink(message)]
        pub fn do_sth(&mut self) {
            assert!(!self.called, "has called, should clear account state");
            assert!(
                self.flipper_account != AccountId::default(),
                "should set accountID"
            );

            let mut flipper = <Flipper>::from_account_id(self.flipper_account);
            <Flipper>::call_flip(&mut flipper, self.env().account_id());
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            if !self.called {
                self.called = true;

                let mut flipper = <Flipper>::from_account_id(self.flipper_account);
                match self.call_type {
                    CALL_SAME => <Flipper>::call_flip(&mut flipper, self.env().account_id()),
                    CALL_OTHER => <Flipper>::flip(&mut flipper),
                    _ => return
                }
            }
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            false
        }

        #[ink(message)]
        pub fn call_flip(&mut self, _caller: AccountId) {}
    }
}
