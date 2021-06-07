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
