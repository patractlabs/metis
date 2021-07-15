#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod contract {
    pub use erc1155::{
        Error,
        Result,
        TokenId,
    };
    use metis_erc1155 as erc1155;
    use metis_lang::{
        import,
        metis,
    };

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[import(erc1155)]
    pub struct Erc1155 {
        erc1155: erc1155::Data<Erc1155>,
    }

    // TODO: gen by marco with Erc1155 component
    impl erc1155::Impl<Erc1155> for Erc1155 {}

    /// Emitted when `value` tokens of token type `id` are transferred from `from` to `to` by `operator`.
    #[ink(event)]
    #[metis(erc1155)]
    pub struct TransferSingle {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub id: TokenId,
        pub value: Balance,
    }

    /// @dev Equivalent to multiple {TransferSingle} events, where `operator`, `from` and `to` are the same for all
    /// transfers.
    #[ink(event)]
    #[metis(erc1155)]
    pub struct TransferBatch {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub id: Vec<TokenId>,
        pub value: Vec<Balance>,
    }

    /// Emitted when `owner` enables or disables (`approved`) `operator` to manage all of its assets.
    #[ink(event)]
    #[metis(erc1155)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub operator: AccountId,
        pub approved: bool,
    }

    /// @dev Emitted when the URI for token type `id` changes to `value`, if it is a non-programmatic URI.
    ///
    /// If an {URI} event was emitted for `id`, the standard
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata-extensions[guarantees] that `value` will equal the value
    /// returned by {IERC1155MetadataURI-uri}.
    #[ink(event)]
    #[metis(erc1155)]
    pub struct Url {
        pub value: String,
        #[ink(topic)]
        pub id: TokenId,
    }

    // for test message
    impl Erc1155 {}

    // impl
    impl Erc1155 {
        #[ink(constructor)]
        pub fn new(url: String) -> Self {
            let mut instance = Self {
                erc1155: erc1155::Data::new(),
            };

            erc1155::Impl::init(&mut instance, url);
            instance
        }

        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            String::from("")
        }
    }
}
