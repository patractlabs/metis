# Ownable

`Ownable` component which provides a basic access control mechanism, where there is an account (an owner) that can be granted exclusive access to specific functions.

By default, the owner account will be the account which deploys the contract. This can later be changed with {transferOwnership}.

This components is used through inheritance. It will make available the func like `ensure_caller_is_owner`, which can be applied to your functions to restrict their use to the owner.

## Usage

To use the `Ownable` component need, There is a example:

```rust
#[metis_lang::contract]
pub mod contract {
    use metis_lang::{
        import,
        metis,
    };
    use metis_ownable as ownable;

    #[ink(storage)]
    #[import(ownable)]
    pub struct Contract {
        // ...
        ownable: ownable::Data<Contract>,
        // ...
    }

    // ...

    /// Event emitted when Owner AccountId Transferred
    #[ink(event)]
    #[metis(ownable)]
    pub struct OwnershipTransferred {
        /// previous owner account id
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        /// new owner account id
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self {
                // ...
                ownable: ownable::Data::new(),
                // ...
            };

            // ...

            ownable::Impl::init(&mut instance);

            instance
        }

        /// Return the owner AccountId
        #[ink(message)]
        pub fn owner(&self) -> Option<AccountId> {
            *ownable::Impl::owner(self)
        }

        /// Leaves the contract without owner. It will not be possible to call
        /// `ensure_xxx` functions anymore. Can only be called by the current owner.
        /// NOTE: Renouncing ownership will leave the contract without an owner,
        /// thereby removing any functionality that is only available to the owner.
        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            ownable::Impl::renounce_ownership(self)
        }

        /// Transfers ownership of the contract to a new account (`new_owner`).
        /// Can only be called by the current owner.
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            ownable::Impl::transfer_ownership(self, &new_owner)
        }

        /// Example for owner
        #[ink(message)]
        pub fn msg_use_owner(&mut self) {
            ownable::Impl::ensure_caller_is_owner(self);

            // other logics
        }
    }
}
```

## Module

`Ownable` components has one storage for the owner of the contract.

```rust
pub struct Data<E>
where
    E: Env,
{
    /// The owner of contract
    owner: Lazy<Option<E::AccountId>>,
}
```

In constructor of contract, can use `init` to make the owner to caller:

```rust
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self {
                // ...
                ownable: ownable::Data::new(),
                // ...
            };

            // ...
            ownable::Impl::init(&mut instance);
            // ...

            instance
        }
```

## Messages for Txs

`Ownable` component use `transfer_ownership` and `renounce_ownership` to modify the owner of the contract.
all this need owner as caller.

### renounce_ownership

`renounce_ownership` Leaves the contract without owner. It will not be possible to call `ensure_xxx` functions anymore. Can only be called by the current owner.

```rust
    /// Leaves the contract without owner. It will not be possible to call
    /// `ensure_xxx` functions anymore. Can only be called by the current owner.
    /// NOTE: Renouncing ownership will leave the contract without an owner,
    /// thereby removing any functionality that is only available to the owner.
    #[ink(message)]
    pub fn renounce_ownership(&mut self) {
        ownable::Impl::renounce_ownership(self)
    }
```

> **NOTE**: Renouncing ownership will leave the contract without an owner, thereby removing any functionality that is only available to the owner.

`renounce_ownership` will emit `OwnershipTransferred` event by `new_owner` to `None`.

### transfer_ownership

Transfers ownership of the contract to a new account (`new_owner`). Can only be called by the current owner.

```rust
    /// Transfers ownership of the contract to a new account (`new_owner`).
    /// Can only be called by the current owner.
    #[ink(message)]
    pub fn transfer_ownership(&mut self, new_owner: AccountId) {
        ownable::Impl::transfer_ownership(self, &new_owner)
    }
```

## Message for Querys

### owner

`owner` Return the owner AccountId.

```rust
    /// Return the owner AccountId.
    #[ink(message)]
    pub fn owner(&self) -> Option<AccountId> {
        *ownable::Impl::owner(self)
    }
```

## APIs

A contract with `Ownable` component can allow `owner` to be granted exclusive access to specific functions, this check need impl by `ensure_xxx` functions.

### ensure_owner

`ensure_owner` check if `owner` is the owner of the contract, panic if the `owner` not the owner of the contract.

```rust
    /// Panic if `owner` is not an owner
    fn ensure_owner(&self, owner: &E::AccountId) {
        assert!(&self.get().get_ownership().clone().unwrap() == owner);
    }
```

### ensure_caller_is_owner

`ensure_caller_is_owner` check if caller is the owner of the contract, panic if the caller not the owner of the contract.

```rust
    /// Panic if caller is not an owner
    fn ensure_caller_is_owner(&self) {
        self.ensure_owner(&Self::caller());
    }
```

### ensure_owner_renounce

`ensure_owner_renounce` check the current owner is renounced.

```rust
    /// Panic the contract owner is not renounced,
    fn ensure_owner_renounce(&self) {
        assert!(self.get().get_ownership().is_none());
    }
```

## Events

### OwnershipTransferred

The Event will emit when the ownership of the contract is transferred:

- in constructor, will emit `None` -> caller.
- in `transfer_ownership`, will emit caller -> new_owner.
- in `renounce_ownership`, will emit caller -> `None`

```rust
    /// Event emitted when Owner AccountId Transferred
    #[ink(event)]
    #[metis(ownable)]
    pub struct OwnershipTransferred {
        /// previous owner account id
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        /// new owner account id
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }
```
