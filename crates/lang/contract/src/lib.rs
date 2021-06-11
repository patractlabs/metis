#![cfg_attr(not(feature = "std"), no_std)]

mod arithmetic;
mod traits;

pub use traits::{
    AccountId,
    Balance,
    BlockNumber,
    ChainExtension,
    Hash,
    Timestamp,
};

pub trait Env: 'static {
    type AccountId: AccountId;
    type Balance: Balance;
    type BlockNumber: BlockNumber;
    type Hash: Hash;
    type Timestamp: Timestamp;
}

impl Env for ink_env::DefaultEnvironment{
    type AccountId = <ink_env::DefaultEnvironment as ink_env::Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as ink_env::Environment>::Balance;
    type BlockNumber = <ink_env::DefaultEnvironment as ink_env::Environment>::BlockNumber;
    type Hash = <ink_env::DefaultEnvironment as ink_env::Environment>::Hash;
    type Timestamp = <ink_env::DefaultEnvironment as ink_env::Environment>::Timestamp;
}

pub trait Storage<E, D>
where
    E: Env,
{
    fn get(&self) -> &D;
    fn get_mut(&mut self) -> &mut D;
}

pub trait EnvAccess<E: Env> {
    fn caller() -> E::AccountId;
    fn transferred_balance() -> E::Balance;
}
