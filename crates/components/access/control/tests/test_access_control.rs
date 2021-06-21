#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod access_control_mock;
}

mod utils;

pub use access_control_tests::{
    assert_role_admin_changed_event,
    assert_role_granted_event,
    assert_role_revoked_event,
};

mod access_control_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use access_control_mock::{
        AccessControl,
        RoleAdminChanged,
        RoleId,
        ROLE_ID_ADMIN,
        ROLE_ID_FLIPER,
        ROLE_ID_SETTER,
    };
    use ink::ContractEnv;

    use ink_lang as ink;
    use mocks::access_control_mock::access_control_mock;
    use utils::*;

    type AccountId =
        <<AccessControl as ContractEnv>::Env as ink_env::Environment>::AccountId;
    // type Balance = <<AccessControl as ContractEnv>::Env as ink_env::Environment>::Balance;
    type Hash = <<AccessControl as ContractEnv>::Env as ink_env::Environment>::Hash;
    type Event = <AccessControl as ink::BaseEvent>::Type;

    // TODO: use marco to generate this function

    /// assert the RoleAdminChanged event
    pub fn assert_role_admin_changed_event(
        event: &ink_env::test::EmittedEvent,
        expected_role: RoleId,
        expected_previous_admin_role: Option<RoleId>,
        expected_new_admin_role: RoleId,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::RoleAdminChanged(RoleAdminChanged {
            role,
            previous_admin_role,
            new_admin_role,
        }) = decoded_event
        {
            assert_eq!(
                role, expected_role,
                "encountered invalid RoleAdminChanged.role"
            );
            assert_eq!(
                previous_admin_role, expected_previous_admin_role,
                "encountered invalid RoleAdminChanged.previous_admin_role"
            );
            assert_eq!(
                new_admin_role, expected_new_admin_role,
                "encountered invalid RoleAdminChanged.new_admin_role"
            );
        } else {
            panic!("encountered unexpected event kind: expected a Transfer event")
        }
        let expected_topics = vec![
            encoded_into_hash(&PrefixedValue {
                value: b"AccessControl::RoleAdminChanged",
                prefix: b"",
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleAdminChanged::role",
                value: &expected_role,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleAdminChanged::previous_admin_role",
                value: &expected_previous_admin_role,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleAdminChanged::new_admin_role",
                value: &expected_new_admin_role,
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

    /// assert the RoleGranted event
    pub fn assert_role_granted_event(
        event: &ink_env::test::EmittedEvent,
        expected_role: RoleId,
        expected_account: AccountId,
        expected_sender: AccountId,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::RoleGranted(access_control_mock::RoleGranted {
            role,
            account,
            sender,
        }) = decoded_event
        {
            assert_eq!(
                role, expected_role,
                "encountered invalid RoleGranted.role"
            );
            assert_eq!(
                account, expected_account,
                "encountered invalid RoleGranted.account"
            );
            assert_eq!(
                sender, expected_sender,
                "encountered invalid RoleGranted.sender"
            );
        } else {
            panic!("encountered unexpected event kind: expected a Transfer event")
        }
        let expected_topics = vec![
            encoded_into_hash(&PrefixedValue {
                value: b"AccessControl::RoleGranted",
                prefix: b"",
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleGranted::role",
                value: &expected_role,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleGranted::account",
                value: &expected_account,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleGranted::sender",
                value: &expected_sender,
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

    /// assert the RoleRevoked event
    pub fn assert_role_revoked_event(
        event: &ink_env::test::EmittedEvent,
        expected_role: RoleId,
        expected_account: AccountId,
        expected_sender: AccountId,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::RoleRevoked(access_control_mock::RoleRevoked {
            role,
            account,
            sender,
        }) = decoded_event
        {
            assert_eq!(
                role, expected_role,
                "encountered invalid RoleRevoked.role"
            );
            assert_eq!(
                account, expected_account,
                "encountered invalid RoleRevoked.account"
            );
            assert_eq!(
                sender, expected_sender,
                "encountered invalid RoleRevoked.sender"
            );
        } else {
            panic!("encountered unexpected event kind: expected a Transfer event")
        }
        let expected_topics = vec![
            encoded_into_hash(&PrefixedValue {
                value: b"AccessControl::RoleRevoked",
                prefix: b"",
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleRevoked::role",
                value: &expected_role,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleRevoked::account",
                value: &expected_account,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"AccessControl::RoleRevoked::sender",
                value: &expected_sender,
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
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let ac =
            AccessControl::new(true, default_account, accounts.bob, accounts.charlie);

        assert_eq!(ac.get(), true, "flip state should ok");
        assert_eq!(
            ac.has_role(ROLE_ID_FLIPER, default_account),
            true,
            "default should have init role"
        );
        assert_eq!(
            ac.has_role(ROLE_ID_SETTER, accounts.bob),
            true,
            "default should have init role"
        );
        assert_eq!(
            ac.has_role(ROLE_ID_ADMIN, accounts.charlie),
            true,
            "default should have init role"
        );

        let emitted_events = assert_emitted_event_len(4);
        assert_role_admin_changed_event(
            &emitted_events[3],
            ROLE_ID_FLIPER,
            None,
            ROLE_ID_ADMIN,
        );
    }
}
