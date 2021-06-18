#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod access_control_mock;
}

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
    use ink_env::{
        hash::{
            Blake2x256,
            CryptoHash,
            HashOutput,
        },
        Clear,
    };
    use ink_lang as ink;
    use mocks::access_control_mock::access_control_mock;

    type AccountId =
        <<AccessControl as ContractEnv>::Env as ink_env::Environment>::AccountId;
    // type Balance = <<AccessControl as ContractEnv>::Env as ink_env::Environment>::Balance;
    type Hash = <<AccessControl as ContractEnv>::Env as ink_env::Environment>::Hash;
    type Event = <AccessControl as ink::BaseEvent>::Type;

    /// assert_emitted_event_len check event emitted current len is expected
    pub fn assert_emitted_event_len(expected: usize) -> Vec<ink_env::test::EmittedEvent> {
        // Transfer event triggered during initial construction.
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(expected, emitted_events.len());
        emitted_events
    }

    struct PrefixedValue<'a, 'b, T> {
        pub prefix: &'a [u8],
        pub value: &'b T,
    }
    impl<X> scale::Encode for PrefixedValue<'_, '_, X>
    where
        X: scale::Encode,
    {
        #[inline]
        fn size_hint(&self) -> usize {
            self.prefix.size_hint() + self.value.size_hint()
        }
        #[inline]
        fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
            self.prefix.encode_to(dest);
            self.value.encode_to(dest);
        }
    }

    fn encoded_into_hash<T>(entity: &T) -> Hash
    where
        T: scale::Encode,
    {
        let mut result = Hash::clear();
        let len_result = result.as_ref().len();
        let encoded = entity.encode();
        let len_encoded = encoded.len();
        if len_encoded <= len_result {
            result.as_mut()[..len_encoded].copy_from_slice(&encoded);
            return result
        }
        let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
        <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
        let copy_len = core::cmp::min(hash_output.len(), len_result);
        result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
        result
    }

    fn assert_role_admin_changed_event(
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
