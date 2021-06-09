#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod erc20_mock;
}

mod utils {
    pub mod event;
}

/// Unit tests.
#[cfg(test)]
mod erc20_basic_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use ink_lang as ink;
    use ink_prelude::string::String;
    use utils::event::{
        PrefixedValue,
        assert_emitted_event_len,
        encoded_into_hash,
    };

    use ink::ContractEnv;
    use mocks::erc20_mock::erc20_contract::*;

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
}
