# Use Component

## Metis-MCCI architecture

- `M` : Data model. Most contracts read and write contract world states. These states map to data models, each associated with only one component.
- `C` : component. A component is a reusable, independent implementation unit that encapsulates data and methods and maintains orthogonality with other components.
- `C` : controller. The controller coordinates the components and implements the contract interface.
- `I` : interface. The interface is the user interface of the contract. The interface defines the behavior of the contract and, to some extent, defines metadata.

```txt
┌───────┐          ┌───────────────┬────────────────────────────────┐
│       │          │ Interface     │ Control                        │
│       │          │               │  ┌─────────────────────┐       │
│       │          │  Constructor  │  │ Component           │       │
│ User  │  Call    │               │  │ ┌───────────────────┴──┐    │
│       ├─────────►│  Messages     │  │ │ Component            │    │
│       │          │               │  │ │ ┌────────────────────┴─┐  │
│       │          │  Events       │  │ │ │ Component            │  │
├───────┤          │               │  │ │ │        ┌───────────┐ │  │
│       │  Call    │               │  │ │ │ Msgs   │           │ │  │
│       ├─────────►│               │  │ │ │        │ Module    │ │  │
│       │          │               │  │ │ │ Apis   │           │ │  │
│ Apps  │          │               │  │ │ │        │           │ │  │
│       │  Event   │               │  └─┤ │ Events └───────────┘ │  │
│       │◄─────────┤               │    └─┤                      │  │
│       │          │               │      └──────────────────────┘  │
│       │          │               │                                │
└───────┘          └───────────────┴────────────────────────────────┘
```

As shown in the figure, under the MCCI architecture, the contract is divided into a series of reusable components. The contract behavior is implent through the collaboration of components, and the contract behavior is clearly defined by interface and controller.

The contract's interface defines the contract's behavior, including:

- constructor
- message
- event

The user of the contract interacts with the contract based on these three elements. In fact, these three elements also constitute ink! The main part of the contract metadata.

For a contract, these three things are guaranteed to be deterministic, unambiguous, and easy to understand. Therefore, the interface code of the contract code should be as cohesive as possible.

The contract controller is responsible for integrating the components. We break the main logic of the contract down into a series of reusable components, which can **extend** and **compose** based on other components.

A data model is the encapsulation of the state of a contract. A component of a contract needs a contract to meet the requirements of its data model. For a contract, its state will be represented as a combination of a series of data models.

In generally, the data model can also be considered as part of the contract behavior, and thus as part of the contract interface, but in most scenarios, external applications and users can not directly use the state of the chain, so the external encapsulation of the data model is not emphasized here.

## Inheritance Vs Composition

In contract development, we focus more on auditability of contracts, and the use of inheritance in solidity contract development will increase the difficulty of contract audit: The behavior logic of the contract is spread out in multiple files, even in different projects. Therefore, in Metis, we do not inherit the interface and implementation of the contract. The components and data model are introduced into the contract in a combined way.

Each component impl a series of functions, include the impl of messages and apis. Components can **extend** and **compose** based on other components.

For most of components, will like this:

```rust
/// The `EventEmit` impl the event emit api for ownable component.
pub trait EventEmit<E: Env>: EnvAccess<E> {
    /// Emit OwnershipTransferred event
    fn emit_event_ownership_transferred(
        &mut self,
        previous_owner: Option<E::AccountId>,
        new_owner: Option<E::AccountId>,
    );
}

/// The `Impl` define ownable component impl funcs
pub trait Impl<E: Env>: Storage<E, Data<E>> + EventEmit<E> {
    /// init Initializes the contract setting the deployer as the initial owner.
    fn init(&mut self) {
        // logic
    }

    /// Message impl 
    fn one_message_impl(&mut self) -> Result<()> {
        // msg impl which will call by ```xxx::Impl::one_message_impl(self)```

        // use the hook
        self.hook(xxx)?

        Ok(())
    }

    /// Message for Query impl
    fn one_query_impl(& self, param_acc: &E::AccountId) -> Data {
        Data::default()
    }

    /// API for other message
    fn check_xxx(&self, owner: &E::AccountId) {
    }

    // Hook which need impl by contract
    fn hook(&mut self, params: &E::Balance) -> Result<()>;
}

```

Some component will contains a default implementation:

```rust
// a default impl, each contract which impl storage and event emitter can be component
impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> Impl<E> for T {}
```

To use this component, we can import this to contract:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract] // use `metis_lang::contract`
pub mod contract {
    // use the component: xxx1 and xxx2
    use metis_component_xxx1 as xxx1;
    use metis_component_xxx2 as xxx2;

    // use `import` and `metis` marco
    use metis_lang::{
        import,
        metis,
    };

    #[ink(storage)]
    #[import(xxx1, xxx2)] // import the component
    pub struct Contract {
        // add data to storage, which use Contract as Env to Data
        xxx1: xxx1::Data<Contract>,
        xxx2: xxx2::Data<Contract>,
    }

    /// add event for component
    /// in emit it will be emit_event_ownership_transferred
    #[ink(event)]
    #[metis(xxx1)] // event for xxx1
    pub struct OwnershipTransferred {
        /// previous owner account id
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        /// new owner account id
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    /// Event emitted when payee withdraw
    #[ink(event)]
    #[metis(xxx2)] // event for xxx1
    pub struct OtherEvent {
        #[ink(topic)]
        pub payee: AccountId,
        pub amount: Balance,
    }

    impl xxx1::Impl<Contract> for Contract {
        fn hook(
            &mut self,
            params: &E::Balance
        ) -> Result<()> {
            // some logic

            Ok(())
        }
    }

    // impl
    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            // impl for default
            let mut instance = Self {
                xxx1: xxx1::Data::new(),
                xxx2: xxx2::Data::new(),
            };

            // init call
            xxx1::Impl::init(&mut instance);
            xxx2::Impl::init(&mut instance);

            // return instance
            instance
        }

        /// commits for one_message_impl
        #[ink(message)]
        pub fn one_message_impl(&mut self) -> Result<()> {
            // some other check
            xxx2::Impl::do_some_check(self);
            xxx1::Impl::one_message_impl(self)
        }

        /// commits for one_query_impl
        #[ink(message, payable)]
        pub fn one_query_impl(&self, payee: AccountId) {
            xxx1::Impl::one_query_impl(self, payee)
        }

        /// commits for other_message_impl
        #[ink(message)]
        pub fn other_message_impl(&mut self, payee: AccountId) {
            xxx1::Impl::check_xxx(self)
            // other logic
        }
    }

    #[cfg(test)]
    mod tests {
        // test for contract
    }
}
```

!> Warnning: We can redefine the name of metis component, but we should keep all name of component should be changed also.

**Note** this:

```rust
    use metis_component_xxx1 as xxx1;
```

if we use the `xxx1` be the alias of the `metis_component_xxx1`, so the all tag of components should be `xxx1`:

- The `import` marco for contract
- The name of item in contract struct type
- The `metis` marco for event

For example:

```rust
pub mod contract {
    // use the component: xxx1 and xxx2
    use metis_component_xxx1 as xxx1;

    ...

    #[ink(storage)]
    #[import(xxx1)] // The `import` marco for contract, should be xxx1
    pub struct Contract {
        //  The name of item in contract struct type, should be xxx1
        xxx1: xxx1::Data<Contract>,
    }

    ...

    #[ink(event)]
    #[metis(xxx1)] // The `metis` marco for event
    pub struct OwnershipTransferred {
    }

    ...
}
```

## Hook and Impl

In the last example, we can see the hook:

```rust
    // Hook which need impl by contract
    fn hook(&mut self, params: &E::Balance) -> Result<()>;
```

In some component, the hook has a default implementation:

```rust
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
    /// To learn more about hooks,
    /// head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _before_token_transfer(
        &mut self,
        _operator: &E::AccountId,
        _from: &Option<&E::AccountId>,
        _to: &Option<&E::AccountId>,
        _amount: &E::Balance,
    ) -> Result<()> {
        Ok(())
    }
```

The hook will call by component functions, user can define it, a normal example is Pausable ERC20 component:

```rust
    fn before_token_transfer(
        &mut self,
        _from: &E::AccountId,
        _to: &E::AccountId,
        _amount: E::Balance,
    ) -> Result<()> {
        metis_pausable::Impl::<E>::ensure_not_paused(self);

        Ok(())
    }
```

The Pausable ERC20 component is extend the erc20 component, which implements the hook.

## Metis Contract component

In previous versions of Metis, we will first implement openZeppelin-contracts components for developers to use. These components include:

- Token: ERC20, ERC721, ERC777, ERC1155 and extensions of the above Token contracts
- Access: Ownable, AccessControl, TimelockController
- Security: PullPayment, ReentrancyGuard, Pausable

Metis will implement a set of common components, similar to the OpenZeppelin-Contracts development library, which will be fully tested and code audited,
These components will be kept as consistent as possible with OpenZeppelin-contracts to reduce the developer's burden while effectively absorbing the experience of Solidity Ecology.
