#![cfg_attr(not(feature = "std"), no_std)]

use metis_lang as metis;

#[metis::contract]
pub mod flipper {
    use super::*;

    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Flip {
        #[ink(topic)]
        from: Option<AccountId>,
        value: bool,
    }

    #[metis::supports(interface(new, default), interface(flip, get))]
    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Return the contract is support the interface_id
        #[ink(message)]
        pub fn supports_interface(&self, interface_id: u32) -> bool {
            self._supports_interface(interface_id)
        }

        /// Flips the current value of the Flipper's bool.
        #[ink(message)]
        pub fn flip(&mut self) {
            let caller = Self::env().caller();
            self.value = !self.value;

            Self::env().emit_event(Flip {
                from: Some(caller),
                value: self.value,
            });
        }

        /// Flips set the current value
        #[ink(message)]
        pub fn set(&mut self, value: bool) {
            let caller = Self::env().caller();
            self.value = value;

            Self::env().emit_event(Flip {
                from: Some(caller),
                value: self.value,
            });
        }

        /// Returns the current value of the Flipper's bool.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        #[ink::test]
        fn support_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        #[ink::test]
        fn set_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);

            flipper.set(true);
            assert_eq!(flipper.get(), true);

            flipper.set(false);
            assert_eq!(flipper.get(), false);
        }
    }
}
