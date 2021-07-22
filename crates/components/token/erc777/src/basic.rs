pub use super::module::Data;
use ink_prelude::{
    string::String,
    vec::Vec,
};
pub use metis_lang::{
    Env,
    EnvAccess,
    Storage,
};

/// The ERC-777 error types.
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

/// The ERC-777 result type.
pub type Result<T> = core::result::Result<T, Error>;

/// The `EventEmit` impl the event emit api for erc777 component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emitted when `value` tokens are moved from one account (`from`) to
    /// another (`to`).
    ///
    /// Note that `value` may be zero.
    fn emit_event_transfer(
        &mut self,
        from: Option<E::AccountId>,
        to: Option<E::AccountId>,
        value: E::Balance,
    );

    /// Emitted when the allowance of a `spender` for an `owner` is set by
    /// a call to {approve}. `value` is the new allowance.
    fn emit_event_approval(
        &mut self,
        owner: E::AccountId,
        spender: E::AccountId,
        value: E::Balance,
    );

    fn emit_event_sent(
        &mut self,
        operator: E::AccountId,
        from: E::AccountId,
        to: E::AccountId,
        amount: E::Balance,
        data: Vec<u8>,
        operator_data: Vec<u8>,
    );

    fn emit_event_minted(
        &mut self,
        operator: E::AccountId,
        to: E::AccountId,
        amount: E::Balance,
        data: Vec<u8>,
        operator_data: Vec<u8>,
    );

    fn emit_event_burned(
        &mut self,
        operator: E::AccountId,
        from: E::AccountId,
        amount: E::Balance,
        data: Vec<u8>,
        operator_data: Vec<u8>,
    );

    fn emit_event_authorized_operator(
        &mut self,
        operator: E::AccountId,
        token_holder: E::AccountId,
    );

    fn emit_event_revoked_operator(
        &mut self,
        operator: E::AccountId,
        token_holder: E::AccountId,
    );
}

/// The `Impl` define erc777 component impl funcs, with `_before_token_transfer` as hook
pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> {
    /// Initialize the erc777 component
    fn init(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: E::Balance,
    ) {
        let caller = &Self::caller();

        self.get_mut().set_total_supply(initial_supply);
        self.get_mut().set_balance(caller, initial_supply);
        self.get_mut().set_symbols(name, symbol, decimals);

        self.emit_event_transfer(None, Some(caller.clone()), initial_supply);
    }

    /// Returns the name of the token.
    fn name(&self) -> String {
        self.get().name().clone()
    }

    /// Returns the symbol of the token, usually a shorter version of the name.
    fn symbol(&self) -> String {
        self.get().symbol().clone()
    }

    /// Returns the number of decimals used to get its user representation.
    /// For example, if `decimals` equals `2`, a balance of `505` tokens should
    /// be displayed to a user as `5,05` (`505 / 10 ** 2`).
    ///
    /// Tokens usually opt for a value of 18, imitating the relationship between
    /// Ether and Wei in ETH. This is the value {ERC20} uses, unless this function is
    /// overridden;
    ///
    /// NOTE: This information is only used for _display_ purposes: it in
    /// no way affects any of the arithmetic of the contract
    fn decimals(&self) -> u8 {
        self.get().decimals().clone()
    }

    /// @dev Returns the smallest part of the token that is not divisible. This
    /// means all token operations (creation, movement and destruction) must have
    /// amounts that are a multiple of this number.
    ///
    /// For most token contracts, this value will equal 1.
    fn granularity(&self) -> E::Balance {
        E::Balance::from(1_u8)
    }

    /// Returns the amount of tokens in existence.
    fn total_supply(&self) -> E::Balance {
        self.get().total_supply()
    }

    /// Returns the amount of tokens owned by `account`.
    fn balance_of(&self, account: &E::AccountId) -> E::Balance {
        self.get().balance_of(account)
    }

    /// @dev Moves `amount` tokens from the caller's account to `recipient`.
    ///
    /// If send or receive hooks are registered for the caller and `recipient`,
    /// the corresponding functions will be called with `data` and empty
    /// `operator_data`. See {IERC777Sender} and {IERC777Recipient}.
    ///
    /// Emits a {Sent} event.
    ///
    /// Requirements
    ///
    /// - the caller must have at least `amount` tokens.
    /// - `recipient` cannot be the zero address.
    /// - if `recipient` is a contract, it must implement the {IERC777Recipient}
    /// interface.
    fn send(
        &mut self,
        recipient: E::AccountId,
        amount: E::Balance,
        data: Vec<u8>,
    ) -> Result<()> {
        self._send(
            Self::caller(),
            recipient,
            amount,
            data,
            Vec::default(),
            true,
        )
    }

    /// Moves `amount` tokens from the caller's account to `recipient`.
    ///
    /// Returns a boolean value indicating whether the operation succeeded.
    ///
    /// Emits a {Transfer} event.
    fn transfer(&mut self, recipient: &E::AccountId, amount: E::Balance) -> Result<()> {
        let null_account = &E::AccountId::default();
        let from = &Self::caller();

        if recipient == null_account {
            return Err(Error::AccountIsZero)
        }

        let null_data = &Vec::<u8>::default();

        self._call_tokens_to_send(
            &from,
            &Some(&from),
            &Some(&recipient),
            &amount,
            null_data,
            null_data,
        );

        self._move(&from, &from, &recipient, &amount, null_data, null_data)?;

        self._call_tokens_received(
            &from,
            &Some(&from),
            &Some(&recipient),
            &amount,
            null_data,
            null_data,
            false,
        );

        Ok(())
    }

    /// @dev Destroys `amount` tokens from the caller's account, reducing the
    /// total supply.
    ///
    /// If a send hook is registered for the caller, the corresponding function
    /// will be called with `data` and empty `operator_data`. See {IERC777Sender}.
    ///
    /// Emits a {Burned} event.
    ///
    /// Requirements
    ///
    /// - the caller must have at least `amount` tokens.
    fn burn(&mut self, amount: E::Balance, data: Vec<u8>) -> Result<()> {
        self._burn(Self::caller(), amount, data, Vec::default())
    }

    /// @dev Returns true if an account is an operator of `token_holder`.
    /// Operators can send and burn tokens on behalf of their owners. All
    /// accounts are their own operator.
    ///
    /// See {operatorSend} and {operatorBurn}.
    fn is_operator_for(&self, operator: E::AccountId, token_holder: E::AccountId) -> bool {
        self.get().is_operator_for(&operator, &token_holder)
    }

    /// @dev Make an account an operator of the caller.
    ///
    /// See {isOperatorFor}.
    ///
    /// Emits an {AuthorizedOperator} event.
    ///
    /// Requirements
    ///
    /// - `operator` cannot be calling address.
    fn authorize_operator(&mut self, operator: E::AccountId) {
        let caller = Self::caller();
        assert!(caller != operator, "ERC777: authorizing self as operator");

        let key = (caller.clone(), operator.clone());

        if self.get().is_default_operator(&operator) {
            self.get_mut()
                .revoked_default_operators
                .take(&key);
        } else {
            self.get_mut().operators.insert(key, ());
        }

        self.emit_event_authorized_operator(operator, caller);
    }

    /// @dev Revoke an account's operator status for the caller.
    ///
    /// See {isOperatorFor} and {defaultOperators}.
    ///
    /// Emits a {RevokedOperator} event.
    ///
    /// Requirements
    ///
    /// - `operator` cannot be calling address.
    fn revoke_operator(&mut self, operator: E::AccountId) {
        let caller = Self::caller();
        assert!(caller != operator, "ERC777: revoke self as operator");

        let key = (caller.clone(), operator.clone());

        if self.get().is_default_operator(&operator) {
            self.get_mut()
                .revoked_default_operators
                .insert(key, ());
        } else {
            self.get_mut().operators.take(&key);
        }

        self.emit_event_revoked_operator(operator, caller);
    }

    /// @dev Returns the list of default operators. These accounts are operators
    /// for all token holders, even if {authorizeOperator} was never called on
    /// them.
    ///
    /// This list is immutable, but individual holders may revoke these via
    /// {revokeOperator}, in which case {isOperatorFor} will return false.
    fn default_operators(&self) -> Vec<E::AccountId> {
        self.get().default_operators_array.clone()
    }

    /// @dev Moves `amount` tokens from `sender` to `recipient`. The caller must
    /// be an operator of `sender`.
    ///
    /// If send or receive hooks are registered for `sender` and `recipient`,
    /// the corresponding functions will be called with `data` and
    /// `operator_data`. See {IERC777Sender} and {IERC777Recipient}.
    ///
    /// Emits a {Sent} event.
    ///
    /// Requirements
    ///
    /// - `sender` cannot be the zero address.
    /// - `sender` must have at least `amount` tokens.
    /// - the caller must be an operator for `sender`.
    /// - `recipient` cannot be the zero address.
    /// - if `recipient` is a contract, it must implement the {IERC777Recipient}
    /// interface.
    fn operator_send(
        &mut self,
        sender: E::AccountId,
        recipient: E::AccountId,
        amount: E::Balance,
        data: Vec<u8>,
        operator_data: Vec<u8>,
    ) -> Result<()> {
        assert!(
            self.get().is_operator_for(&Self::caller(), &sender),
            "ERC777: caller is not an operator for holder"
        );

        self._send(sender, recipient, amount, data, operator_data, true)
    }

    /// @dev Destroys `amount` tokens from `account`, reducing the total supply.
    /// The caller must be an operator of `account`.
    ///
    /// If a send hook is registered for `account`, the corresponding function
    /// will be called with `data` and `operator_data`. See {IERC777Sender}.
    ///
    /// Emits a {Burned} event.
    ///
    /// Requirements
    ///
    /// - `account` cannot be the zero address.
    /// - `account` must have at least `amount` tokens.
    /// - the caller must be an operator for `account`.
    fn operator_burn(
        &mut self,
        account: E::AccountId,
        amount: E::Balance,
        data: Vec<u8>,
        operator_data: Vec<u8>,
    ) -> Result<()> {
        assert!(
            self.get().is_operator_for(&Self::caller(), &account),
            "ERC777: caller is not an operator for holder"
        );

        self._burn(account, amount, data, operator_data)
    }

    /// Returns the remaining number of tokens that `spender` will be
    /// allowed to spend on behalf of `owner` through {transferFrom}. This is
    /// zero by default.
    ///
    /// This value changes when {approve} or {transferFrom} are called.
    fn allowance(&self, owner: &E::AccountId, spender: &E::AccountId) -> E::Balance {
        self.get().allowance(owner, spender)
    }

    /// Sets `amount` as the allowance of `spender` over the caller's tokens.
    ///
    /// Returns a boolean value indicating whether the operation succeeded.
    ///
    /// IMPORTANT: Beware that changing an allowance with this method brings the risk
    /// that someone may use both the old and the new allowance by unfortunate
    /// transaction ordering. One possible solution to mitigate this race
    /// condition is to first reduce the spender's allowance to 0 and set the
    /// desired value afterwards:
    /// <https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729>
    ///
    /// Emits an {Approval} event.
    fn approve(&mut self, spender: &E::AccountId, amount: E::Balance) -> Result<()> {
        self._approve(&Self::caller(), spender, amount)
    }

    /// Moves `amount` tokens from `sender` to `recipient` using the
    /// allowance mechanism. `amount` is then deducted from the caller's
    /// allowance.
    ///
    /// Returns a boolean value indicating whether the operation succeeded.
    ///
    /// Emits a {Transfer} event.
    fn transfer_from(
        &mut self,
        holder: &E::AccountId,
        recipient: &E::AccountId,
        amount: E::Balance,
    ) -> Result<()> {
        let caller = &Self::caller();
        let null_account = &E::AccountId::default();

        if recipient == null_account {
            return Err(Error::AccountIsZero)
        }

        let current_allowance = self.get().allowance(holder, caller);
        if current_allowance < amount {
            return Err(Error::InsufficientAllowance)
        }

        let spender = Self::caller();

        self._call_tokens_to_send(
            &spender,
            &Some(&holder),
            &Some(&recipient),
            &amount,
            &Vec::default(),
            &Vec::default(),
        );

        self._approve(holder, caller, current_allowance - amount)?;

        self._move(
            &spender,
            &holder,
            &recipient,
            &amount,
            &Vec::default(),
            &Vec::default(),
        )?;

        self._call_tokens_received(
            &spender,
            &Some(&holder),
            &Some(&recipient),
            &amount,
            &Vec::default(),
            &Vec::default(),
            false,
        );

        Ok(())
    }

    /// @dev Creates `amount` tokens and assigns them to `account`, increasing
    /// the total supply.
    ///
    /// If a send hook is registered for `account`, the corresponding function
    /// will be called with `operator`, `data` and `operator_data`.
    ///
    /// See {IERC777Sender} and {IERC777Recipient}.
    ///
    /// Emits {Minted} and {IERC20-Transfer} events.
    ///
    /// Requirements
    ///
    /// - `account` cannot be the zero address.
    /// - if `account` is a contract, it must implement the {IERC777Recipient}
    /// interface.
    fn _mint(
        &mut self,
        account: E::AccountId,
        amount: E::Balance,
        user_data: Vec<u8>,
        operator_data: Vec<u8>,
    ) -> Result<()> {
        self._mint_required_reception_ack(
            account,
            amount,
            user_data,
            operator_data,
            true,
        )
    }

    /// @dev Creates `amount` tokens and assigns them to `account`, increasing
    /// the total supply.
    ///
    /// If `requireReceptionAck` is set to true, and if a send hook is
    /// registered for `account`, the corresponding function will be called with
    /// `operator`, `data` and `operator_data`.
    ///
    /// See {IERC777Sender} and {IERC777Recipient}.
    ///
    /// Emits {Minted} and {IERC20-Transfer} events.
    ///
    /// Requirements
    ///
    /// - `account` cannot be the zero address.
    /// - if `account` is a contract, it must implement the {IERC777Recipient}
    /// interface.
    fn _mint_required_reception_ack(
        &mut self,
        account: E::AccountId,
        amount: E::Balance,
        user_data: Vec<u8>,
        operator_data: Vec<u8>,
        required_reception_ack: bool,
    ) -> Result<()> {
        let null_account = E::AccountId::default();

        if account == null_account {
            return Err(Error::AccountIsZero)
        }

        let operator = Self::caller();

        self._before_token_transfer(&operator, &None, &Some(&account), &amount)?;

        // Update state variables
        let current_total = self.get().total_supply();
        let current_balance = self.get().balance_of(&account);

        self.get_mut().set_total_supply(current_total + amount);
        self.get_mut()
            .set_balance(&account, current_balance + amount);

        self._call_tokens_received(
            &operator,
            &None,
            &Some(&account),
            &amount,
            &user_data,
            &operator_data,
            required_reception_ack,
        );

        self.emit_event_minted(operator, account.clone(), amount, user_data, operator_data);
        self.emit_event_transfer(None, Some(account), amount);

        Ok(())
    }

    /// @dev Send tokens
    /// @param from address token holder address
    /// @param to address recipient address
    /// @param amount uint256 amount of tokens to transfer
    /// @param userData bytes extra information provided by the token holder (if any)
    /// @param operator_data bytes extra information provided by the operator (if any)
    /// @param requireReceptionAck if true, contract recipients are required to implement ERC777TokensRecipient
    fn _send(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        amount: E::Balance,
        user_data: Vec<u8>,
        operator_data: Vec<u8>,
        required_reception_ack: bool,
    ) -> Result<()> {
        let null_account = E::AccountId::default();

        assert!(from != null_account, "ERC777: send from the zero address");
        assert!(to != null_account, "ERC777: send to the zero address");

        let operator = Self::caller();

        self._call_tokens_to_send(
            &operator,
            &Some(&from),
            &Some(&to),
            &amount,
            &user_data,
            &operator_data,
        );

        self._move(&operator, &from, &to, &amount, &user_data, &operator_data)?;

        self._call_tokens_received(
            &operator,
            &Some(&from),
            &Some(&to),
            &amount,
            &user_data,
            &operator_data,
            required_reception_ack,
        );

        Ok(())
    }

    /// @dev Burn tokens
    /// @param from address token holder address
    /// @param amount uint256 amount of tokens to burn
    /// @param data bytes extra information provided by the token holder
    /// @param operator_data bytes extra information provided by the operator (if any)
    fn _burn(
        &mut self,
        from: E::AccountId,
        amount: E::Balance,
        data: Vec<u8>,
        operator_data: Vec<u8>,
    ) -> Result<()> {
        let null_account = E::AccountId::default();

        assert!(from != null_account, "ERC777: burn from the zero address");

        let operator = Self::caller();

        self._call_tokens_to_send(
            &operator,
            &Some(&from),
            &None,
            &amount,
            &data,
            &operator_data,
        );

        self._before_token_transfer(&operator, &Some(&from), &None, &amount)?;

        // Update state variables
        let from_balance = self.get().balance_of(&from);
        assert!(
            from_balance >= amount,
            "ERC777: burn amount exceeds balance"
        );
        self.get_mut().set_balance(&from, from_balance - amount);

        let current_total = self.get().total_supply();
        self.get_mut().set_total_supply(current_total - amount);

        self.emit_event_burned(operator, from.clone(), amount, data, operator_data);
        self.emit_event_transfer(Some(from), None, amount);

        Ok(())
    }

    fn _move(
        &mut self,
        operator: &E::AccountId,
        from: &E::AccountId,
        to: &E::AccountId,
        amount: &E::Balance,
        user_data: &Vec<u8>,
        operator_data: &Vec<u8>,
    ) -> Result<()> {
        self._before_token_transfer(&operator, &Some(from), &Some(to), &amount)?;

        let from_balance = self.get().balance_of(from);
        let to_balance = self.get().balance_of(to);
        if &from_balance < amount {
            return Err(Error::InsufficientBalance)
        }

        self.get_mut().set_balance(from, from_balance - *amount);
        self.get_mut().set_balance(to, to_balance + *amount);

        self.emit_event_sent(
            operator.clone(),
            from.clone(),
            to.clone(),
            amount.clone(),
            user_data.clone(),
            operator_data.clone(),
        );
        self.emit_event_transfer(Some(from.clone()), Some(to.clone()), amount.clone());

        Ok(())
    }

    /// The implementation of approve, for extensions call
    fn _approve(
        &mut self,
        owner: &E::AccountId,
        spender: &E::AccountId,
        amount: E::Balance,
    ) -> Result<()> {
        let null_account = &E::AccountId::default();

        if owner == null_account || spender == null_account {
            return Err(Error::AccountIsZero)
        }

        self.get_mut().set_allowance(owner, spender, amount);
        self.emit_event_approval(owner.clone(), spender.clone(), amount);

        Ok(())
    }

    /// @dev Call from.tokensToSend() if the interface is registered
    /// @param operator address operator requesting the transfer
    /// @param from address token holder address
    /// @param to address recipient address
    /// @param amount uint256 amount of tokens to transfer
    /// @param user_data bytes extra information provided by the token holder (if any)
    /// @param operator_data bytes extra information provided by the operator (if any)
    fn _call_tokens_to_send(
        &mut self,
        _operator: &E::AccountId,
        _from: &Option<&E::AccountId>,
        _to: &Option<&E::AccountId>,
        _amount: &E::Balance,
        _user_data: &Vec<u8>,
        _operator_data: &Vec<u8>,
    ) {
        // TODO: support ERC1820
    }

    /// @dev Call to.tokensReceived() if the interface is registered. Reverts if the recipient is a contract but
    /// tokensReceived() was not registered for the recipient
    /// @param operator address operator requesting the transfer
    /// @param from address token holder address
    /// @param to address recipient address
    /// @param amount uint256 amount of tokens to transfer
    /// @param userData bytes extra information provided by the token holder (if any)
    /// @param operator_data bytes extra information provided by the operator (if any)
    /// @param requireReceptionAck if true, contract recipients are required to implement ERC777TokensRecipient
    fn _call_tokens_received(
        &mut self,
        _operator: &E::AccountId,
        _from: &Option<&E::AccountId>,
        _to: &Option<&E::AccountId>,
        _amount: &E::Balance,
        _user_data: &Vec<u8>,
        _operator_data: &Vec<u8>,
        _required_reception_ack: bool,
    ) {
        // TODO: support ERC1820
    }

    /// @dev Hook that is called before any token transfer. This includes
    /// calls to {send}, {transfer}, {operatorSend}, minting and burning.
    ///
    /// Calling conditions:
    ///
    /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// will be to transferred to `to`.
    /// - when `from` is zero, `amount` tokens will be minted for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens will be burned.
    /// - `from` and `to` are never both zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _before_token_transfer(
        &mut self,
        _operator: &E::AccountId,
        _from: &Option<&E::AccountId>,
        _to: &Option<&E::AccountId>,
        _amount: &E::Balance,
    ) -> Result<()> {
        Ok(())
    }
}
