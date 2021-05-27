#[metis_util_macro::contract]
mod flipper {
    use metis_ownable as ownable;

    #[ink(storage)]
    pub struct Flipper {
        data_owner: ownable::Data<Flipper>,
        value: bool,
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    use ::ink_lang::{EmitEvent, Env, StaticEnv};

    #[cfg(not(feature = "ink-as-dependency"))]
    impl metis_contract::Env for Flipper {
        type BaseEvent = <Flipper as ::ink_lang::BaseEvent>::Type;
        type AccountId = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::AccountId;
        type Balance = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Balance;
        type Hash = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Hash;
        type Timestamp = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Timestamp;
        type BlockNumber = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::BlockNumber;
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl metis_contract::EnvAccess<Flipper> for Flipper {
        fn caller() -> <Flipper as metis_contract::Env>::AccountId {
            Self::env().caller()
        }

        fn transferred_balance() -> <Flipper as metis_contract::Env>::Balance {
            Self::env().transferred_balance()
        }
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl ownable::Storage<Flipper> for Flipper {
        fn get(&self) -> &ownable::Data<Flipper> {
            &self.data_owner
        }

        fn get_mut(&mut self) -> &mut ownable::Data<Flipper> {
            &mut self.data_owner
        }
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

    #[cfg(not(feature = "ink-as-dependency"))]
    impl ownable::EventEmit<Flipper> for Flipper {
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

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self { 
                data_owner: ownable::Data::new(),
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
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn get_ownership(&self) -> Option<AccountId> {
            *ownable::Storage::get(self).get_ownership()
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
        use ink_env::{
            hash::{
                Blake2x256,
                CryptoHash,
                HashOutput,
            },
            Clear,
        };

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        #[ink::test]
        fn owner_works() {
            let caller = AccountId::from([0x01; 32]);
            let to = AccountId::from([0x02; 32]);

            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get_ownership(), Some(caller));

            flipper.transfer_ownership(to);
            assert_eq!(flipper.get_ownership(), Some(to));

            flipper.renounce_ownership();

            assert_eq!(flipper.get_ownership(), None);
        }
    }
}

fn main() {}
