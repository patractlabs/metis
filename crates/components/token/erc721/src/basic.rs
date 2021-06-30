pub use super::module::Data;
use ink_prelude::string::String;
pub use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

/// The ERC-20 error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if account is zero
    AccountIsZero,
}

use crate::types::TokenId;

/// The ERC-20 result type.
pub type Result<T> = core::result::Result<T, Error>;

/// The `EventEmit` impl the event emit api for erc20 component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emitted when `token_id` token is transferred from `from` to `to`.
    fn emit_event_transfer(
        &mut self,
        from: Option<E::AccountId>,
        to: Option<E::AccountId>,
        token_id: TokenId,
    );

    /// Emitted when `owner` enables `approved` to manage the `token_id` token.
    fn emit_event_approval(
        &mut self,
        owner: E::AccountId,
        spender: Option<E::AccountId>,
        token_id: TokenId,
    );

    /// Emitted when `owner` enables or disables (`approved`) `operator` to manage all of its assets.
    fn emit_event_approval_for_all(
        &mut self,
        owner: E::AccountId,
        operator: E::AccountId,
        approved: bool,
    );
}

/// The `Impl` define erc20 component impl funcs, with `_before_token_transfer` as hook
/// To use erc20 Impl need impl the trait as:
///
/// impl erc20::hookable::Impl<Contract> for Contract {
///     fn _before_token_transfer(
///         &mut self,
///         _from: &AccountId,
///         _to: &AccountId,
///         _amount: Balance,
///     ) -> Result<()> {
///         Ok(())
///     }
/// }
pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> {
    /// Initialize the erc20 component
    fn init(&mut self, name: String, symbol: String) {
        self.get_mut().set_symbols(name, symbol);
    }

    /// Hook that is called before any token transfer. This includes minting
    /// and burning.
    ///
    /// Calling conditions:
    ///
    /// - When `from` and `to` are both non-zero, ``from``'s `token_id` will be
    /// transferred to `to`.
    /// - When `from` is zero, `token_id` will be minted for `to`.
    /// - When `to` is zero, ``from``'s `token_id` will be burned.
    /// - `from` and `to` are never both zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _before_token_transfer(
        &mut self,
        from: Option<E::AccountId>,
        to: Option<E::AccountId>,
        token_id: &TokenId,
    ) -> Result<()>;

    /// Returns the name of the token.
    fn name(&self) -> String {
        self.get().name().clone()
    }

    /// Returns the symbol of the token, usually a shorter version of the name.
    fn symbol(&self) -> String {
        self.get().symbol().clone()
    }

    /// Returns the Uniform Resource Identifier (URI) for `token_id` token.
    fn token_url(&self, token_id: &TokenId) -> String {
        assert!(
            self._exists(token_id),
            "ERC721Metadata: URI query for nonexistent token"
        );

        let mut base_url = self._base_url();

        match base_url.len() {
            0 => String::from(""),
            _ => {
                base_url.push_str(token_id.to_string().as_str());
                base_url
            }
        }
    }

    /// @dev Returns the number of tokens in ``owner``'s account.
    fn balance_of(&self, account: &E::AccountId) -> E::Balance {
        self.get().balance_of(account)
    }

    /// @dev Returns the owner of the `token_id` token.
    ///
    /// Requirements:
    ///
    /// - `token_id` must exist.
    fn owner_of(&self, token_id: &TokenId) -> E::AccountId {
        match self.get().owners.get(token_id) {
            Some(owner) => owner.clone(),
            None => panic!("ERC721: owner query for nonexistent token"),
        }
    }

    /// @dev Returns the account approved for `token_id` token.
    ///
    /// Requirements:
    ///
    /// - `token_id` must exist.
    fn get_approved(&self, token_id: &TokenId) -> Option<&E::AccountId> {
        assert!(
            self._exists(token_id),
            "ERC721: approved query for nonexistent token"
        );

        self.get().token_approvals.get(token_id)
    }

    /// @dev Returns if the `operator` is allowed to manage all of the assets of `owner`.
    ///
    /// See {setApprovalForAll}
    fn is_approved_for_all(&self, owner: &E::AccountId, operator: &E::AccountId) -> bool {
        self.get().is_approved_for_all(owner.clone(), operator.clone())
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
    fn approve(&mut self, to: E::AccountId, token_id: &TokenId) {
        let owner = self.owner_of(token_id);
        let caller = Self::caller();

        assert!(to != owner, "ERC721: approval to current owner");

        assert!(
            caller == owner || self.is_approved_for_all(&owner, &caller),
            "ERC721: approve caller is not owner nor approved for all"
        );

        self._approve(Some(to), token_id);
    }

    /// @dev Approve or remove `operator` as an operator for the caller.
    /// Operators can call {transferFrom} or {safeTransferFrom} for any token owned by the caller.
    ///
    /// Requirements:
    ///
    /// - The `operator` cannot be the caller.
    ///
    /// Emits an {ApprovalForAll} event.
    fn set_approval_for_all(&mut self, operator: E::AccountId, approved: bool) {
        let caller = Self::caller();
        assert!(operator != caller, "ERC721: approve to caller");

        self.get_mut()
            .set_approval_for_all(caller.clone(), operator.clone(), approved);
        self.emit_event_approval_for_all(caller, operator, approved);
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
    fn transfer_from(
        &mut self,
        from: &E::AccountId,
        to: &E::AccountId,
        token_id: &TokenId,
    ) -> Result<()> {
        assert!(
            self._is_approved_or_owner(&Self::caller(), token_id),
            "ERC721: transfer caller is not owner nor approved"
        );

        self._transfer(from, to, token_id)
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
    fn safe_transfer_from(
        &mut self,
        from: &E::AccountId,
        to: &E::AccountId,
        token_id: &TokenId,
    ) -> Result<()> {
        self.safe_transfer_from_with_data(from, to, token_id, Vec::default())
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
    fn safe_transfer_from_with_data(
        &mut self,
        from: &E::AccountId,
        to: &E::AccountId,
        token_id: &TokenId,
        data: Vec<u8>,
    ) -> Result<()> {
        assert!(
            self._is_approved_or_owner(&Self::caller(), token_id),
            "ERC721: transfer caller is not owner nor approved"
        );

        self._safe_transfer(from, to, token_id, data)
    }

    /// @dev Base URI for computing {tokenURI}. If set, the resulting URI for each
    /// token will be the concatenation of the `baseURI` and the `token_id`. Empty
    /// by default, can be overriden in child contracts.
    fn _base_url(&self) -> String;

    /// @dev Returns whether `token_id` exists.
    ///
    /// Tokens can be managed by their owner or approved accounts via {approve} or {setApprovalForAll}.
    ///
    /// Tokens start existing when they are minted (`_mint`),
    /// and stop existing when they are burned (`_burn`).
    fn _exists(&self, token_id: &TokenId) -> bool {
        match self.get().owners.get(token_id) {
            Some(_) => true,
            None => false,
        }
    }

    /// @dev Safely transfers `token_id` token from `from` to `to`, checking first that contract recipients
    /// are aware of the ERC721 protocol to prevent tokens from being forever locked.
    ///
    /// `_data` is additional data, it has no specified format and it is sent in call to `to`.
    ///
    /// This internal function is equivalent to {safeTransferFrom}, and can be used to e.g.
    /// implement alternative mechanisms to perform token transfer, such as signature-based.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `token_id` token must exist and be owned by `from`.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    ///
    /// Emits a {Transfer} event.
    fn _safe_transfer(
        &mut self,
        from: &E::AccountId,
        to: &E::AccountId,
        token_id: &TokenId,
        data: Vec<u8>,
    ) -> Result<()> {
        self._transfer(from, to, token_id)?;

        assert!(
            self._check_on_erc721_received(from, to, token_id, data),
            "ERC721: transfer to non ERC721Receiver implementer"
        );

        Ok(())
    }

    /// @dev Returns whether `spender` is allowed to manage `token_id`.
    ///
    /// Requirements:
    ///
    /// - `token_id` must exist.
    fn _is_approved_or_owner(&self, spender: &E::AccountId, token_id: &TokenId) -> bool {
        assert!(
            self._exists(token_id),
            "ERC721: operator query for nonexistent token"
        );

        let owner = &self.owner_of(token_id);

        spender == owner
            || self.get_approved(token_id) == Some(spender)
            || self.is_approved_for_all(owner, spender)
    }

    /// @dev Safely mints `token_id` and transfers it to `to`.
    ///
    /// Requirements:
    ///
    /// - `token_id` must not exist.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    ///
    /// Emits a {Transfer} event.
    fn _safe_mint(&mut self, to: &E::AccountId, token_id: &TokenId) -> Result<()> {
        self._safe_mint_with_data(to, token_id, Vec::default())
    }

    /// @dev Same as {xref-ERC721-_safeMint-address-uint256-}[`_safeMint`], with an additional `data` parameter which is
    /// forwarded in {IERC721Receiver-onERC721Received} to contract recipients.
    fn _safe_mint_with_data(
        &mut self,
        to: &E::AccountId,
        token_id: &TokenId,
        data: Vec<u8>,
    ) -> Result<()> {
        self._mint(to, token_id)?;

        assert!(
            self._check_on_erc721_received(&E::AccountId::default(), to, token_id, data),
            "ERC721: transfer to non ERC721Receiver implementer"
        );

        Ok(())
    }

    /// @dev Mints `token_id` and transfers it to `to`.
    ///
    /// WARNING: Usage of this method is discouraged, use {_safeMint} whenever possible
    ///
    /// Requirements:
    ///
    /// - `token_id` must not exist.
    /// - `to` cannot be the zero address.
    ///
    /// Emits a {Transfer} event.
    fn _mint(&mut self, to: &E::AccountId, token_id: &TokenId) -> Result<()> {
        assert!(
            *to != E::AccountId::default(),
            "ERC721: mint to the zero address"
        );
        assert!(!self._exists(token_id), "ERC721: token already minted");

        self._before_token_transfer(None, Some(to.clone()), token_id)?;

        self.get_mut().balance_inc(to);
        self.get_mut().owners.insert(token_id.clone(), to.clone());

        self.emit_event_transfer(None, Some(to.clone()), token_id.clone());

        Ok(())
    }

    /// @dev Destroys `token_id`.
    /// The approval is cleared when the token is burned.
    ///
    /// Requirements:
    ///
    /// - `token_id` must exist.
    ///
    /// Emits a {Transfer} event.
    fn _burn(&mut self, token_id: &TokenId) -> Result<()> {
        let owner = self.owner_of(token_id);

        self._before_token_transfer(Some(owner.clone()), None, token_id)?;

        // Clear approvals
        self._approve(None, token_id);

        self.get_mut().balance_dec(&owner);

        self.get_mut().owners.take(token_id);

        self.emit_event_transfer(Some(owner), None, *token_id);

        Ok(())
    }

    /// @dev Transfers `token_id` from `from` to `to`.
    ///  As opposed to {transferFrom}, this imposes no restrictions on msg.sender.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - `token_id` token must be owned by `from`.
    ///
    /// Emits a {Transfer} event.
    fn _transfer(
        &mut self,
        from: &E::AccountId,
        to: &E::AccountId,
        token_id: &TokenId,
    ) -> Result<()> {
        assert!(
            self.owner_of(token_id) == *from,
            "ERC721: transfer of token that is not own"
        );
        assert!(
            *to != E::AccountId::default(),
            "ERC721: transfer to the zero address"
        );

        self._before_token_transfer(Some(from.clone()), Some(to.clone()), token_id)?;

        // Clear approvals from the previous owner
        self._approve(None, token_id);

        self.get_mut().balance_dec(from);
        self.get_mut().balance_inc(to);
        self.get_mut().owners.insert(token_id.clone(), to.clone());

        self.emit_event_transfer(Some(from.clone()), Some(to.clone()), token_id.clone());

        Ok(())
    }

    /// @dev Approve `to` to operate on `token_id`
    ///
    /// Emits a {Approval} event.
    fn _approve(&mut self, to: Option<E::AccountId>, token_id: &TokenId) {
        match to.clone() {
            Some(to_account) => {
                self.get_mut()
                    .token_approvals
                    .insert(token_id.clone(), to_account.clone())
            }
            None => self.get_mut().token_approvals.take(token_id),
        };

        self.emit_event_approval(self.owner_of(token_id), to, *token_id);
    }

    /// @dev Internal function to invoke {IERC721Receiver-onERC721Received} on a target address.
    /// The call is not executed if the target address is not a contract.
    ///
    /// @param from address representing the previous owner of the given token ID
    /// @param to target address that will receive the tokens
    /// @param token_id uint256 ID of the token to be transferred
    /// @param _data bytes optional data to send along with the call
    /// @return bool whether the call correctly returned the expected magic value
    fn _check_on_erc721_received(
        &mut self,
        _from: &E::AccountId,
        _to: &E::AccountId,
        _token_id: &TokenId,
        _data: Vec<u8>,
    ) -> bool {
        true
    }
}
