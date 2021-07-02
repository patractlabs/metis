#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod pausable;
}

mod erc721_basic_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use ink::ContractEnv;
    use ink_lang as ink;
    use ink_prelude::string::String;
    use mocks::pausable::contract::{
        Erc721,
        TokenId,
    };

    type AccountId = <<Erc721 as ContractEnv>::Env as ink_env::Environment>::AccountId;

    /// The default constructor does its job.
    #[ink::test]
    fn new_works() {
        // Constructor works.
        let erc721 = Erc721::new(String::from("MockErc721Token"), String::from("MET"));

        // for metadatas
        assert_eq!(
            String::from("MockErc721Token"),
            erc721.name(),
            "name should be default"
        );

        assert_eq!(
            String::from("MET"),
            erc721.symbol(),
            "symbol should be default"
        );
    }

    #[ink::test]
    fn mint_work() {
        let default_account = AccountId::from([0x01; 32]);

        // Constructor works.
        let mut erc721 =
            Erc721::new(String::from("MockErc721Token"), String::from("MET"));

        // Check Current Balance
        assert_eq!(
            0,
            erc721.balance_of(default_account),
            "default account balance_of should be default"
        );

        // Mint, current mint is a mock
        let token_a = TokenId::new([0x0a; 32]);
        assert_eq!(
            erc721.mint(default_account, &token_a),
            Ok(()),
            "mint should be ok"
        );

        assert_eq!(
            1,
            erc721.balance_of(default_account),
            "curr account balance_of should be add 1"
        );

        assert_eq!(
            default_account,
            erc721.owner_of(&token_a),
            "token A owner should be default account"
        );
    }

    #[ink::test]
    #[should_panic(expected = "Pausable: ensure not paused")]
    fn pause_work() {
        let default_account = AccountId::from([0x01; 32]);
        let bob = AccountId::from([0x02; 32]);

        // Constructor works.
        let mut erc721 =
            Erc721::new(String::from("MockErc721Token"), String::from("MET"));

        // Mint, current mint is a mock
        let token_a = TokenId::new([0x0a; 32]);
        assert_eq!(
            erc721.mint(default_account, &token_a),
            Ok(()),
            "mint should be ok"
        );

        erc721.pause();
        assert!(erc721.paused(), "current should be paused");

        let _ = erc721.transfer_from(default_account, bob, token_a);
    }

    #[ink::test]
    fn unpause_work() {
        let default_account = AccountId::from([0x01; 32]);
        let bob = AccountId::from([0x02; 32]);

        // Constructor works.
        let mut erc721 =
            Erc721::new(String::from("MockErc721Token"), String::from("MET"));

        // Mint, current mint is a mock
        let token_a = TokenId::new([0x0a; 32]);
        assert_eq!(
            erc721.mint(default_account, &token_a),
            Ok(()),
            "mint should be ok"
        );

        erc721.pause();
        assert!(erc721.paused(), "current should be paused");

        erc721.unpause();
        assert!(!erc721.paused(), "current should be not paused");

        assert_eq!(
            erc721.transfer_from(default_account, bob, token_a),
            Ok(()),
            "tranfer should ok"
        );
    }
}
