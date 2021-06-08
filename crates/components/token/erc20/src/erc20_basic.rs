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
}

/// The ERC-20 result type.
pub type Result<T> = core::result::Result<T, Error>;

/// The `EventEmit` impl the event emit api for erc20 component.
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
/// 
pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> {
    /// Initialize the erc20 component
    fn init(&mut self, name: String, symbol: String, initial_supply: E::Balance) {
        let caller = &Self::caller();

        self.get_mut().set_total_supply(initial_supply);
        self.get_mut().set_balance(caller, initial_supply);
        self.get_mut().set_symbols(name, symbol);

        self.emit_event_transfer(None, Some(caller.clone()), initial_supply);
    }

    /// Hook that is called before any transfer of tokens. This includes
    /// minting and burning.
    ///
    /// Calling conditions:
    ///
    /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// will be to transferred to `to`.
    /// - when `from` is zero, `amount` tokens will be minted for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens will be burned.
    /// - `from` and `to` are never both zero.
    fn _before_token_transfer(&mut self, from: &E::AccountId, to: &E::AccountId, amount: E::Balance) -> Result<()>;

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
        18_u8
    }

    /// Returns the amount of tokens in existence.
    fn total_supply(&self) -> E::Balance {
        self.get().total_supply()
    }

    /// Returns the amount of tokens owned by `account`.
    fn balance_of(&self, owner: &E::AccountId) -> E::Balance {
        self.get().balance_of(owner)
    }

    /// Returns the remaining number of tokens that `spender` will be
    /// allowed to spend on behalf of `owner` through {transferFrom}. This is
    /// zero by default.
    ///
    /// This value changes when {approve} or {transferFrom} are called.
    fn allowance(&self, owner: &E::AccountId, spender: &E::AccountId) -> E::Balance {
        self.get().allowance(owner, spender)
    }

    /// Moves `amount` tokens from the caller's account to `recipient`.
    ///
    /// Returns a boolean value indicating whether the operation succeeded.
    ///
    /// Emits a {Transfer} event.
    fn transfer(&mut self, to: &E::AccountId, value: E::Balance) -> Result<()> {
        self._transfer_from_to(&Self::caller(), to, value)
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
        self._approve(&Self::caller(), spender, amount);
        Ok(())
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
        from: &E::AccountId,
        to: &E::AccountId,
        amount: E::Balance,
    ) -> Result<()> {
        let caller = &Self::caller();

        let current_allowance = self.get().allowance(from, caller);
        if current_allowance < amount {
            return Err(Error::InsufficientAllowance)
        }

        self._transfer_from_to(from, to, amount)?;

        self._approve(from, caller, current_allowance - amount);

        Ok(())
    }

    /// The implementation of approve, for extensions call
    fn _approve(
        &mut self,
        owner: &E::AccountId,
        spender: &E::AccountId,
        amount: E::Balance,
    ) {
        let null_account = &E::AccountId::default();

        assert!(owner != null_account);
        assert!(spender != null_account);

        self.get_mut().set_allowance(owner, spender, amount);
        self.emit_event_approval(owner.clone(), spender.clone(), amount);
    }

    /// Moves tokens `amount` from `sender` to `recipient`.
    ///
    /// This is internal function is equivalent to {transfer}, and can be used to
    /// e.g. implement automatic token fees, slashing mechanisms, etc.
    ///
    /// Emits a {Transfer} event.
    ///
    /// Requirements:
    ///
    /// - `sender` cannot be the zero address.
    /// - `recipient` cannot be the zero address.
    /// - `sender` must have a balance of at least `amount`.
    fn _transfer_from_to(
        &mut self,
        sender: &E::AccountId,
        recipient: &E::AccountId,
        amount: E::Balance,
    ) -> Result<()> {
        let null_account = &E::AccountId::default();

        assert!(sender != null_account);
        assert!(recipient != null_account);

        self._before_token_transfer(sender, recipient, amount)?;

        let sender_balance = self.get().balance_of(sender);
        if sender_balance < amount {
            return Err(Error::InsufficientBalance)
        }

        self.get_mut().set_balance(sender, sender_balance - amount);
        let recipient_balance = self.get().balance_of(recipient);
        self.get_mut()
            .set_balance(recipient, recipient_balance + amount);

        self.emit_event_transfer(Some(sender.clone()), Some(recipient.clone()), amount);

        Ok(())
    }

    /// Destroys `amount` tokens from `account`, reducing the
    /// total supply.
    ///
    /// Emits a {Transfer} event with `to` set to the None address.
    ///
    /// Requirements:
    ///
    /// - `account` must have at least `amount` tokens.
    fn _burn(&mut self, account: &E::AccountId, amount: E::Balance) -> Result<()> {
        let null_account = &E::AccountId::default();
        assert!(account != null_account);

        self._before_token_transfer(account, null_account, amount)?;
        
        let account_balance = self.get().balance_of(account);
        let total_supply = self.get().total_supply();

        assert!(account_balance >= amount);
        self.get_mut()
            .set_balance(account, account_balance - amount);
        self.get_mut().set_total_supply(total_supply - amount);

        self.emit_event_transfer(Some(account.clone()), None, amount);

        Ok(())
    }
}
