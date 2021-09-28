#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
mod flipper {
    use metis_lang::{
        import,
        metis,
    };

    use metis_ownable as ownable;
    use metis_pausable as pausable;

    #[ink(storage)]
    #[import(pausable, ownable)]
    pub struct Flipper {
        pausable: pausable::Data,
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

    /// Event emitted when Pause
    #[ink(event)]
    #[metis(pausable)]
    pub struct Paused {
        /// paused caller
        #[ink(topic)]
        account: AccountId,
    }

    /// Event emitted when unPause
    #[ink(event)]
    #[metis(pausable)]
    pub struct Unpaused {
        /// unpaused caller
        #[ink(topic)]
        account: AccountId,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self {
                pausable: pausable::Data::default(),
                ownable: ownable::Data::default(),

                value: init_value,
            };

            pausable::Impl::init(&mut instance);
            ownable::Impl::init(&mut instance);
            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false)
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            pausable::Impl::ensure_not_paused(self);
            self.value = !self.value;
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

        #[ink(message)]
        pub fn paused(&self) -> bool {
            pausable::Impl::paused(self)
        }

        #[ink(message)]
        pub fn pause(&mut self) {
            ownable::Impl::ensure_caller_is_owner(self);
            pausable::Impl::_pause(self)
        }

        #[ink(message)]
        pub fn unpause(&mut self) {
            ownable::Impl::ensure_caller_is_owner(self);
            pausable::Impl::_unpause(self)
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

        #[ink::test]
        fn pause_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.paused(), false);

            flipper.pause();
            assert_eq!(flipper.paused(), true);

            flipper.unpause();
            assert_eq!(flipper.paused(), false);
        }

        #[ink::test]
        #[should_panic]
        fn pause_enuse_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.paused(), false);
            assert_eq!(flipper.get(), false);

            flipper.pause();
            assert_eq!(flipper.paused(), true);

            // should panic
            flipper.flip();
            assert_eq!(flipper.get(), false);

            flipper.unpause();
            assert_eq!(flipper.paused(), false);

            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        #[ink::test]
        fn unpause_enuse_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.paused(), false);
            assert_eq!(flipper.get(), false);

            flipper.pause();
            assert_eq!(flipper.paused(), true);

            // should panic
            // flipper.flip();
            assert_eq!(flipper.get(), false);

            flipper.unpause();
            assert_eq!(flipper.paused(), false);

            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
