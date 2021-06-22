#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod access_control_enumerable_mock;
}

mod access_control_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use ink::ContractEnv;
    use mock::{
        AccessControl,
        RoleId,
        ROLE_ID_ADMIN,
        ROLE_ID_FLIPER,
        ROLE_ID_SETTER,
    };

    const ROLE_ID_EMPTY: RoleId = RoleId::new([0x00; 32]);

    use ink_lang as ink;
    use mocks::access_control_enumerable_mock::mock;

    type AccountId =
        <<AccessControl as ContractEnv>::Env as ink_env::Environment>::AccountId;

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

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_ADMIN),
            1,
            "default ROLE_ID_ADMIN role account should 1"
        );
        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            1,
            "default ROLE_ID_FLIPER role account should 1"
        );
        assert_eq!(
            ac.get_role_member_count(ROLE_ID_SETTER),
            1,
            "default ROLE_ID_SETTER role account should 1"
        );
        assert_eq!(
            ac.get_role_member(ROLE_ID_ADMIN, 0),
            accounts.charlie,
            "default ROLE_ID_ADMIN member should be ok"
        );
        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            default_account,
            "default ROLE_ID_FLIPER member should be ok"
        );
        assert_eq!(
            ac.get_role_member(ROLE_ID_SETTER, 0),
            accounts.bob,
            "default ROLE_ID_SETTER member should be ok"
        );
    }

    #[ink::test]
    fn get_role_member_should_ok() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        let django = accounts.django;
        let eve = accounts.eve;
        let bob = accounts.bob;
        let charlie = accounts.charlie;

        // Constructor works.
        let mut ac = AccessControl::new(true, default_account, bob, charlie);

        ac._setup_role(ROLE_ID_FLIPER, django);
        ac._setup_role(ROLE_ID_FLIPER, eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            3,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            default_account,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 2),
            eve,
            "ROLE_ID_FLIPER member 2 should be ok"
        );
    }

    #[ink::test]
    #[should_panic]
    fn get_role_member_idx_out_eq_should_panic() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let mut ac =
            AccessControl::new(true, default_account, accounts.bob, accounts.charlie);

        ac._setup_role(ROLE_ID_FLIPER, accounts.django);
        ac._setup_role(ROLE_ID_FLIPER, accounts.eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            3,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 3),
            default_account,
            "ROLE_ID_FLIPER member 0 should be ok"
        );
    }

    #[ink::test]
    #[should_panic]
    fn get_role_member_idx_out_should_panic() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let ac =
            AccessControl::new(true, default_account, accounts.bob, accounts.charlie);

        assert_eq!(
            ac.get_role_member(ROLE_ID_SETTER, 4),
            default_account,
            "ROLE_ID_FLIPER member 0 should be ok"
        );
    }

    #[ink::test]
    #[should_panic]
    fn get_role_member_role_no_exist_should_panic() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let ac =
            AccessControl::new(true, default_account, accounts.bob, accounts.charlie);

        assert_eq!(
            ac.get_role_member(ROLE_ID_EMPTY, 0),
            default_account,
            "ROLE_ID_FLIPER member 0 should be ok"
        );
    }

    #[ink::test]
    fn grant_role_members_should_ok() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let mut ac =
            AccessControl::new(true, accounts.bob, accounts.charlie, default_account);

        ac.grant_role(ROLE_ID_FLIPER, accounts.django);
        ac.grant_role(ROLE_ID_FLIPER, accounts.eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            3,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.bob,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 2),
            accounts.eve,
            "ROLE_ID_FLIPER member 2 should be ok"
        );
    }

    #[ink::test]
    fn revoke_role_members_last_should_ok() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let mut ac =
            AccessControl::new(true, accounts.bob, accounts.charlie, default_account);

        ac.grant_role(ROLE_ID_FLIPER, accounts.django);
        ac.grant_role(ROLE_ID_FLIPER, accounts.eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            3,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.bob,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 2),
            accounts.eve,
            "ROLE_ID_FLIPER member 2 should be ok"
        );

        // revoke_role the role last
        ac.revoke_role(ROLE_ID_FLIPER, accounts.eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            2,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.bob,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );
    }

    #[ink::test]
    fn revoke_role_members_mid_should_ok() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let mut ac =
            AccessControl::new(true, accounts.bob, accounts.charlie, default_account);

        ac.grant_role(ROLE_ID_FLIPER, accounts.django);
        ac.grant_role(ROLE_ID_FLIPER, accounts.eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            3,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.bob,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 2),
            accounts.eve,
            "ROLE_ID_FLIPER member 2 should be ok"
        );

        // revoke_role the role last
        ac.revoke_role(ROLE_ID_FLIPER, accounts.django);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            2,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.bob,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.eve,
            "ROLE_ID_FLIPER member 1 should be ok"
        );
    }

    #[ink::test]
    fn revoke_role_members_first_should_ok() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let mut ac =
            AccessControl::new(true, accounts.bob, accounts.charlie, default_account);

        ac.grant_role(ROLE_ID_FLIPER, accounts.django);
        ac.grant_role(ROLE_ID_FLIPER, accounts.eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            3,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.bob,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 2),
            accounts.eve,
            "ROLE_ID_FLIPER member 2 should be ok"
        );

        // revoke_role the role last
        ac.revoke_role(ROLE_ID_FLIPER, accounts.bob);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            2,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.eve,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );
    }

    #[ink::test]
    fn revoke_role_members_all_should_ok() {
        let default_account = AccountId::from([0x01; 32]);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        // Constructor works.
        let mut ac =
            AccessControl::new(true, accounts.bob, accounts.charlie, default_account);

        ac.grant_role(ROLE_ID_FLIPER, accounts.django);
        ac.grant_role(ROLE_ID_FLIPER, accounts.eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            3,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.bob,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 2),
            accounts.eve,
            "ROLE_ID_FLIPER member 2 should be ok"
        );

        // revoke_role the role last
        ac.revoke_role(ROLE_ID_FLIPER, accounts.bob);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            2,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.eve,
            "ROLE_ID_FLIPER member 0 should be ok"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 1),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );

        // revoke_role the role 2
        ac.revoke_role(ROLE_ID_FLIPER, accounts.eve);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            1,
            "ROLE_ID_FLIPER role account should 3"
        );

        assert_eq!(
            ac.get_role_member(ROLE_ID_FLIPER, 0),
            accounts.django,
            "ROLE_ID_FLIPER member 1 should be ok"
        );

        // revoke_role the role 3
        ac.revoke_role(ROLE_ID_FLIPER, accounts.django);

        assert_eq!(
            ac.get_role_member_count(ROLE_ID_FLIPER),
            0,
            "ROLE_ID_FLIPER role account should 3"
        );
    }
}
