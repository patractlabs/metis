#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod flipper {
    use ink_env::call::FromAccountId;
    use metis_lang::{
        import,
        metis,
    };
    use metis_ownable as ownable;
    use metis_reentrancy_guard as reentrancy_guard;
    use reentrancy_guard_traits::{
        stub::Flipper as FlipperStub,
    };

    #[ink(storage)]
    #[import(reentrancy_guard, ownable)]
    pub struct Flipper {
        ownable: ownable::Data<Flipper>,
        value: bool,
        reentrancy_guard: reentrancy_guard::Data,
    }

    /// Event emitted when Owner AccountId Transferred
    #[ink(event)]
    #[metis(ownable)]
    pub struct OwnershipTransferred {
        /// previous owner account id
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        /// new owner account id
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self {
                ownable: ownable::Data::default(),
                value: init_value,
                reentrancy_guard: reentrancy_guard::Data::default(),
            };

            ownable::Impl::init(&mut instance);
            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false)
        }

        #[ink(message)]
        pub fn flip_panic(&mut self) {
            reentrancy_guard::Impl::_check_nonreentrant(self);
            reentrancy_guard::Impl::_set_entered(self);

            self.value = !self.value;
            self.flip();

            reentrancy_guard::Impl::_set_not_entered(self);
        }

        #[ink(message)]
        pub fn get_ownership(&self) -> Option<AccountId> {
            *ownable::Impl::owner(self)
        }

        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            ownable::Impl::renounce_ownership(self)
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            ownable::Impl::transfer_ownership(self, &new_owner)
        }
    
        #[ink(message)]
        #[metis_lang::reentrancy_guard]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        #[metis_lang::reentrancy_guard]
        pub fn call_flip(&mut self, caller: AccountId) {
            self.value = !self.value;

            let mut flipper = <FlipperStub>::from_account_id(caller);
            <FlipperStub>::flip(&mut flipper);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        #[ink::test]
        #[should_panic]
        fn call_reentrant_should_panic() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip_panic();
            assert_eq!(flipper.get(), true);
        }
    }
}
