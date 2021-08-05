#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod erc20_contract {
    use erc20::{
        capped,
        Result,
    };
    use ink_prelude::string::String;
    use metis_erc20 as erc20;
    use metis_lang::{
        import,
        metis,
    };

    // A Example for erc20 contract
    #[ink(storage)]
    #[import(erc20, capped)]
    pub struct Erc20Capped {
        erc20: erc20::Data<Erc20Capped>,
        capped: capped::Data<Erc20Capped>,
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::hookable::Impl<Erc20Capped> for Erc20Capped {
        fn before_token_transfer(
            &mut self,
            _from: &AccountId,
            _to: &AccountId,
            _amount: &Balance,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::capped::Impl<Erc20Capped> for Erc20Capped {}

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

    // impl
    impl Erc20Capped {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, max_supply: Balance) -> Self {
            let mut instance = Self {
                erc20: erc20::Data::new(),
                capped: capped::Data::new(max_supply),
            };

            erc20::Impl::init(
                &mut instance,
                String::from("MetisTestToken"),
                String::from("MET"),
                18_u8,
                initial_supply,
            );

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

        /// Return the cap of balance supply
        #[ink(message)]
        pub fn cap(&self) -> Balance {
            capped::Impl::cap(self)
        }

        /// mint tokens for test
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            // WARNNING: this not by role control, in real world contract, it need
            // TODO: its maybe forget by developers: not erc20::Impl::_mint(self, &to, value)
            capped::Impl::_mint(self, to, value)
        }
    }
}
