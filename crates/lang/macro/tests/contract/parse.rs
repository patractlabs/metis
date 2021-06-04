#[metis_util_macro::contract]
pub mod erc20ownable {
    use erc20::Result;
    use metis_erc20 as erc20;
    use metis_ownable as ownable;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ::ink_lang::{EmitEvent, Env, StaticEnv};

    /// A simple ERC-20 contract.
    #[ink(storage)]
    pub struct Erc20Ownable {
        data_erc20: erc20::Data<Erc20Ownable>,
        data_owner: ownable::Data<Erc20Ownable>,
    }

    // TODO: Make by macro
    #[cfg(not(feature = "ink-as-dependency"))]
    impl metis_contract::Env for Erc20Ownable {
        type AccountId = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::AccountId;
        type Balance = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Balance;
        type Hash = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Hash;
        type Timestamp = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Timestamp;
        type BlockNumber = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::BlockNumber;
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl metis_contract::EnvAccess<Erc20Ownable> for Erc20Ownable {
        fn caller() -> <Erc20Ownable as metis_contract::Env>::AccountId {
            Self::env().caller()
        }

        fn transferred_balance() -> <Erc20Ownable as metis_contract::Env>::Balance {
            Self::env().transferred_balance()
        }
    }
    // TODO: Make by macro

    // ERC20 Impl
    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::Storage<Erc20Ownable> for Erc20Ownable {
        fn get(&self) -> &erc20::Data<Erc20Ownable> {
            &self.data_erc20
        }

        fn get_mut(&mut self) -> &mut erc20::Data<Erc20Ownable> {
            &mut self.data_erc20
        }
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }
    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::EventEmit<Erc20Ownable> for Erc20Ownable {
        fn emit_event_transfer(
            &mut self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            value: Balance,
        ) {
            self.env().emit_event(Transfer { from, to, value });
        }

        fn emit_event_approval(&mut self, owner: AccountId, spender: AccountId, value: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
        }
    }
    // TODO: Event in ink!

    // Owner Impl
    #[cfg(not(feature = "ink-as-dependency"))]
    impl ownable::Storage<Erc20Ownable> for Erc20Ownable {
        fn get(&self) -> &ownable::Data<Erc20Ownable> {
            &self.data_owner
        }

        fn get_mut(&mut self) -> &mut ownable::Data<Erc20Ownable> {
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
    impl ownable::EventEmit<Erc20Ownable> for Erc20Ownable {
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

    // impl
    impl Erc20Ownable {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut instance = Self {
                data_erc20: erc20::Data::new(),
                data_owner: ownable::Data::new(),
            };

            erc20::Impl::init(&mut instance, initial_supply);
            ownable::Impl::init(&mut instance);
            instance
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            erc20::Storage::get(self).total_supply()
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            erc20::Storage::get(self).balance_of(&owner)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            erc20::Storage::get(self).allowance(&owner, &spender)
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the caller's account balance.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::transfer(self, &to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::approve(self, &spender, value)
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        ///
        /// This can be used to allow a contract to transfer tokens on ones behalf and/or
        /// to charge fees in sub-currencies, for example.
        ///
        /// On success a `Transfer` event is emitted.
        ///
        /// # Errors
        ///
        /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
        /// for the caller to withdraw from `from`.
        ///
        /// Returns `InsufficientBalance` error if there are not enough tokens on
        /// the the account balance of `from`.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc20::Impl::transfer_from(self, &from, &to, value)
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
}

fn main() {}
