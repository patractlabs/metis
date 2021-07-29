#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod stub {
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    pub struct Receiver {}

    impl Receiver {
        #[ink(constructor)]
        pub fn default() -> Self {
            unimplemented!()
        }

        #[ink(message, payable)]
        pub fn on_call(
            &mut self,
            _operator: AccountId,
            _data: Vec<u8>,
        ) -> bool {
            unimplemented!()
        }
    }

    #[cfg(feature = "ink-as-dependency")]
    const _: () = {
        // TODO: use marco to gen code
        impl metis_lang::Env for Receiver {
            type AccountId =
                <::ink_env::DefaultEnvironment as ::ink_env::Environment>::AccountId;
            type Balance =
                <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Balance;
            type Hash = <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Hash;
            type Timestamp =
                <::ink_env::DefaultEnvironment as ::ink_env::Environment>::Timestamp;
            type BlockNumber =
                <::ink_env::DefaultEnvironment as ::ink_env::Environment>::BlockNumber;
        }

        impl<E> metis_lang::FromAccountId<E> for Receiver
        where
            E: metis_lang::Env,
        {
            /// Creates the contract instance from the account ID of the already instantiated contract.
            fn from_account_id(account_id: E::AccountId) -> Self {
                <Receiver as ::ink_env::call::FromAccountId<
                    ink_env::DefaultEnvironment,
                >>::from_account_id(account_id.into())
            }
        }
    };
}

pub use stub::{
    Receiver,
};
