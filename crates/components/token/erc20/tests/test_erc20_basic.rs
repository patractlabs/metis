#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod behavior;
    pub mod erc20_mock;
}

mod utils {
    pub mod event;
}

mod erc20_basic_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use ink_lang as ink;
    use ink_prelude::string::String;
    use utils::event::*;

    use erc20_contract::{
        Erc20,
        Transfer,
        Error,
    };
    use ink::ContractEnv;
    use mocks::erc20_mock::erc20_contract;
    use mocks::behavior::Erc20BehaviorChecker;

    type AccountId = <<Erc20 as ContractEnv>::Env as ink_env::Environment>::AccountId;
    type Balance = <<Erc20 as ContractEnv>::Env as ink_env::Environment>::Balance;
    type Hash = <<Erc20 as ContractEnv>::Env as ink_env::Environment>::Hash;
    type Event = <Erc20 as ink::BaseEvent>::Type;

    fn assert_transfer_event(
        event: &ink_env::test::EmittedEvent,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_value: Balance,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
            assert_eq!(from, expected_from, "encountered invalid Transfer.from");
            assert_eq!(to, expected_to, "encountered invalid Transfer.to");
            assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
        } else {
            panic!("encountered unexpected event kind: expected a Transfer event")
        }
        let expected_topics = vec![
            encoded_into_hash(&PrefixedValue {
                value: b"Erc20::Transfer",
                prefix: b"",
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20::Transfer::from",
                value: &expected_from,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20::Transfer::to",
                value: &expected_to,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20::Transfer::value",
                value: &expected_value,
            }),
        ];
        for (n, (actual_topic, expected_topic)) in
            event.topics.iter().zip(expected_topics).enumerate()
        {
            let topic = actual_topic
                .decode::<Hash>()
                .expect("encountered invalid topic encoding");
            assert_eq!(topic, expected_topic, "encountered invalid topic at {}", n);
        }
    }

    #[ink::test]
    fn should_erc20_behavior_work() {
        let init_amount = 100000000000000000;
        let default_account = AccountId::from([0x01; 32]);
        let mut erc20 = Erc20::new(
            String::from("MockErc20Token"),
            String::from("MET"),
            init_amount,
        );
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");
        let checker = Erc20BehaviorChecker::new(
            &mut erc20,
            init_amount,
            default_account,
            accounts.alice,
            accounts.bob,
        );

        // init state
        checker.init_state_should_work();
    }

    /// The default constructor does its job.
    #[ink::test]
    fn new_works() {
        let init_amount = 100000000000000000;
        let default_account = AccountId::from([0x01; 32]);

        // Constructor works.
        let erc20 = Erc20::new(
            String::from("MockErc20Token"),
            String::from("MET"),
            init_amount,
        );

        // for emit the init transfer
        let emitted_events = assert_emitted_event_len(1);
        assert_transfer_event(
            &emitted_events[0],
            None,
            Some(default_account),
            100000000000000000,
        );

        // for metadatas
        assert_eq!(
            String::from("MockErc20Token"),
            erc20.name(),
            "name should be default"
        );
        assert_eq!(
            String::from("MET"),
            erc20.symbol(),
            "symbol should be default"
        );
        assert_eq!(18, erc20.decimals(), "default decimals should be 18");

        // for init amount
        assert_eq!(
            init_amount,
            erc20.total_supply(),
            "total amount should be default"
        );
        assert_eq!(
            init_amount,
            erc20.balance_of(default_account),
            "default account balance_of should be default"
        );
    }

    #[ink::test]
    fn mint_work() {
        let init_amount = 100000000000000000;
        let default_account = AccountId::from([0x01; 32]);

        // Constructor works.
        let mut erc20 = Erc20::new(
            String::from("MockErc20Token"),
            String::from("MET"),
            init_amount,
        );

        // Check Current Balance
        let current_total = erc20.total_supply();
        let current_balance = erc20.balance_of(default_account);

        assert_eq!(init_amount, current_total, "total amount should be default");
        assert_eq!(
            init_amount, current_balance,
            "default account balance_of should be default"
        );

        // Mint, current mint is a mock
        let mint_amount = 100000;
        assert_eq!(
            erc20.mint(default_account, mint_amount),
            Ok(()),
            "mint should be ok"
        );

        let emitted_events = get_last_emitted_event();
        assert_transfer_event(&emitted_events, None, Some(default_account), mint_amount);

        assert_eq!(
            init_amount + mint_amount,
            erc20.total_supply(),
            "total amount should be default"
        );
        assert_eq!(
            init_amount + mint_amount,
            erc20.balance_of(default_account),
            "default account balance_of should be default"
        );
    }

    #[ink::test]
    fn mint_to_other_work() {
        let init_amount = 100000000000000000;
        let default_account = AccountId::from([0x01; 32]);

        // Constructor works.
        let mut erc20 = Erc20::new(
            String::from("MockErc20Token"),
            String::from("MET"),
            init_amount,
        );

        // Check Current Balance
        let current_total = erc20.total_supply();
        let current_balance = erc20.balance_of(default_account);

        assert_eq!(init_amount, current_total, "total amount should be default");
        assert_eq!(
            init_amount, current_balance,
            "default account balance_of should be default"
        );

        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Mint, current mint is a mock
        let mint_amount = 100000;
        assert_eq!(
            erc20.mint(accounts.bob, mint_amount),
            Ok(()),
            "mint should be ok"
        );

        let emitted_events = get_last_emitted_event();
        assert_transfer_event(&emitted_events, None, Some(accounts.bob), mint_amount);

        assert_eq!(
            init_amount + mint_amount,
            erc20.total_supply(),
            "total amount should be default"
        );
        assert_eq!(
            init_amount,
            erc20.balance_of(default_account),
            "default account balance_of should be default"
        );
        assert_eq!(
            mint_amount,
            erc20.balance_of(accounts.bob),
            "default account balance_of should be default"
        );
    }

    #[ink::test]
    fn mint_to_nil_account_should_error() {
        let init_amount = 100000000000000000;
        let default_account = AccountId::from([0x01; 32]);

        // Constructor works.
        let mut erc20 = Erc20::new(
            String::from("MockErc20Token"),
            String::from("MET"),
            init_amount,
        );

        // Check Current Balance
        let current_total = erc20.total_supply();
        let current_balance = erc20.balance_of(default_account);

        assert_eq!(init_amount, current_total, "total amount should be default");
        assert_eq!(
            init_amount, current_balance,
            "default account balance_of should be default"
        );

        // Mint, current mint is a mock
        let mint_amount = 100000;
        assert_eq!(
            erc20.mint(AccountId::from([0x00; 32]), mint_amount),
            Err(Error::AccountIsZero),
            "mint should be failed by zero account"
        );


    }
}
