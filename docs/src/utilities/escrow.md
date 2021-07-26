# Escrow

Base escrow contract, holds funds designated for a payee until they
withdraw them.

Intended usage: This contract (and derived escrow contracts) should be a
standalone contract, that only interacts with the contract that instantiated
it. That way, it is guaranteed that all Ether will be handled according to
the `Escrow` rules, and there is no need to check for payable functions or
transfers in the inheritance tree. The contract that uses the escrow as its
payment method should be its owner, and provide public methods redirecting
to the escrow's deposit and withdraw.

## Usage

To use `Escrow` component, should import escrow, in most cases it also need a access control component, the example we use ownable:

```rust
#[metis_lang::contract]
pub mod mock {
    use metis_escrow as escrow;
    use metis_lang::{
        import,
        metis,
    };
    use metis_ownable as ownable;

    #[ink(storage)]
    #[import(ownable, escrow)]
    pub struct Escrow {
        ownable: ownable::Data<Escrow>,
        escrow: escrow::Data<Escrow>,
    }

    // others
}
```

then define the events:

```rust
    /// Event emitted when payee deposit amount
    #[ink(event)]
    #[metis(escrow)]
    pub struct Deposited {
        #[ink(topic)]
        pub payee: AccountId,
        pub amount: Balance,
    }

    /// Event emitted when payee withdraw
    #[ink(event)]
    #[metis(escrow)]
    pub struct Withdrawn {
        #[ink(topic)]
        pub payee: AccountId,
        pub amount: Balance,
    }

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

`OwnershipTransferred` event is for ownable component.

Add the constructor:

```rust
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self {
                ownable: ownable::Data::new(),
                escrow: escrow::Data::new(),
            };

            ownable::Impl::init(&mut instance);

            // escrow not need init

            instance
        }

```

The messages, not forget the `payable`:

```rust
        /// return the deposits of account
        #[ink(message)]
        pub fn deposits_of(&self, payee: AccountId) -> Balance {
            escrow::Impl::deposits_of(self, &payee)
        }

        /// deposit by payee, the pay value is the amount to transfer
        #[ink(message, payable)]
        pub fn deposit(&mut self, payee: AccountId) {
            ownable::Impl::ensure_caller_is_owner(self);
            escrow::Impl::deposit(self, payee)
        }

        // withdraw all deposits from the payee
        #[ink(message)]
        pub fn withdraw(&mut self, payee: AccountId) {
            ownable::Impl::ensure_caller_is_owner(self);
            escrow::Impl::withdraw(self, payee)
        }
```

In most cases, the owner of escrow contract should be another contract, the contract call the `deposit` and `withdraw` from its contract.

To call the escrow contract, developer can use the [stub]([../../../crates/components/utils/stub/src/lib.rs](https://github.com/patractlabs/metis/blob/master/crates/components/utils/escrow/stub/src/lib.rs)) of the escrow

## Module

the deposits is the map of payee to balance:

```rust
/// The Data of escrow component
#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E>
where
    E: Env,
{
    /// The owner of contract
    pub deposits: StorageHashMap<E::AccountId, E::Balance>,
}
```

## Messages for Txs

### deposit

Stores the sent amount as credit to be withdrawn.

```rust
    /// Stores the sent amount as credit to be withdrawn.
    /// @param payee The destination address of the funds.
    fn deposit(&mut self, payee: E::AccountId) {
        self.ensure_caller_is_owner();

        let amount = Self::transferred_balance();

        Storage::<E, Data<E>>::get_mut(self).add(&payee, &amount);

        self.emit_event_deposited(payee, amount);
    }
```

### withdraw

Withdraw accumulated balance for a payee, forwarding all gas to the recipient.

> WARNING: Forwarding all gas opens the door to reentrancy vulnerabilities.
> Make sure you trust the recipient, or are either following the
> checks-effects-interactions pattern or using {ReentrancyGuard}.

param:

- `payee` : The address whose funds will be withdrawn and transferred to.

```rust
    /// @dev Withdraw accumulated balance for a payee, forwarding all gas to the
    /// recipient.
    ///
    /// WARNING: Forwarding all gas opens the door to reentrancy vulnerabilities.
    /// Make sure you trust the recipient, or are either following the
    /// checks-effects-interactions pattern or using {ReentrancyGuard}.
    ///
    /// @param payee The address whose funds will be withdrawn and transferred to.
    fn withdraw(&mut self, payee: E::AccountId) {
        self.ensure_caller_is_owner();

        let payment = Storage::<E, Data<E>>::get(self).get(&payee);

        Storage::<E, Data<E>>::get_mut(self).clean(&payee);

        let res = Self::transfer(payee.clone(), payment);
        assert!(res.is_ok(), "Escrow: transfer to payee error");

        self.emit_event_withdrawn(payee, payment);
    }
```

## Message for Querys

### deposits_of

Return the deposits of payee

```rust
    /// Return the deposits of payee
    fn deposits_of(&self, payee: &E::AccountId) -> E::Balance {
        Storage::<E, Data<E>>::get(self).get(payee)
    }
```

## Events

### Deposited

Event emitted when payee deposit amount

```rust
    /// Event emitted when payee deposit amount
    #[ink(event)]
    #[metis(escrow)]
    pub struct Deposited {
        #[ink(topic)]
        pub payee: AccountId,
        pub amount: Balance,
    }
```

### Withdrawn

Event emitted when payee withdraw

```rust
    /// Event emitted when payee withdraw
    #[ink(event)]
    #[metis(escrow)]
    pub struct Withdrawn {
        #[ink(topic)]
        pub payee: AccountId,
        pub amount: Balance,
    }
```
