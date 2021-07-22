#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod contract {
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use metis_erc777 as erc777;
    pub use metis_erc777::{
        Error,
        Result,
    };
    use metis_lang::{
        import,
        metis,
    };

    #[ink(storage)]
    #[import(erc777)]
    pub struct Erc777 {
        erc777: erc777::Data<Erc777>,
    }

    impl erc777::Impl<Erc777> for Erc777 {
        fn _before_token_transfer(
            &mut self,
            _operator: &AccountId,
            _from: &Option<&AccountId>,
            _to: &Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<()> {
            Ok(())
        }
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[metis(erc777)]
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
    #[metis(erc777)]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: AccountId,
        pub value: Balance,
    }

    #[ink(event)]
    #[metis(erc777)]
    pub struct Sent {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub from: AccountId,
        #[ink(topic)]
        pub to: AccountId,
        pub amount: Balance,
        pub data: Vec<u8>,
        pub operator_data: Vec<u8>,
    }

    #[ink(event)]
    #[metis(erc777)]
    pub struct Minted {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub to: AccountId,
        pub amount: Balance,
        pub data: Vec<u8>,
        pub operator_data: Vec<u8>,
    }

    #[ink(event)]
    #[metis(erc777)]
    pub struct Burned {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub from: AccountId,
        pub amount: Balance,
        pub data: Vec<u8>,
        pub operator_data: Vec<u8>,
    }

    #[ink(event)]
    #[metis(erc777)]
    pub struct AuthorizedOperator {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub token_holder: AccountId,
    }

    #[ink(event)]
    #[metis(erc777)]
    pub struct RevokedOperator {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub token_holder: AccountId,
    }

    // for test message
    impl Erc777 {
        /// For test to mint
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc777::Impl::_mint(self, to, value, Vec::default(), Vec::default())
        }
    }

    // impl
    impl Erc777 {
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            decimals: u8,
            initial_supply: Balance,
        ) -> Self {
            let mut instance = Self {
                erc777: erc777::Data::new(),
            };

            erc777::Impl::init(&mut instance, name, symbol, decimals, initial_supply);
            instance
        }


        #[ink(message)]
        pub fn name(&self) -> String {
            erc777::Impl::name(self)
        }

        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc777::Impl::symbol(self)
        }

        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            erc777::Impl::decimals(self)
        }

        #[ink(message)]
        pub fn granularity(&self) -> Balance {
            erc777::Impl::granularity(self)
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            erc777::Impl::total_supply(self)
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            erc777::Impl::balance_of(self, &owner)
        }

        #[ink(message)]
        pub fn send(
            &mut self,
            recipient: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<()> {
            erc777::Impl::send(self, recipient, amount, data)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc777::Impl::transfer(self, &to, value)
        }

        #[ink(message)]
        pub fn burn(&mut self, amount: Balance, data: Vec<u8>) -> Result<()> {
            erc777::Impl::burn(self, amount, data)
        }
    
        #[ink(message)]
        pub fn is_operator_for(&self, operator: AccountId, token_holder: AccountId) -> bool {
            erc777::Impl::is_operator_for(self, operator, token_holder)
        }

        #[ink(message)]
        pub fn authorize_operator(&mut self, operator: AccountId) {
            erc777::Impl::authorize_operator(self, operator)
        }

        #[ink(message)]
        pub fn revoke_operator(&mut self, operator: AccountId) {
            erc777::Impl::revoke_operator(self, operator)
        }


        #[ink(message)]
        pub fn default_operators(&self) -> Vec<AccountId> {
            erc777::Impl::default_operators(self)
        }

        #[ink(message)]
        pub fn operator_send(
            &mut self,
            sender: AccountId,
            recipient: AccountId,
            amount: Balance,
            data: Vec<u8>,
            operator_data: Vec<u8>,
        ) -> Result<()> {
            erc777::Impl::operator_send(self, sender, recipient, amount, data, operator_data)
        }

        #[ink(message)]
        pub fn operator_burn(
            &mut self,
            account: AccountId,
            amount: Balance,
            data: Vec<u8>,
            operator_data: Vec<u8>,
        ) -> Result<()> {
            erc777::Impl::operator_burn(self, account, amount, data, operator_data)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            erc777::Impl::allowance(self, &owner, &spender)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            erc777::Impl::approve(self, &spender, value)
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc777::Impl::transfer_from(self, &from, &to, value)
        }

        #[ink(message)]
        pub fn transfer_internal(
            &mut self,
            spender: AccountId,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc777::Impl::_move(self, &spender, &from, &to, &value, &Vec::default(), &Vec::default())
        }

        #[ink(message)]
        pub fn approve_internal(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc777::Impl::_approve(self, &owner, &spender, value)
        }


    }
}
