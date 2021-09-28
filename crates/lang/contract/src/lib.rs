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

pub use ink_env::hash::{
    CryptoHash,
    HashOutput,
};

pub trait Env: 'static {
    type AccountId: AccountId;
    type Balance: Balance;
    type BlockNumber: BlockNumber;
    type Hash: Hash;
    type Timestamp: Timestamp;
}

impl Env for ink_env::DefaultEnvironment {
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
    fn account_id() -> E::AccountId;
    fn balance() -> E::Balance;
    fn transfer(destination: E::AccountId, value: E::Balance) -> ink_env::Result<()>;
    fn block_timestamp() -> E::Timestamp;
    fn hash_bytes<H>(input: &[u8]) -> <H as HashOutput>::Type
    where
        H: CryptoHash;
}

pub trait FromAccountId<E>
where
    E: Env,
{
    /// Creates the contract instance from the account ID of the already instantiated contract.
    fn from_account_id(account_id: E::AccountId) -> Self;
}
