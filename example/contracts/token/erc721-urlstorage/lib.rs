#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod contract {
    use erc721::urlstorage;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use metis_erc721 as erc721;
    pub use metis_erc721::{
        Error,
        Result,
        TokenId,
    };
    use metis_lang::{
        import,
        metis,
    };

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[import(erc721, urlstorage)]
    pub struct Erc721 {
        erc721: erc721::Data<Erc721>,
        urlstorage: urlstorage::Data,
    }

    // TODO: gen by marco with Erc721 component
    impl erc721::Impl<Erc721> for Erc721 {
        fn _before_token_transfer(
            &mut self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _token_id: &TokenId,
        ) -> Result<()> {
            Ok(())
        }

        fn _base_url(&self) -> String {
            String::from("https://test/")
        }
    }
    impl erc721::urlstorage::Impl<Erc721> for Erc721 {}

    /// Emitted when `token_id` token is transferred from `from` to `to`.
    #[ink(event)]
    #[metis(erc721)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub token_id: TokenId,
    }

    /// Emitted when `owner` enables `approved` to manage the `token_id` token.
    #[ink(event)]
    #[metis(erc721)]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: Option<AccountId>,
        pub token_id: TokenId,
    }

    /// Emitted when `owner` enables or disables (`approved`) `operator` to manage all of its assets.
    #[ink(event)]
    #[metis(erc721)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub operator: AccountId,
        pub approved: bool,
    }

    // for test message
    impl Erc721 {
        /// For test to mint
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, token_id: TokenId) -> Result<()> {
            erc721::Impl::_mint(self, &to, &token_id)
        }

        /// For test to burn
        #[ink(message)]
        pub fn burn(&mut self, token_id: TokenId) -> Result<()> {
            erc721::Impl::_burn(self, &token_id)
        }
    }

    // impl
    impl Erc721 {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let mut instance = Self {
                erc721: erc721::Data::new(),
                urlstorage: urlstorage::Data::default(),
            };

            erc721::Impl::init(&mut instance, name, symbol);
            instance
        }

        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            erc721::Impl::name(self)
        }

        /// Returns the symbol of the token, usually a shorter version of the name.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc721::Impl::symbol(self)
        }

        /// Returns the Uniform Resource Identifier (URI) for `token_id` token.
        #[ink(message)]
        pub fn token_url(&self, token_id: TokenId) -> String {
            erc721::Impl::token_url(self, &token_id)
        }

        /// @dev Returns the number of tokens in ``owner``'s account.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u64 {
            erc721::Impl::balance_of(self, &owner)
        }

        /// @dev Returns the owner of the `token_id` token.
        ///
        /// Requirements:
        ///
        /// - `token_id` must exist.
        #[ink(message)]
        pub fn owner_of(&self, token_id: TokenId) -> AccountId {
            erc721::Impl::owner_of(self, &token_id)
        }

        /// @dev Returns the account approved for `token_id` token.
        ///
        /// Requirements:
        ///
        /// - `token_id` must exist.
        #[ink(message)]
        pub fn get_approved(&self, token_id: TokenId) -> Option<AccountId> {
            erc721::Impl::get_approved(self, &token_id)
        }

        /// @dev Returns if the `operator` is allowed to manage all of the assets of `owner`.
        ///
        /// See {setApprovalForAll}
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            erc721::Impl::is_approved_for_all(self, &owner, &operator)
        }

        /// @dev Gives permission to `to` to transfer `token_id` token to another account.
        /// The approval is cleared when the token is transferred.
        ///
        /// Only a single account can be approved at a time, so approving the zero address clears previous approvals.
        ///
        /// Requirements:
        ///
        /// - The caller must own the token or be an approved operator.
        /// - `token_id` must exist.
        ///
        /// Emits an {Approval} event.
        #[ink(message)]
        pub fn approve(&mut self, to: Option<AccountId>, token_id: TokenId) {
            erc721::Impl::approve(self, to, &token_id)
        }

        /// @dev Approve or remove `operator` as an operator for the caller.
        /// Operators can call {transferFrom} or {safeTransferFrom} for any token owned by the caller.
        ///
        /// Requirements:
        ///
        /// - The `operator` cannot be the caller.
        ///
        /// Emits an {ApprovalForAll} event.
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) {
            erc721::Impl::set_approval_for_all(self, operator, approved)
        }

        /// @dev Transfers `token_id` token from `from` to `to`.
        ///
        /// WARNING: Usage of this method is discouraged, use {safeTransferFrom} whenever possible.
        ///
        /// Requirements:
        ///
        /// - `from` cannot be the zero address.
        /// - `to` cannot be the zero address.
        /// - `token_id` token must be owned by `from`.
        /// - If the caller is not `from`, it must be approved to move this token by either {approve} or {setApprovalForAll}.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
        ) -> Result<()> {
            erc721::Impl::transfer_from(self, from, to, token_id)
        }

        /// @dev Safely transfers `token_id` token from `from` to `to`, checking first that contract recipients
        /// are aware of the ERC721 protocol to prevent tokens from being forever locked.
        ///
        /// Requirements:
        ///
        /// - `from` cannot be the zero address.
        /// - `to` cannot be the zero address.
        /// - `token_id` token must exist and be owned by `from`.
        /// - If the caller is not `from`, it must be have been allowed to move this token by either {approve} or {setApprovalForAll}.
        /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
        ) -> Result<()> {
            erc721::Impl::safe_transfer_from(self, from, to, token_id)
        }

        /// @dev Safely transfers `token_id` token from `from` to `to`.
        ///
        /// Requirements:
        ///
        /// - `from` cannot be the zero address.
        /// - `to` cannot be the zero address.
        /// - `token_id` token must exist and be owned by `from`.
        /// - If the caller is not `from`, it must be approved to move this token by either {approve} or {setApprovalForAll}.
        /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn safe_transfer_from_with_data(
            &mut self,
            from: AccountId,
            to: AccountId,
            token_id: TokenId,
            data: Vec<u8>,
        ) -> Result<()> {
            erc721::Impl::safe_transfer_from_with_data(self, from, to, token_id, data)
        }
    }
}
