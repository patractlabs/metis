#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;

pub enum PatraEnvironment {}

impl Environment for PatraEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = u32;

    type ChainExtension = ();
    type RentFraction = <ink_env::DefaultEnvironment as Environment>::RentFraction;
}
