# Pausable

Contract module which allows children to implement an emergency stop
mechanism that can be triggered by an authorized account.

This module is used through inheritance. It will make available the
api `ensure_not_paused` and `ensure_paused`, which can be applied to
the functions of your contract. Note that they will not be pausable by
simply including this module, only once the modifiers are put in place.

## Usage

To use `Pausable` component, should import pausable, at the same time, if use pausable component,
we need some access control to control pause or unpause, so we will also import ownerable module.

```rust
#[metis_lang::contract]
mod contract {
    use metis_lang::{
        import,
        metis,
    };
    
    use metis_ownable as ownable;
    use metis_pausable as pausable;

    #[ink(storage)]
    #[import(pausable, ownable)]
    pub struct PausableContract {
        pausable: pausable::Data,
        ownable: ownable::Data<Flipper>,

        // other datas
    }
}
```

Note pausable module not need use Env for types in environment, just use `pausable::Data`.

Add Events which pausable need:

```rust
    /// Event emitted when Pause
    #[ink(event)]
    #[metis(pausable)]
    pub struct Paused {
        /// paused caller
        #[ink(topic)]
        account: AccountId,
    }

    /// Event emitted when unPause
    #[ink(event)]
    #[metis(pausable)]
    pub struct Unpaused {
        /// unpaused caller
        #[ink(topic)]
        account: AccountId,
    }

    /// Need OwnershipTransferred for ownerable.
```

in constructor, call the init from pausable:

```rust
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self {
            pausable: pausable::Data::default(),
            ownable: ownable::Data::default(),

            // other datas default data
        };

        pausable::Impl::init(&mut instance);
        ownable::Impl::init(&mut instance);

        // other initializes

        instance
    }
```

then add the message to control if is paused:

```rust
    /// Returns true if the contract is paused, and false otherwise
    #[ink(message)]
    pub fn paused(&self) -> bool {
        pausable::Impl::paused(self)
    }

    /// Pause the contract, will emit the `Paused` Event
    ///
    /// Requirements:
    ///
    /// - The contract must be not paused.
    /// - The caller should be the owner of contract
    #[ink(message)]
    pub fn pause(&mut self) {
        ownable::Impl::ensure_caller_is_owner(self);
        pausable::Impl::_pause(self)
    }

    /// Unpause the contract, will emit the `Unpaused` Event
    ///
    /// Requirements:
    ///
    /// - The contract must be paused.
    /// - The caller should be the owner of contract
    #[ink(message)]
    pub fn unpause(&mut self) {
        ownable::Impl::ensure_caller_is_owner(self);
        pausable::Impl::_unpause(self)
    }
```

In this example, only owner can pause or unpause the contract. In different contracts, this also can impl by access-control component.

In other contract messages, we can use `ensure_paused` and `ensure_not_paused` to control contract logic by paused:

```rust
    #[ink(message)]
    pub fn do_sth(&mut self) {
        // if contract is paused, the do_sth cannot called
        pausable::Impl::ensure_not_paused(self);
        
        // logic for do_sth
    }
```

## Module

`Pausable` has only one state for pause or unpause:

```rust
/// The Data of pausable component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data {
    /// is contract current paused
    paused: Lazy<bool>,
}
```

## Messages for Txs

To control a contract is paused or unpause, we need impl some access limit for it, so the `_pause` and `_unpause` should call by develop.

> **WARNNING** NOT use `_pause` and `_unpause` directly.

### _pause

Pause the contract, will emit the Paused Event.

Requirements:

- The contract must be not paused.

```rust
    /// Pause the contract, will emit the Paused Event
    ///
    /// Requirements:
    ///
    /// - The contract must be not paused.
    fn _pause(&mut self) {
        self.ensure_not_paused();
        self.get_mut().pause();
        self.emit_event_paused(Self::caller());
    }
```

### _unpause

Unpause the contract, will emit the `Unpaused` Event

Requirements:

- The contract must be paused.

```rust
    /// Unpause the contract, will emit the `Unpaused` Event
    ///
    /// Requirements:
    ///
    /// - The contract must be paused.
    fn _unpause(&mut self) {
        self.ensure_paused();
        self.get_mut().unpause();
        self.emit_event_unpaused(Self::caller());
    }
```

## Message for Querys

### paused

Returns true if the contract is paused, and false otherwise

```rust
    /// Returns true if the contract is paused, and false otherwise
    fn paused(&self) -> bool {
        self.get().is_paused()
    }
```

## APIs

A contract with `Pausable` component can use `ensure_paused` and `ensure_not_paused` to make sure the messages cannot used when paused.

### ensure_paused

Panic if current is not paused.

```rust
    /// Panic if current is not paused.
    fn ensure_paused(&self) {
        assert!(self.get().is_paused(), "Pausable: ensure paused");
    }
```

### ensure_not_paused

Panic if current is paused.

```rust
    /// Panic if current is paused.
    fn ensure_not_paused(&self) {
        assert!(!self.get().is_paused(), "Pausable: ensure not paused");
    }
```

## Events

### Paused

The Event will emit when the contract is into paused state.

```rust
    /// Event emitted when Pause
    #[ink(event)]
    #[metis(pausable)]
    pub struct Paused {
        /// paused caller
        #[ink(topic)]
        account: AccountId,
    }
```

### Unpaused

The Event will emit when the contract is unpaused.

```rust
    /// Event emitted when unPause
    #[ink(event)]
    #[metis(pausable)]
    pub struct Unpaused {
        /// unpaused caller
        #[ink(topic)]
        account: AccountId,
    }
```
