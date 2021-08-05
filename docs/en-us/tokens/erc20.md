# ERC20

Details of ERC20 can be found in [ERC20](https://eips.ethereum.org/EIPS/eip-20).

## Usage

To make a new erc20-like token, we should import erc20 at first:

```rust
#[metis_lang::contract]
pub mod contract {
    // use Error and Result for erc20
    pub use erc20::{
        Error,
        Result,
    };

    // use erc20 component
    use metis_erc20 as erc20;
    use metis_lang::{
        import,
        metis,
    };

    /// ERC-20 contract.
    #[ink(storage)]
    #[import(erc20)]
    pub struct Erc20 {
        erc20: erc20::Data<Erc20>,
    }

    // other logics
}
```

Then add the event for erc20:

```rust
    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: AccountId,
        pub value: Balance,
    }
```

Then implement the component:

```rust
    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc20::Impl<Erc20> for Erc20 {
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
        fn _before_token_transfer(
            &mut self,
            _from: &E::AccountId,
            _to: &E::AccountId,
            _amount: &E::Balance,
        ) -> Result<()>{
            // some logic

            Ok(())
        }
    }
```

impl the constructor for contract:

```rust
impl Erc20 {
    /// the constructor of the contract
    #[ink(constructor)]
    pub fn new(
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: Balance,
    ) -> Self {
        let mut instance = Self {
            erc20: erc20::Data::new(),
        };

        erc20::Impl::init(&mut instance, name, symbol, decimals, initial_supply);

        // do some other logic here

        instance
    }
  }
```

Then implement the messages for contract:

```rust
    impl Erc20 {
        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            erc20::Impl::name(self)
        }

        /// Returns the symbol of the token,
        /// usually a shorter version of the name.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc20::Impl::symbol(self)
        }

        /// Returns the number of decimals used to
        /// get its user representation.
        /// For example, if `decimals` equals `2`,
        /// a balance of `505` tokens should
        /// be displayed to a user as `5,05` (`505 / 10 ** 2`).
        ///
        /// Tokens usually opt for a value of 18,
        /// imitating the relationship between
        /// Ether and Wei in ETH. This is the value {ERC20} uses,
        /// unless this function is
        /// overridden;
        ///
        /// NOTE: This information is only used for _display_ purposes:
        /// it in no way affects any of the arithmetic of the contract
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            erc20::Impl::decimals(self)
        }

        /// Returns the amount of tokens in existence.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            erc20::Impl::total_supply(self)
        }

        /// Returns the amount of tokens owned by `account`.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            erc20::Impl::balance_of(self, owner)
        }

        /// Returns the remaining number of tokens that `spender` will be
        /// allowed to spend on behalf of `owner` through {transferFrom}. This is
        /// zero by default.
        ///
        /// This value changes when {approve} or {transferFrom} are called.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            erc20::Impl::allowance(self, owner, spender)
        }

        /// Moves `amount` tokens from the caller's account to `recipient`.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::transfer(self, to, value)
        }

        /// Sets `amount` as the allowance of `spender` over the caller's tokens.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// IMPORTANT: Beware that changing an allowance with this method brings
        /// the risk that someone may use both the old and the new allowance
        /// by unfortunate transaction ordering. One possible solution to 
        /// mitigate this race condition is to first reduce the spender's 
        /// allowance to 0 and set the desired value afterwards:
        /// <https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729>
        ///
        /// Emits an {Approval} event.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            erc20::Impl::approve(self, spender, value)
        }

        /// Moves `amount` tokens from `sender` to `recipient` using the
        /// allowance mechanism. `amount` is then deducted from the caller's
        /// allowance.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// Emits a {Transfer} event.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc20::Impl::transfer_from(self, from, to, value)
        }
    }
```

In the end, we can add some other messages.

## Messages for Txs

### transfer

Moves `amount` tokens from the caller's account to `recipient`.

Returns a Result indicating whether the operation succeeded.

Emits a `Transfer` event.

```rust
    /// Moves `amount` tokens from the caller's account to `recipient`.
    ///
    /// Returns a Result indicating whether the operation succeeded.
    ///
    /// Emits a `Transfer` event.
    fn transfer(&mut self, to: E::AccountId, value: E::Balance) -> Result<()> {
        self._transfer_from_to(Self::caller(), to, value)
    }
```

### approve

Sets `amount` as the allowance of `spender` over the caller's tokens.

Returns a boolean value indicating whether the operation succeeded.

IMPORTANT: Beware that changing an allowance with this method brings the risk
that someone may use both the old and the new allowance by unfortunate
transaction ordering. One possible solution to mitigate this race
condition is to first reduce the spender's allowance to 0 and set the
desired value afterwards:
<https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729>

Emits an `Approval` event.

```rust
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
    /// Emits an `Approval` event.
    fn approve(&mut self, spender: E::AccountId, amount: E::Balance) -> Result<()> {
        self._approve(Self::caller(), spender, amount)
    }
```

### transfer_from

Moves `amount` tokens from `sender` to `recipient` using the
allowance mechanism. `amount` is then deducted from the caller's
allowance.

Returns a boolean value indicating whether the operation succeeded.

Emits a `Transfer` event.

```rust
    /// Moves `amount` tokens from `sender` to `recipient` using the
    /// allowance mechanism. `amount` is then deducted from the caller's
    /// allowance.
    ///
    /// Returns a boolean value indicating whether the operation succeeded.
    ///
    /// Emits a `Transfer` event.
    fn transfer_from(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        amount: E::Balance,
    ) -> Result<()> {
        let caller = Self::caller();

        let current_allowance = self.get().allowance(from.clone(), caller.clone());
        if current_allowance < amount {
            return Err(Error::InsufficientAllowance)
        }

        self._transfer_from_to(from.clone(), to.clone(), amount.clone())?;

        self._approve(from, caller, current_allowance - amount)?;

        Ok(())
    }

```

## Message for Querys

### name

Returns the name of the token.

```rust
    /// Returns the name of the token.
    fn name(&self) -> String {
        self.get().name().clone()
    }
```

### symbol

Returns the symbol of the token, usually a shorter version of the name.

```rust
    /// Returns the symbol of the token, usually a shorter version of the name.
    fn symbol(&self) -> String {
        self.get().symbol().clone()
    }
```

### decimals

Returns the number of decimals used to get its user representation.
For example, if `decimals` equals `2`, a balance of `505` tokens should
be displayed to a user as `5,05` (`505 / 10 ** 2`).

Tokens usually opt for a value of 18, imitating the relationship between
Ether and Wei in ETH. This is the value {ERC20} uses, unless this function is
overridden;

> NOTE: This information is only used for _display_ purposes: it in
> no way affects any of the arithmetic of the contract

```rust
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
```

### balance_of

Returns the amount of tokens owned by `account`.

```rust
    /// Returns the amount of tokens owned by `account`.
    fn balance_of(&self, account: E::AccountId) -> E::Balance {
        self.get().balance_of(&account)
    }
```

### total_supply

Returns the amount of tokens in existence.

```rust
    /// Returns the amount of tokens in existence.
    fn total_supply(&self) -> E::Balance {
        self.get().total_supply()
    }
```

### allowance

Returns the remaining number of tokens that `spender` will be
allowed to spend on behalf of `owner` through `transfer_from`. This is
zero by default.

This value changes when `approve` or `transfer_from` are called.

```rust
    /// Returns the remaining number of tokens that `spender` will be
    /// allowed to spend on behalf of `owner` through `transfer_from`. This is
    /// zero by default.
    ///
    /// This value changes when `approve` or `transfer_from` are called.
    fn allowance(&self, owner: E::AccountId, spender: E::AccountId) -> E::Balance {
        self.get().allowance(owner, spender)
    }
```

## APIs

If the contract need make some logic by token, developers can based on this apis:

- `_mint` : mint token to a account with amount
- `_burn` : burn token from a account by amount
- `_transfer_from_to` : move token from a account to another

### _mint

Creates `amount` tokens and assigns them to `account`, increasing
the total supply.

Emits a `Transfer` event with `from` set to the zero address.

Requirements:

- `account` cannot be the zero address.

```rust
    /// Creates `amount` tokens and assigns them to `account`, increasing
    /// the total supply.
    ///
    /// Emits a `Transfer` event with `from` set to the zero address.
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    fn _mint(&mut self, account: E::AccountId, amount: E::Balance) -> Result<()> {
        let null_account = E::AccountId::default();
        if account == null_account {
            return Err(Error::AccountIsZero)
        }

        self._before_token_transfer(&null_account, &account, &amount)?;

        let total_supply = self.get().total_supply();
        let account_balance = self.get().balance_of(&account);

        self.get_mut().set_total_supply(total_supply + amount);
        self.get_mut()
            .set_balance(account.clone(), account_balance + amount);

        self.emit_event_transfer(None, Some(account), amount);

        Ok(())
    }
```

### _burn

Destroys `amount` tokens from `account`, reducing the total supply.

Emits a `Transfer` event with `to` set to the None address.

Requirements:

- `account` must have at least `amount` tokens.

```rust
    /// Destroys `amount` tokens from `account`, reducing the
    /// total supply.
    ///
    /// Emits a {Transfer} event with `to` set to the None address.
    ///
    /// Requirements:
    ///
    /// - `account` must have at least `amount` tokens.
    fn _burn(&mut self, account: E::AccountId, amount: E::Balance) -> Result<()> {
        let null_account = E::AccountId::default();

        if account == null_account {
            return Err(Error::AccountIsZero)
        }

        self._before_token_transfer(&account, &null_account, &amount)?;

        let account_balance = self.get().balance_of(&account);
        let total_supply = self.get().total_supply();

        if account_balance < amount {
            return Err(Error::InsufficientBalance)
        }

        self.get_mut()
            .set_balance(account.clone(), account_balance - amount);
        self.get_mut().set_total_supply(total_supply - amount);

        self.emit_event_transfer(Some(account), None, amount);

        Ok(())
    }
```

### _transfer_from_to

Moves tokens `amount` from `sender` to `recipient`.

This is internal function is equivalent to `transfer`, and can be used to

e.g. implement automatic token fees, slashing mechanisms, etc.
Emits a `Transfer` event.

Requirements:

- `sender` cannot be the zero address.
- `recipient` cannot be the zero address.
- `sender` must have a balance of at least `amount`.

```rust
    /// Moves tokens `amount` from `sender` to `recipient`.
    ///
    /// This is internal function is equivalent to `transfer`, and can be used to
    /// e.g. implement automatic token fees, slashing mechanisms, etc.
    ///
    /// Emits a `Transfer` event.
    ///
    /// Requirements:
    ///
    /// - `sender` cannot be the zero address.
    /// - `recipient` cannot be the zero address.
    /// - `sender` must have a balance of at least `amount`.
    fn _transfer_from_to(
        &mut self,
        sender: E::AccountId,
        recipient: E::AccountId,
        amount: E::Balance,
    ) -> Result<()> {
        let null_account = E::AccountId::default();

        if sender == null_account || recipient == null_account {
            return Err(Error::AccountIsZero)
        }

        self._before_token_transfer(&sender, &recipient, &amount)?;

        let sender_balance = self.get().balance_of(&sender);
        if sender_balance < amount {
            return Err(Error::InsufficientBalance)
        }

        self.get_mut().set_balance(sender.clone(), sender_balance - amount);
        let recipient_balance = self.get().balance_of(&recipient);
        self.get_mut()
            .set_balance(recipient.clone(), recipient_balance + amount);

        self.emit_event_transfer(Some(sender), Some(recipient), amount);

        Ok(())
    }
```

## Hooks

### _before_token_transfer

Hook that is called before any transfer of tokens. This includes
minting and burning.

Calling conditions:

- when `from` and `to` are both non-zero, `amount` of `from`'s tokens
will be to transferred to `to`.
- when `from` is zero, `amount` tokens will be minted for `to`.
- when `to` is zero, `amount` of `from`'s tokens will be burned.
- `from` and `to` are never both zero.

```rust
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
    fn _before_token_transfer(
        &mut self,
        _from: &E::AccountId,
        _to: &E::AccountId,
        _amount: &E::Balance,
    ) -> Result<()>{
        Ok(())
    }
```

## Events

### Transfer

Event emitted when a token transfer occurs.

```rust
    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub value: Balance,
    }
```

### Approval

Event emitted when an approval occurs that `spender` is allowed to withdraw up to the amount of `value` tokens from `owner`.

```rust
    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    #[metis(erc20)]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: AccountId,
        pub value: Balance,
    }
```
