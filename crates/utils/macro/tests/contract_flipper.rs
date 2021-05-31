#[metis_util_macro::contract]
mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        data_owner: metis_ownable::Data<Flipper>, // Need generate?
        value: bool,
    }

    /// Event emitted when Owner AccountId Transferred
    #[ink(event)]
    pub struct OwnershipTransferred {
        /// previous owner account id
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        /// new owner account id
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    impl Flipper {
        // Need generate III -------------------------------------------
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self {
                data_owner: metis_ownable::Data::new(),
                value: init_value,
            };

            metis_ownable::Impl::init(&mut instance);
            instance
        }
        // Need generate III -------------------------------------------

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

    // Need generate II Owner -------------------------------------------
    #[cfg(not(feature = "ink-as-dependency"))]
    use metis_ownable;

    #[cfg(not(feature = "ink-as-dependency"))]
    impl metis_ownable::EventEmit<Flipper> for Flipper {
        fn emit_event_ownership_transferred(
            &mut self,
            previous_owner: Option<AccountId>,
            new_owner: Option<AccountId>,
        ) {
            self.env().emit_event(OwnershipTransferred {
                previous_owner,
                new_owner,
            });
        }
    }
    // Need generate II Owner -------------------------------------------

    // Need generate I Owner -------------------------------------------
    #[cfg(not(feature = "ink-as-dependency"))]
    impl metis_ownable::Storage<Flipper> for Flipper {
        fn get(&self) -> &metis_ownable::Data<Flipper> {
            &self.data_owner
        }

        fn get_mut(&mut self) -> &mut metis_ownable::Data<Flipper> {
            &mut self.data_owner
        }
    }
    // Need generate I Owner -------------------------------------------

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