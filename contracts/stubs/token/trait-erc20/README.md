# Trait ERC20 Interface

## Usage
### Cargo.toml Setting
```
[dependencies]
trait-erc20-stub = { git = "https://github.com/patractlabs/metis", default-features = false, features = ["ink-as-dependency"] }

[features]
default = ["std"]
std = [
    "trait-erc20-stub/std",
]
```
### Example Contract
```rust
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod delegate {
    use trait_erc20_stub::Erc20Stub;
    use ink_env::call::FromAccountId;

    #[ink(storage)]
    pub struct Delegate {
        token: Erc20Stub,
    }

    impl Delegate {
        #[ink(constructor)]
        pub fn new(contract_account: AccountId) -> Self {
            let token: Erc20Stub = FromAccountId::from_account_id(contract_account);
            Self { token }
        }

        #[ink(message)]
        pub fn call(&self, owner: AccountId) -> Balance {
            self.token.balance_of(owner)
        }
    }
}
```
