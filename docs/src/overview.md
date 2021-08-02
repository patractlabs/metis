# Overview

[Metis](https://github.com/patractlabs/metis) will be the Wasm contract standard library developed by Patract Labs. Patract Labs will work with community forces to formulate various Wasm contract standards, develop corresponding implementations of ink! and Ask! versions, and conduct audits by professional institutions.

## 0. Motivation

We believe that in the Wasm contract ecosystem, a development library similar to Openzeppelin-contracts is very necessary. This is Metis. For the sake of simplicity and readability, the patterns and meta-language provided by this library should be as consistent with Openzeppelin-contracts as possible, so as to reduce the burden on developers and effectively absorb the experience accumulated in the Solidity ecosystem.

But you need to realize that the Wasm contract development based on ink! and Ask! is very different from the EVM contract development based on Solidity. Wasm's mature system support allows developers to use a variety of different languages and tools to develop smart contracts. Based on these mature or rapid iteration platforms, developers can enjoy a large number of underlying facilities support and development experience accumulation, but different languages and tools often have different considerations and trade-offs. This means that developers will use completely different codes to express the same mechanism and design, which will bring huge communication costs and mental burdens to the community.

Faced with such problems, we need to summarize and accumulate the implementation model in the development of smart contracts. This idea was introduced in the book "Implementation Patterns" by Kent Beck. The purpose of summarizing the implementation mode is to clearly and accurately express the developer's intentions and ideas through these clear patterns, so that "code that others can understand" can be implemented. In contract development, this idea is very important.

From Openzeppelin-contracts, we can see that it summarizes several implementation models in the development of smart contracts, such as the "contract expansion model" based on the Solidity inheritance grammar. These implementation models are valuable experience accumulated in the Solidity ecology. Of course, different languages have different ways of practicing these modes. Metis will implement these modes and provide corresponding support on different platforms. For example, in Rust, it is obvious that you cannot directly use inheritance to implement contract expansion. For this, Metis will provide code implementation examples and provide a series of help libraries to reduce the developer's mental cost.

Metis will be more than just a Wasm contract standard library. We hope that through the practice of Metis, we can fully inherit and absorb previous contract development experience while exploring the best practices of Wasm contract development.

## 1. Problem

At present, contract-based developers mainly face the following problems:

- Lack of reliable implementation of common contracts (such as ERC20)
- It is difficult to achieve contract combination and expansion similar to the Solidity inheritance mechanism
- Lack of a series of reliable public components to implement contracts
- Lack of experience accumulation and model summary based on ink! development contracts

The above problems severely limit the current ink!-based contract development ecology. Metis will solve these problems while avoiding the existing problems in Solidity.

## 2. Contract Standard Library

Metis will implement a series of common components, similar to the Openzeppelin-contracts development library. These components will be thoroughly tested and code audited. These components will be as consistent as possible with Openzeppelin-contracts, which can reduce the burden on developers and effectively absorb the experience accumulated in the Solidity ecosystem.

Metis will include the following components:

- Openzeppelin-contracts-like components, including basic Access and Security components, as well as Token and Governance components
- Component developed for ink! contracts, wasm-based contracts can support better abstraction mechanisms, and we can implement more complex and practical components
- Summarizing abstract components from mature contract projects, metis will absorb the experience and accumulation of the community, including both the mature Solidity ecology and emerging blockchain contract projects.
- The expansion of ink! contracts, including basic data structures for different scenarios, etc.

In the previous version of Metis, we will first implement Openzeppelin-contracts-like components for developers to use. These components include:

- Token: ERC20, ERC721, ERC777, ERC1155 and the expansion contract of the above Token contract
- Access: Ownable, AccessControl, TimelockController
- Security: PullPayment, ReentrancyGuard, Pausable

## 3. ink! component

Most of Metis development libraries are composed of contract components. In Solidity, the introduction of contract components can be implemented based on inheritance. Generally, the contract components will include the following parts:

- Component Storage: The storage state related to the logic of the component itself, these states do not need to be exposed to the outside
- Component Message: The externally-facing Message of the component
- Component Event: The event that the component's own logic will  emit
- Component internal interface: an interface for other logic in the contract to call

For example, common Ownable contracts:

```Solidity
abstract contract Ownable is Context {
    // Component Storage
    address private _owner;

    // Component Event
    event OwnershipTransferred(
        address indexed previousOwner,
        address indexed newOwner
    );
    
    // Component internal interface
    modifier onlyOwner() {
        ...
    }

    // Component Event
    function renounceOwnership() public virtual onlyOwner {
        ...
    }

    ...
}
```

If a contract needs to be Ownable, just inherit the contract:

```Solidity
contract Escrow is Ownable {
...
}
```

Similarly, other components can also have Ownable through inheritance:

```Solidity
contract Pausable is Ownable {
  ...
  function unpause() public onlyOwner whenPaused {
      ...
  }
}
```

The intention of inheritance here is actually composition rather than an `is-a` relationship. Although excessive use of inheritance in Solidity will cause many problems, inheritance for specific intentions is still an important way to achieve it.

In contract development based on ink!, through metis, we can also achieve the same motivation based on generics and traits:

```rust
...

#[metis::contract] // metis contract macro, will use ink_lang::contract auto
mod flipper {
    ...

    #[ink(storage)]
    #[import(ownable)] // flipper import the ownable
    pub struct Flipper {
        ownable: ownable::Data<Flipper>, // data by ownable

        value: bool,
    }

    #[ink(event)]
    #[metis(ownable)] // event in ink! will refactor
    pub struct OwnershipTransferred {
        //...
    }

    impl Flipper {
        //...

        #[ink(message)]
        pub fn flip(&mut self) {
            // check owner
            ownable::Impl::ensure_caller_is_owner(self);

            self.value = !self.value;
        }

        //...

        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            ownable::Impl::renounce_ownership(self) // owner message
        }

        //...
    }
}
```

Metis assists developers to achieve the same function through a series of helper macros. In order to improve the auditability of the contract, here we hope that users clearly implement storage, event and message declarations.

At the same time, metis has improved the implementation of components:

```rust
//...

// Storage
#[metis::component::storage]
pub struct Data<E>
where
    E: Env,
{
    owner: Lazy<Option<E::AccountId>>,
}

// Event trait
pub trait EventEmit<E: Env>: EnvAccess<E> {
    //...
}

// Impl trait
pub trait Impl<E: Env>: Storage<E> + EventEmit<E> {
    // logics
    fn init(&mut self) {
        self.get_mut().set_ownership(&Some(Self::caller()));
    }

    fn renounce_ownership(&mut self) {
        self.ensure_caller_is_owner();

        self.emit_event_ownership_transferred(
            self.get().get_ownership().clone(),
            None);

        self.get_mut().set_ownership(&None);
    }

    //...

    /// Panic if `owner` is not an owner
    fn ensure_owner(&self, owner: &E::AccountId) {
        assert!(&self.get().get_ownership().clone().unwrap() == owner);
    }

    //...
}

```

Such components can extend their functions by inheriting other components, such as an ERC20 component with the function of destroying tokens:

```rust
pub trait Impl<E>: erc20::Impl<E>
where
    E: Env,
{
    fn _burn(&mut self, account: &E::AccountId, amount: E::Balance) -> Result<()> {
        //...
    }

    fn burn(&mut self, amount: E::Balance) -> Result<()> {
        self._burn(&Self::caller(), amount)
    }

    fn burn_from(&mut self, account: &E::AccountId, amount: E::Balance) -> Result<()> {
        //...
    }
}
```

Based on metis, we can implement various contract combination modes implemented by Solidity through inheritance under limited intent, and at the same time, with the help of rust's zero-cost abstraction, these abstractions will not bring additional performance consumption.

## 4. Overall Roadmap

Metis for ink! is divided into several milestones:

- **[M1]** Implement basic component macros and components, improve component testing support, developers can build regular DAPPs based on Metis
- **[M2]** Complete component macros, complete component development support, developers can build custom components. Complete the api support corresponding to the metis component.
- **[M3]** Rich component library, complete component and API support for governance and financial mechanism, perfect mathematical library suitable for contract development to support DeFi-type contracts that require complex calculations.

Considering that the current ink! and contract-pallet are still in iteration, some metis features will be implemented based on subsequent improvements, including:

- Contract proxy and upgradeable support will depend on the improvement of subsequent contract calls, by [739](https://github.com/paritytech/ink/issues/739).
- The Event in the component, the Event in the current ink! cannot be independent of the contract, by [759](https://github.com/paritytech/ink/issues/759), the event in the current component is only an early implementation, and it will be refactored based on the improvement of ink! in the future.

With the richness and completeness of the ink! contract community, metis will further implement more public components and libraries to assist developers in developing large-scale contract projects. Therefore, we may arrange [MR] milestones. It will be developed based on the iterative schedule of ink!

**[MR]** According to the ink!'s iterative progress, community feedback, contract upgrades, contract proxy and cross-contract call support, refactor Event-related implementations, improve basic components and add development assistance macros to reduce duplication while ensuring auditability Code.
