#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod basic;
}

mod erc1155_basic_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    // use ink::ContractEnv;
    use ink_lang as ink;
    use ink_prelude::string::String;
    use mocks::basic::contract::{
        Erc1155,
        TokenId,
    };

    // type AccountId = <<Erc1155 as ContractEnv>::Env as ink_env::Environment>::AccountId;

    /// The default constructor does its job.
    #[ink::test]
    fn new_works() {
        // Constructor works.
        let erc1155 = Erc1155::new(String::from("MockErc1155Token"));
        let token_a = TokenId::new([0x0a; 32]);

        // for metadatas
        assert_eq!(
            String::from("MockErc1155Token"),
            erc1155.url(token_a),
            "name should be default"
        );
    }
}