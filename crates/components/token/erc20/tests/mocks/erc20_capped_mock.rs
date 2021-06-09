#![cfg_attr(not(feature = "std"), no_std)]

pub mod erc20_capped {
    #[metis_lang::contract]
    pub mod erc20_capped {
        use erc20::Result;
        use metis_erc20 as erc20;
        use metis_lang::{
            import,
            metis,
        };
        use erc20::capped;

        /// A simple ERC-20 contract.
        #[ink(storage)]
        #[import(erc20, capped)]
        pub struct Erc20Capped {
            erc20: erc20::Data<Erc20Capped>,
            capped: capped::Data<Erc20Capped>,
        }

        // TODO: gen by marco with erc20 component
        impl erc20::hookable::Impl<Erc20Capped> for Erc20Capped {
            fn before_token_transfer(
                &mut self,
                _from: &AccountId,
                _to: &AccountId,
                _amount: Balance,
            ) -> Result<()> {
                Ok(())
            }
        }
        // burnable
        impl capped::Impl<Erc20Capped> for Erc20Capped {}

        /// Event emitted when a token transfer occurs.
        #[ink(event)]
        #[metis(erc20)]
        pub struct Transfer {
            #[ink(topic)]
            pub from: Option<AccountId>,
            #[ink(topic)]
            pub to: Option<AccountId>,
            pub value: Balance,
        }

        /// Event emitted when an approval occurs that `spender` is allowed to withdraw
        /// up to the amount of `value` tokens from `owner`.
        #[ink(event)]
        #[metis(erc20)]
        pub struct Approval {
            #[ink(topic)]
            pub owner: AccountId,
            #[ink(topic)]
            pub spender: AccountId,
            pub value: Balance,
        }

        // impl
        impl Erc20Capped {
            #[ink(constructor)]
            pub fn new(name: String, symbol: String, initial_supply: Balance, cap_supply: Balance) -> Self {
                let mut instance = Self {
                    erc20: erc20::Data::new(),
                    capped: capped::Data::new(cap_supply),
                };

                erc20::Impl::init(&mut instance, name, symbol, initial_supply);
                instance
            }

            // ERC20 messages
            #[ink(message)]
            pub fn name(&self) -> String {
                erc20::Impl::name(self)
            }

            #[ink(message)]
            pub fn symbol(&self) -> String {
                erc20::Impl::symbol(self)
            }

            #[ink(message)]
            pub fn decimals(&self) -> u8 {
                erc20::Impl::decimals(self)
            }

            #[ink(message)]
            pub fn total_supply(&self) -> Balance {
                erc20::Impl::total_supply(self)
            }

            #[ink(message)]
            pub fn balance_of(&self, owner: AccountId) -> Balance {
                erc20::Impl::balance_of(self, &owner)
            }

            #[ink(message)]
            pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
                erc20::Impl::allowance(self, &owner, &spender)
            }

            #[ink(message)]
            pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
                erc20::Impl::transfer(self, &to, value)
            }

            #[ink(message)]
            pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
                erc20::Impl::approve(self, &spender, value)
            }

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
            pub fn cap(& self) -> Balance {
                capped::Impl::cap(self)
            }

            #[ink(message)]
            pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
                // TODO: its maybe forget by developers: not erc20::Impl::_mint(self, &to, value)
                capped::Impl::_mint(self, &to, value)
            }

            #[ink(message)]
            pub fn burn(&mut self, to: AccountId, value: Balance) -> Result<()> {
                erc20::Impl::_burn(self, &to, value)
            }

            #[ink(message)]
            pub fn transfer_internal(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
            ) -> Result<()> {
                erc20::Impl::_transfer_from_to(self, &from, &to, value)
            }

            #[ink(message)]
            pub fn approve_internal(
                &mut self,
                owner: AccountId,
                spender: AccountId,
                value: Balance,
            ) {
                erc20::Impl::_approve(self, &owner, &spender, value)
            }
        }
    }
}
