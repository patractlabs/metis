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

    /// Equivalent to multiple `TransferSingle` events, where `operator`, `from` and `to` are the same for all
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

    /// Emitted when the URI for token type `id` changes to `value`, if it is a non-programmatic URI.
    ///
    /// If an `URI` event was emitted for `id`, the standard
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata-extensions[guarantees] that `value` will equal the value
    /// returned by `uri`.
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

        /// Returns the URI for token type `id`.
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

        /// Returns the amount of tokens of token type `id` owned by `account`.
        ///
        /// Requirements:
        ///
        /// - `account` cannot be the zero address.
        #[ink(message)]
        pub fn balance_of(&self, account: &AccountId, id: &TokenId) -> Balance {
            erc1155::Impl::balance_of(self, account, id)
        }

        /// Batched version of balance_of
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

        /// Grants or revokes permission to `operator` to transfer the caller's tokens, according to `approved`,
        ///
        /// Emits an `ApprovalForAll` event.
        ///
        /// Requirements:
        ///
        /// - `operator` cannot be the caller.
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) {
            erc1155::Impl::set_approval_for_all(self, operator, approved)
        }

        /// Returns true if `operator` is approved to transfer ``account``'s tokens.
        ///
        /// See `set_approval_for_all`.
        #[ink(message)]
        pub fn is_approved_for_all(
            &self,
            account: &AccountId,
            operator: &AccountId,
        ) -> bool {
            erc1155::Impl::is_approved_for_all(self, account, operator)
        }

        /// Transfers `amount` tokens of token type `id` from `from` to `to`.
        ///
        /// Emits a `TransferSingle` event.
        ///
        /// Requirements:
        ///
        /// - `to` cannot be the zero address.
        /// - If the caller is not `from`, it must be have been approved to spend ``from``'s tokens via `set_approval_for_all`.
        /// - `from` must have a balance of tokens of type `id` of at least `amount`.
        /// - If `to` refers to a smart contract, it must implement `on_erc1155_received` and return the
        ///   acceptance magic value.
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

        /// Batched version of the `safe_transfer_from`
        ///
        /// Emits a `TransferBatch` event.
        ///
        /// Requirements:
        ///
        /// - `ids` and `amounts` must have the same length.
        /// - If `to` refers to a smart contract, it must implement `on_erc1155_batch_received` and return the
        ///   acceptance magic value.
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
    }
}
