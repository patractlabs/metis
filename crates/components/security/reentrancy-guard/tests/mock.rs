#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
mod flipper {
    use metis_lang::{
        import,
        metis,
    };
    
    use metis_ownable as ownable;
    use metis_reentrancy_guard as reentrancy_guard;

    #[ink(storage)]
    #[import(reentrancy_guard, ownable)]
    pub struct Flipper {
        reentrancy_guard: reentrancy_guard::Data,
        ownable: ownable::Data<Flipper>,

        value: bool,
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
                reentrancy_guard: reentrancy_guard::Data::default(),
                ownable: ownable::Data::default(),

                value: init_value,
            };

            ownable::Impl::init(&mut instance);
            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false)
        }

        #[ink(message)]
        #[metis_lang::reentrancy_guard]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        #[metis_lang::reentrancy_guard]
        pub fn flip_panic(&mut self) {
            self.value = !self.value;
            self.flip();
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
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
        fn call_reentrant_should_panic(){
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip_panic();
            assert_eq!(flipper.get(), true);
        }
    }
}
