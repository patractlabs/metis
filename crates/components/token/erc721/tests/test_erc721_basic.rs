#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod basic;
}

mod erc721_basic_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    use ink::ContractEnv;
    use ink_lang as ink;
    use ink_prelude::string::String;
    use mocks::basic::contract::{
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
    fn transfer_work() {
        let default_account = AccountId::from([0x01; 32]);
        let other_account = AccountId::from([0x02; 32]);
        let second_account = AccountId::from([0x03; 32]);

        // Constructor works.
        let mut erc721 =
            Erc721::new(String::from("MockErc721Token"), String::from("MET"));

        // Check Current Balance

        // Mint, current mint is a mock
        let token_a = TokenId::new([0x0a; 32]);
        let token_b = TokenId::new([0x0b; 32]);
        let token_c = TokenId::new([0x0c; 32]);

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

        assert_eq!(
            erc721.mint(default_account, &token_b),
            Ok(()),
            "mint should be ok"
        );

        assert_eq!(
            2,
            erc721.balance_of(default_account),
            "curr account balance_of should be add 1"
        );

        assert_eq!(
            default_account,
            erc721.owner_of(&token_b),
            "token B owner should be default account"
        );

        assert_eq!(
            erc721.mint(other_account, &token_c),
            Ok(()),
            "mint should be ok"
        );

        assert_eq!(
            1,
            erc721.balance_of(other_account),
            "other account balance_of should be add 1"
        );

        assert_eq!(
            other_account,
            erc721.owner_of(&token_c),
            "token C owner should be other account"
        );

        assert_eq!(
            0,
            erc721.balance_of(second_account),
            "second account balance_of should be 0"
        );

        // transfer from 0x1 to 0x2

        assert_eq!(
            erc721.transfer_from(default_account, other_account, token_a),
            Ok(()),
            "transfer A should ok"
        );

        assert_eq!(
            1,
            erc721.balance_of(default_account),
            "other account balance_of should be add 1"
        );

        assert_eq!(
            2,
            erc721.balance_of(other_account),
            "other account balance_of should be add 1"
        );

        assert_eq!(
            other_account,
            erc721.owner_of(&token_a),
            "token C owner should be other account"
        );

        assert_eq!(
            erc721.transfer_from(default_account, second_account, token_b),
            Ok(()),
            "transfer B should ok"
        );

        assert_eq!(
            0,
            erc721.balance_of(default_account),
            "other account balance_of should be add 1"
        );

        assert_eq!(
            1,
            erc721.balance_of(second_account),
            "other account balance_of should be add 1"
        );

        assert_eq!(
            second_account,
            erc721.owner_of(&token_b),
            "token C owner should be other account"
        );
    }
}
