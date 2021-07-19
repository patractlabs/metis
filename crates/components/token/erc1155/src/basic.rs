pub use super::module::Data;
use ink_lang::ForwardCallMut;
use ink_prelude::{
    string::String,
    vec::Vec,
    vec,
};
pub use metis_lang::{
    Env,
    EnvAccess,
    FromAccountId,
    Storage,
};

use metis_erc1155_receiver::ERC1155ReceiverStub as Receiver;

/// The ERC-1155 error types.
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

pub use crate::TokenId;

/// The ERC-1155 result type.
pub type Result<T> = core::result::Result<T, Error>;

/// The `EventEmit` impl the event emit api for erc1155 component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emitted when `value` tokens of token type `id` are transferred from `from` to `to` by `operator`.
    fn emit_event_transfer_single(
        &mut self,
        operator: E::AccountId,
        from: Option<E::AccountId>,
        to: Option<E::AccountId>,
        id: TokenId,
        value: E::Balance,
    );

    /// @dev Equivalent to multiple {TransferSingle} events, where `operator`, `from` and `to` are the same for all
    /// transfers.
    fn emit_event_transfer_batch(
        &mut self,
        operator: E::AccountId,
        from: Option<E::AccountId>,
        to: Option<E::AccountId>,
        id: Vec<TokenId>,
        value: Vec<E::Balance>,
    );

    /// Emitted when `owner` enables or disables (`approved`) `operator` to manage all of its assets.
    fn emit_event_approval_for_all(
        &mut self,
        owner: E::AccountId,
        operator: E::AccountId,
        approved: bool,
    );

    /// @dev Emitted when the URI for token type `id` changes to `value`, if it is a non-programmatic URI.
    ///
    /// If an {URI} event was emitted for `id`, the standard
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata-extensions[guarantees] that `value` will equal the value
    /// returned by {IERC1155MetadataURI-uri}.
    fn emit_event_url(&mut self, value: String, id: TokenId);
}

/// The `Impl` define erc1155 component impl funcs, with `_before_token_transfer` as hook
/// To use erc1155 Impl need impl the trait as:
///
/// impl erc1155::Impl<Contract> for Contract {
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
    /// Initialize the erc1155 component
    fn init(&mut self, url: String) {
        self.get_mut().set_url(url)
    }

    /// @dev See {IERC1155MetadataURI-uri}.
    ///
    /// This implementation returns the same URI for *all* token types. It relies
    /// on the token type ID substitution mechanism
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
    ///
    /// Clients calling this function must replace the `\{id\}` substring with the
    /// actual token type ID.
    fn url(&self, _id: TokenId) -> String {
        self.get().get_url()
    }

    /// @dev See {IERC1155-balanceOf}.
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    fn balance_of(&self, account: &E::AccountId, id: &TokenId) -> E::Balance {
        assert!(
            *account != E::AccountId::default(),
            "ERC1155: balance query for the zero address"
        );

        self.get().balance_of(id, account)
    }

    /// @dev See {IERC1155-balanceOfBatch}.
    ///
    /// Requirements:
    ///
    /// - `accounts` and `ids` must have the same length.
    fn balance_of_batch(
        &self,
        accounts: Vec<E::AccountId>,
        ids: Vec<TokenId>,
    ) -> Vec<E::Balance> {
        assert!(
            accounts.len() == ids.len(),
            "ERC1155: accounts and ids length mismatch"
        );

        (0..accounts.len())
            .collect::<Vec<_>>()
            .iter()
            .map(|idx| {
                self.balance_of(accounts.get(*idx).unwrap(), ids.get(*idx).unwrap())
            })
            .collect()
    }

    /// @dev See {IERC1155-setApprovalForAll}.
    fn set_approval_for_all(&mut self, operator: E::AccountId, approved: bool) {
        let caller = Self::caller();

        assert!(
            caller != operator,
            "ERC1155: setting approval status for self"
        );

        self.get_mut()
            .set_approval_for_all(caller.clone(), operator.clone(), approved);
        self.emit_event_approval_for_all(caller, operator, approved);
    }

    /// @dev See {IERC1155-isApprovedForAll}.
    fn is_approved_for_all(
        &self,
        account: &E::AccountId,
        operator: &E::AccountId,
    ) -> bool {
        self.get().is_approved_for_all(account, operator)
    }

    /// @dev See {IERC1155-safeTransferFrom}.
    fn safe_transfer_from(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        id: TokenId,
        amount: E::Balance,
        data: Vec<u8>,
    ) -> Result<()> {
        let caller = Self::caller();
        assert!(
            from == caller || self.is_approved_for_all(&from, &caller),
            "ERC1155: caller is not owner nor approved"
        );

        self._safe_transfer_from(from, to, id, amount, data)
    }

    /// @dev See {IERC1155-safeBatchTransferFrom}.
    fn safe_batch_transfer_from(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        id: Vec<TokenId>,
        amount: Vec<E::Balance>,
        data: Vec<u8>,
    ) -> Result<()> {
        let caller = Self::caller();
        assert!(
            from == caller || self.is_approved_for_all(&from, &caller),
            "ERC1155: transfer caller is not owner nor approved"
        );

        self._safe_batch_transfer_from(from, to, id, amount, data)
    }

    /// @dev Transfers `amount` tokens of token type `id` from `from` to `to`.
    ///
    /// Emits a {TransferSingle} event.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - `from` must have a balance of tokens of type `id` of at least `amount`.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
    /// acceptance magic value.
    fn _safe_transfer_from(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        id: TokenId,
        amount: E::Balance,
        data: Vec<u8>,
    ) -> Result<()> {
        if to == E::AccountId::default() {
            return Err(Error::AccountIsZero)
        }

        let operator = Self::caller();

        self._before_token_transfer(
            &operator,
            &Some(&from),
            &Some(&to),
            &vec![id],
            &vec![amount],
            &data,
        )?;

        let from_balance = self.get().balance_of(&id, &from);
        let to_balance = self.get().balance_of(&id, &to);
        assert!(
            from_balance >= amount,
            "ERC1155: insufficient balance for transfer"
        );
        self.get_mut()
            .set_balance(&from, &id, from_balance - amount);
        self.get_mut().set_balance(&to, &id, to_balance + amount);

        self._do_safe_transfer_acceptance_check(
            &operator,
            &Some(from.clone()),
            &to,
            &id,
            &amount,
            &data,
        );
        self.emit_event_transfer_single(operator, Some(from), Some(to), id, amount);

        Ok(())
    }

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_safeTransferFrom}.
    ///
    /// Emits a {TransferBatch} event.
    ///
    /// Requirements:
    ///
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
    /// acceptance magic value.
    fn _safe_batch_transfer_from(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        ids: Vec<TokenId>,
        amounts: Vec<E::Balance>,
        data: Vec<u8>,
    ) -> Result<()> {
        assert!(
            ids.len() == amounts.len(),
            "ERC1155: ids and amounts length mismatch"
        );

        if to == E::AccountId::default() {
            return Err(Error::AccountIsZero)
        }

        let operator = Self::caller();

        self._before_token_transfer(
            &operator,
            &Some(&from),
            &Some(&to),
            &ids,
            &amounts,
            &data,
        )?;

        for i in 0..ids.len() {
            let id = ids[i];
            let amount = amounts[i];

            let from_balance = self.get().balance_of(&id, &from);
            let to_balance = self.get().balance_of(&id, &to);
            assert!(
                from_balance >= amount,
                "ERC1155: insufficient balance for transfer"
            );
            self.get_mut()
                .set_balance(&from, &id, from_balance - amount);
            self.get_mut().set_balance(&to, &id, to_balance + amount);
        }

        self._do_safe_batch_transfer_acceptance_check(
            &operator,
            &Some(from.clone()),
            &to,
            &ids,
            &amounts,
            &data,
        );
        self.emit_event_transfer_batch(operator, Some(from), Some(to), ids, amounts);

        Ok(())
    }

    /// @dev Sets a new URI for all token types, by relying on the token type ID
    /// substitution mechanism
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
    ///
    /// By this mechanism, any occurrence of the `\{id\}` substring in either the
    /// URI or any of the amounts in the JSON file at said URI will be replaced by
    /// clients with the token type ID.
    ///
    /// For example, the `https://token-cdn-domain/\{id\}.json` URI would be
    /// interpreted by clients as
    /// `https://token-cdn-domain/000000000000000000000000000000000000000000000000000000000004cce0.json`
    /// for token type ID 0x4cce0.
    ///
    /// See {uri}.
    ///
    /// Because these URIs cannot be meaningfully represented by the {URI} event,
    /// this function emits no events.
    fn _set_url(&mut self, new_url: String) {
        self.get_mut().set_url(new_url)
    }

    /// @dev Creates `amount` tokens of token type `id`, and assigns them to `account`.
    ///
    /// Emits a {TransferSingle} event.
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    /// - If `account` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
    /// acceptance magic value.
    fn _mint(
        &mut self,
        account: E::AccountId,
        id: TokenId,
        amount: E::Balance,
        data: Vec<u8>,
    ) -> Result<()> {
        if account == E::AccountId::default() {
            return Err(Error::AccountIsZero)
        }

        let operator = Self::caller();

        self._before_token_transfer(
            &operator,
            &None,
            &Some(&account),
            &vec![id],
            &vec![amount],
            &data,
        )?;

        self.get_mut().add_balance(&account, &id, amount);
        self._do_safe_transfer_acceptance_check(
            &operator, &None, &account, &id, &amount, &data,
        );

        self.emit_event_transfer_single(operator, None, Some(account), id, amount);

        Ok(())
    }

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_mint}.
    ///
    /// Requirements:
    ///
    /// - `ids` and `amounts` must have the same length.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
    /// acceptance magic value.
    fn _mint_batch(
        &mut self,
        to: E::AccountId,
        ids: Vec<TokenId>,
        amounts: Vec<E::Balance>,
        data: Vec<u8>,
    ) -> Result<()> {
        assert!(
            ids.len() == amounts.len(),
            "ERC1155: ids and amounts length mismatch"
        );

        if to == E::AccountId::default() {
            return Err(Error::AccountIsZero)
        }

        let operator = Self::caller();

        self._before_token_transfer(&operator, &None, &Some(&to), &ids, &amounts, &data)?;

        for i in 0..ids.len() {
            let id = ids[i];
            let amount = amounts[i];

            self.get_mut().add_balance(&to, &id, amount);
        }

        self._do_safe_batch_transfer_acceptance_check(
            &operator, &None, &to, &ids, &amounts, &data,
        );
        self.emit_event_transfer_batch(operator, None, Some(to), ids, amounts);

        Ok(())
    }

    /// @dev Destroys `amount` tokens of token type `id` from `account`
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    /// - `account` must have at least `amount` tokens of token type `id`.
    fn _burn(
        &mut self,
        account: E::AccountId,
        id: TokenId,
        amount: E::Balance,
    ) -> Result<()> {
        if account == E::AccountId::default() {
            return Err(Error::AccountIsZero)
        }

        let operator = Self::caller();

        self._before_token_transfer(
            &operator,
            &Some(&account),
            &None,
            &vec![id],
            &vec![amount],
            &Vec::<u8>::default(),
        )?;

        let account_balance = self.get().balance_of(&id, &account);
        assert!(
            account_balance >= amount,
            "ERC1155: burn amount exceeds balance"
        );
        self.get_mut()
            .set_balance(&account, &id, account_balance - amount);

        self.emit_event_transfer_single(operator, Some(account), None, id, amount);

        Ok(())
    }

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_burn}.
    ///
    /// Requirements:
    ///
    /// - `ids` and `amounts` must have the same length.
    fn _burn_batch(
        &mut self,
        account: E::AccountId,
        ids: Vec<TokenId>,
        amounts: Vec<E::Balance>,
    ) -> Result<()> {
        assert!(
            ids.len() == amounts.len(),
            "ERC1155: ids and amounts length mismatch"
        );

        if account == E::AccountId::default() {
            return Err(Error::AccountIsZero)
        }

        let operator = Self::caller();

        self._before_token_transfer(
            &operator,
            &Some(&account),
            &None,
            &ids,
            &amounts,
            &Vec::<u8>::default(),
        )?;

        for i in 0..ids.len() {
            let id = ids[i];
            let amount = amounts[i];

            let account_balance = self.get().balance_of(&id, &account);
            assert!(
                account_balance >= amount,
                "ERC1155: burn amount exceeds balance"
            );
            self.get_mut()
                .set_balance(&account, &id, account_balance - amount);
        }

        self.emit_event_transfer_batch(operator, Some(account), None, ids, amounts);

        Ok(())
    }

    /// @dev Hook that is called before any token transfer. This includes minting
    /// and burning, as well as batched variants.
    ///
    /// The same hook is called on both single and batched variants. For single
    /// transfers, the length of the `id` and `amount` arrays will be 1.
    ///
    /// Calling conditions (for each `id` and `amount` pair):
    ///
    /// - When `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// of token type `id` will be  transferred to `to`.
    /// - When `from` is zero, `amount` tokens of token type `id` will be minted
    /// for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens of token type `id`
    /// will be burned.
    /// - `from` and `to` are never both zero.
    /// - `ids` and `amounts` have the same, non-zero length.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _before_token_transfer(
        &mut self,
        _operator: &E::AccountId,
        _from: &Option<&E::AccountId>,
        _to: &Option<&E::AccountId>,
        _ids: &Vec<TokenId>,
        _amounts: &Vec<E::Balance>,
        _data: &Vec<u8>,
    ) -> Result<()> {
        Ok(())
    }

    /// Do safe transfer accept check
    fn _do_safe_transfer_acceptance_check(
        &mut self,
        operator: &E::AccountId,
        from: &Option<E::AccountId>,
        to: &E::AccountId,
        id: &TokenId,
        amount: &E::Balance,
        data: &Vec<u8>,
    ) {
        let mut receiver = <Receiver as FromAccountId<E>>::from_account_id(to.clone());

        let from_account: Option<ink_env::AccountId> = match from {
            Some(account) => Some(account.clone().into()),
            None => None,
        };

        let resp = receiver
            .call_mut()
            .on_erc1155_received(
                operator.clone().into(),
                from_account,
                id.clone(),
                amount.clone().into(),
                data.clone(),
            )
            .fire();

        // TODO: use code gen
        let is_ok = match resp {
            Ok(selector_id) => selector_id == [194u8, 238u8, 217u8, 152u8],
            Err(err) => {
                match err {
                    ink_env::Error::NotCallable => true,
                    _ => panic!("ERC1155: transfer to non ERC1155Receiver implementer"),
                }
            }
        };

        assert!(
            is_ok,
            "ERC1155: transfer to non ERC1155Receiver implementer"
        )
    }

    /// Do safe transfer accept check for batch transfer
    fn _do_safe_batch_transfer_acceptance_check(
        &mut self,
        operator: &E::AccountId,
        from: &Option<E::AccountId>,
        to: &E::AccountId,
        ids: &Vec<TokenId>,
        amounts: &Vec<E::Balance>,
        data: &Vec<u8>,
    ) {
        let mut receiver = <Receiver as FromAccountId<E>>::from_account_id(to.clone());

        let from_account: Option<ink_env::AccountId> = match from {
            Some(account) => Some(account.clone().into()),
            None => None,
        };

        let amounts_param: Vec<u128> = amounts.iter().map(|a| a.clone().into()).collect();

        let resp = receiver
            .call_mut()
            .on_erc1155_batch_received(
                operator.clone().into(),
                from_account,
                ids.clone(),
                amounts_param,
                data.clone(),
            )
            .fire();

        // TODO: use code gen
        let is_ok = match resp {
            Ok(selector_id) => selector_id == [22u8, 32u8, 73u8, 133u8],
            Err(err) => {
                match err {
                    ink_env::Error::NotCallable => true,
                    _ => panic!("ERC1155: transfer to non ERC1155Receiver implementer"),
                }
            }
        };

        assert!(
            is_ok,
            "ERC1155: transfer to non ERC1155Receiver implementer"
        )
    }
}
