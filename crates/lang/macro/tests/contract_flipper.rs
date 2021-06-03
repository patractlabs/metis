use metis_lang_macro::{contract, import, metis};

#[contract]
mod flipper {
    use super::*;

    #[ink(storage)]
    #[import(metis_ownable)]
    pub struct Flipper {
        metis_ownable: metis_ownable::Data<Flipper>,

        value: bool,
    }

    /// Event emitted when Owner AccountId Transferred
    #[ink(event)]
    #[metis(metis_ownable)]
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
                metis_ownable: metis_ownable::Data::default(),

                value: init_value,
            };

            metis_ownable::Impl::init(&mut instance);
            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false)
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn get_ownership(&self) -> Option<AccountId> {
            *metis_ownable::Storage::get(self).get_ownership()
        }

        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            metis_ownable::Impl::renounce_ownership(self)
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            metis_ownable::Impl::transfer_ownership(self, &new_owner)
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
        fn owner_transfer_ownership_works() {
            let caller = AccountId::from([0x01; 32]);
            let to = AccountId::from([0x02; 32]);

            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get_ownership(), Some(caller));

            flipper.transfer_ownership(to);
            assert_eq!(flipper.get_ownership(), Some(to));
        }

        #[ink::test]
        fn owner_renounce_ownership_works() {
            let caller = AccountId::from([0x01; 32]);
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get_ownership(), Some(caller));

            flipper.renounce_ownership();

            assert_eq!(flipper.get_ownership(), None);
        }
    }
}
