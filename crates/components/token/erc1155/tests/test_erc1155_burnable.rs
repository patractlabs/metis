#![cfg_attr(not(feature = "std"), no_std)]

mod mocks {
    pub mod burnable;
}

mod erc1155_burnable_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;
    // use ink::ContractEnv;
    use ink_lang as ink;
    use ink_prelude::string::String;
    use mocks::burnable::contract::{
        Erc1155Burnable,
        TokenId,
    };

    // type AccountId = <<Erc1155 as ContractEnv>::Env as ink_env::Environment>::AccountId;

    /// The default constructor does its job.
    #[ink::test]
    fn new_works() {
        // Constructor works.
        let erc1155 = Erc1155Burnable::new(String::from("MockErc1155Token"));
        let token_a = TokenId::new([0x0a; 32]);

        // for metadatas
        assert_eq!(
            String::from("MockErc1155Token"),
            erc1155.url(token_a),
            "name should be default"
        );
    }
}