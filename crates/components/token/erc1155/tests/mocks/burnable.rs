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
    pub struct Erc1155Burnable {
        erc1155: erc1155::Data<Erc1155Burnable>,
    }

    // TODO: gen by marco with Erc1155Burnable component
    impl erc1155::Impl<Erc1155Burnable> for Erc1155Burnable {}
    impl erc1155::burnable::Impl<Erc1155Burnable> for Erc1155Burnable {}

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
    impl Erc1155Burnable {}

    // impl
    impl Erc1155Burnable {
        #[ink(constructor)]
        pub fn new(url: String) -> Self {
            let mut instance = Self {
                erc1155: erc1155::Data::new(),
            };

            erc1155::Impl::init(&mut instance, url);
            instance
        }

        /// @dev See {IERC1155MetadataURI-uri}.
        ///
        /// This implementation returns the same URI for *all* token types. It relies
        /// on the token type ID substitution mechanism
        /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
        ///
        /// Clients calling this function must replace the `\{id\}` substring with the
        /// actual token type ID.
        #[ink(message)]
        pub fn url(&self, id: TokenId) -> String {
            erc1155::Impl::url(self, id)
        }

        /// @dev See {IERC1155-balanceOf}.
        ///
        /// Requirements:
        ///
        /// - `account` cannot be the zero address.
        #[ink(message)]
        pub fn balance_of(&self, account: &AccountId, id: &TokenId) -> Balance {
            erc1155::Impl::balance_of(self, account, id)
        }

        /// @dev See {IERC1155-balanceOfBatch}.
        ///
        /// Requirements:
        ///
        /// - `accounts` and `ids` must have the same length.
        #[ink(message)]
        pub fn balance_of_batch(
            &self,
            accounts: Vec<AccountId>,
            ids: Vec<TokenId>,
        ) -> Vec<Balance> {
            erc1155::Impl::balance_of_batch(self, accounts, ids)
        }

        /// @dev See {IERC1155-setApprovalForAll}.
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) {
            erc1155::Impl::set_approval_for_all(self, operator, approved)
        }

        /// @dev See {IERC1155-isApprovedForAll}.
        #[ink(message)]
        pub fn is_approved_for_all(
            &self,
            account: &AccountId,
            operator: &AccountId,
        ) -> bool {
            erc1155::Impl::is_approved_for_all(self, account, operator)
        }

        /// @dev See {IERC1155-safeTransferFrom}.
        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: TokenId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<()> {
            erc1155::Impl::safe_transfer_from(self, from, to, id, amount, data)
        }

        /// @dev See {IERC1155-safeBatchTransferFrom}.
        #[ink(message)]
        pub fn safe_batch_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            ids: Vec<TokenId>,
            amounts: Vec<Balance>,
            data: Vec<u8>,
        ) -> Result<()> {
            erc1155::Impl::safe_batch_transfer_from(self, from, to, ids, amounts, data)
        }

        /// @dev Burns `id` by `value`
        ///
        /// Requirements:
        ///
        /// - The caller must own `id` or be an approved operator.
        fn burn(
            &mut self,
            account: AccountId,
            id: TokenId,
            value: Balance,
        ) -> Result<()> {
            erc1155::burnable::Impl::burn(self, account, id, value)
        }

        /// @dev Burns Batch `ids` by `values`
        ///
        /// Requirements:
        ///
        /// - The caller must own `id` or be an approved operator.
        fn burn_batch(
            &mut self,
            account: AccountId,
            ids: Vec<TokenId>,
            values: Vec<Balance>,
        ) -> Result<()> {
            erc1155::burnable::Impl::burn_batch(self, account, ids, values)
        }
    }
}
