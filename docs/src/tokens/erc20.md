# ERC20

## Usage

## Messages for Txs

### transfer

### approve

### transfer_from

## Message for Querys

### name

### symbol

### decimals

### balance_of

### total_supply

### allowance

## APIs

### _mint

### _burn

### _transfer_from_to

## Hooks

### _before_token_transfer

## Events

### Transfer

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

