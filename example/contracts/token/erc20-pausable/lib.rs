#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod erc20_contract {
    use ink_prelude::string::String;
    use metis_lang::{
        import,
        metis,
    };

    use metis_erc20 as erc20;
    use metis_ownable as ownable;
    use metis_pausable as pausable;

    use erc20::Result;

    // A Example for erc20 contract
    #[ink(storage)]
    #[import(erc20, ownable, pausable)]
    pub struct Erc20Pausable {
        erc20: erc20::Data<Erc20Pausable>,
        ownable: ownable::Data<Erc20Pausable>,
        pausable: pausable::Data,
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::pausable::Impl<Erc20Pausable> for Erc20Pausable {}

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[metis(erc20)]
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
    #[metis(erc20)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
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

    // impl
    impl Erc20Pausable {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut instance = Self {
                erc20: erc20::Data::new(),
                ownable: ownable::Data::new(),
                pausable: pausable::Data::new(),
            };

            erc20::Impl::init(
                &mut instance,
                String::from("MetisTestToken"),
                String::from("MET"),
                18_u8,
                initial_supply,
            );
            ownable::Impl::init(&mut instance);
            pausable::Impl::init(&mut instance);

            instance
        }

        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            erc20::Impl::name(self)
        }

        /// Returns the symbol of the token, usually a shorter version of the name.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc20::Impl::symbol(self)
        }

        /// Returns the number of decimals used to
        /// get its user representation.
        /// For example, if `decimals` equals `2`,
        /// a balance of `505` tokens should
        /// be displayed to a user as `5,05` (`505 / 10 ** 2`).
        ///
        /// Tokens usually opt for a value of 18,
        /// imitating the relationship between
        /// Ether and Wei in ETH. This is the value {ERC20} uses,
        /// unless this function is
        /// overridden;
        ///
        /// NOTE: This information is only used for _display_ purposes:
        /// it in no way affects any of the arithmetic of the contract
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            erc20::Impl::decimals(self)
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            erc20::Impl::total_supply(self)
        }

        /// Returns the account balance for the specified `owner`.
        ///
        /// Returns `0` if the account is non-existent.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            erc20::Impl::balance_of(self, owner)
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        ///
        /// Returns `0` if no allowance has been set `0`.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            erc20::Impl::allowance(self, owner, spender)
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
            erc20::Impl::transfer(self, to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        ///
        /// If this function is called again it overwrites the current allowance with `value`.
        ///
        /// An `Approval` event is emitted.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::approve(self, spender, value)
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
            erc20::Impl::transfer_from(self, from, to, value)
        }

        /// Return the owner AccountId
        #[ink(message)]
        pub fn owner(&self) -> Option<AccountId> {
            *ownable::Impl::owner(self)
        }

        /// Leaves the contract without owner. It will not be possible to call
        /// `ensure_xxx` functions anymore. Can only be called by the current owner.
        /// NOTE: Renouncing ownership will leave the contract without an owner,
        /// thereby removing any functionality that is only available to the owner.
        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            ownable::Impl::renounce_ownership(self)
        }

        /// Transfers ownership of the contract to a new account (`new_owner`).
        /// Can only be called by the current owner.
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            ownable::Impl::transfer_ownership(self, &new_owner)
        }

        /// Returns true if the contract is paused, and false otherwise
        #[ink(message)]
        pub fn paused(&self) -> bool {
            pausable::Impl::paused(self)
        }

        /// Pause the contract, will emit the `Paused` Event
        ///
        /// Requirements:
        ///
        /// - The contract must be not paused.
        /// - The caller should be the owner of contract
        #[ink(message)]
        pub fn pause(&mut self) {
            ownable::Impl::ensure_caller_is_owner(self);
            pausable::Impl::_pause(self)
        }

        /// Unpause the contract, will emit the `Unpaused` Event
        ///
        /// Requirements:
        ///
        /// - The contract must be paused.
        /// - The caller should be the owner of contract
        #[ink(message)]
        pub fn unpause(&mut self) {
            ownable::Impl::ensure_caller_is_owner(self);
            pausable::Impl::_unpause(self)
        }
    }
}
