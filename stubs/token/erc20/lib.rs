#![cfg_attr(not(feature = "std"), no_std)]

pub use self::erc20::Erc20Stub;
use ink_lang as ink;

#[ink::contract]
mod erc20 {
    use erc20_trait::{IErc20, Result};
    use ink_prelude::string::String;

    /// Basic version of Erc20Stub.
    #[ink(storage)]
    pub struct Erc20Stub {}

    impl IErc20 for Erc20Stub {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        fn new(
            _initial_supply: Balance,
            _name: Option<String>,
            _symbol: Option<String>,
            _decimals: Option<u8>,
        ) -> Self {
            unimplemented!()
        }

        /// Returns the token name.
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            unimplemented!()
        }

        /// Returns the token symbol.
        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            unimplemented!()
        }

        /// Returns the token decimals.
        #[ink(message)]
        fn token_decimals(&self) -> Option<u8> {
            unimplemented!()
        }

        /// Returns the total token supply.
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            unimplemented!()
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        fn balance_of(&self, _owner: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message)]
        fn transfer(&mut self, _to: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        #[ink(message)]
        fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        #[ink(message)]
        fn transfer_from(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _value: Balance,
        ) -> Result<()> {
            unimplemented!()
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        fn approve(&mut self, _spender: AccountId, _value: Balance) -> Result<()> {
            unimplemented!()
        }
    }
}
