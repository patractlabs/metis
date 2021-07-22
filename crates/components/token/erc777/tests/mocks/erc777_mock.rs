#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod erc777_contract {
    use super::super::behavior;
    pub use erc777::{
        Error,
        Result,
    };
    use metis_erc777 as erc777;
    use metis_lang::{
        import,
        metis,
    };

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[import(erc777)]
    pub struct Erc777 {
        erc777: erc777::Data<Erc777>,
    }

    // TODO: gen by marco with erc777 component
    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc777::Impl<Erc777> for Erc777 {}

    type Event = <Erc777 as ink_lang::BaseEvent>::Type;

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

    impl behavior::IERC20New<Erc777> for Erc777 {
        fn new_erc20(
            name: String,
            symbol: String,
            decimals: u8,
            initial_supply: Balance,
        ) -> Self {
            Self::new(name, symbol, decimals, initial_supply)
        }

        fn next_call_by(account: AccountId) {
            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call.
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4]));

            data.push_arg(&account.clone());

            // Push the new execution context to set from as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                account.clone(),
                callee,
                1000000,
                1000000,
                data,
            );
        }
    }

    impl behavior::IERC20Event<Erc777> for Erc777 {
        fn decode_transfer_event(
            event: &ink_env::test::EmittedEvent,
        ) -> (Option<AccountId>, Option<AccountId>, Balance) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                return (from, to, value)
            }
            panic!("encountered unexpected event kind: expected a Transfer event")
        }

        fn decode_approval_event(
            event: &ink_env::test::EmittedEvent,
        ) -> (AccountId, AccountId, Balance) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Approval(Approval {
                owner,
                spender,
                value,
            }) = decoded_event
            {
                return (owner, spender, value)
            }
            panic!("encountered unexpected event kind: expected a Transfer event")
        }

        fn assert_topics(
            event: &ink_env::test::EmittedEvent,
            expected_topics: &Vec<Hash>,
        ) {
            for (n, (actual_topic, expected_topic)) in
                event.topics.iter().zip(expected_topics).enumerate()
            {
                let topic = actual_topic
                    .decode::<Hash>()
                    .expect("encountered invalid topic encoding");
                assert_eq!(topic, *expected_topic, "encountered invalid topic at {}", n);
            }
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
        pub fn is_operator_for(
            &self,
            operator: AccountId,
            token_holder: AccountId,
        ) -> bool {
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
            erc777::Impl::operator_send(
                self,
                sender,
                recipient,
                amount,
                data,
                operator_data,
            )
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
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc777::Impl::_mint(self, to, value, Vec::default(), Vec::default())
        }

        #[ink(message)]
        pub fn transfer_internal(
            &mut self,
            spender: AccountId,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc777::Impl::_move(
                self,
                &spender,
                &from,
                &to,
                &value,
                &Vec::default(),
                &Vec::default(),
            )
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
