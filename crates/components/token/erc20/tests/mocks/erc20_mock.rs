#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod erc20_contract {
    use super::super::behavior;
    pub use erc20::{
        Error,
        Result,
    };
    use metis_erc20 as erc20;
    use metis_lang::{
        import,
        metis,
    };

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[import(erc20)]
    pub struct Erc20 {
        erc20: erc20::Data<Erc20>,
    }

    // TODO: gen by marco with erc20 component
    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::Impl<Erc20> for Erc20 {}

    type Event = <Erc20 as ink_lang::BaseEvent>::Type;

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

    impl behavior::IERC20New<Erc20> for Erc20 {
        fn new_erc20(name: String, symbol: String, initial_supply: Balance) -> Self {
            Self::new(name, symbol, initial_supply)
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

    impl behavior::IERC20Event<Erc20> for Erc20 {
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
    impl Erc20 {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, initial_supply: Balance) -> Self {
            let mut instance = Self {
                erc20: erc20::Data::new(),
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
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::_mint(self, &to, value)
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
        ) -> Result<()> {
            erc20::Impl::_approve(self, &owner, &spender, value)
        }
    }
}
