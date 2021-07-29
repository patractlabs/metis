#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod contract {
    use ink_prelude::vec::Vec;
    use metis_lang::{
        import,
        metis,
    };
    use metis_ownable as ownable;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ::ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };

    #[ink(storage)]
    #[import(ownable)]
    pub struct Receiver {
        ownable: ownable::Data<Receiver>,

        is_receive: Lazy<bool>,
        receive: StorageHashMap<AccountId, ()>,
    }

    /// Emitted when contract is received the erc721 `token_id` token is transferred from `from` to `to`.
    #[ink(event)]
    pub struct CallReceived {
        #[ink(topic)]
        pub operator: AccountId,
        pub value: Balance,
        pub data: Vec<u8>,
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

    impl Receiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut res = Self::default();

            ownable::Impl::init(&mut res);

            res
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                ownable: ownable::Data::new(),

                is_receive: Lazy::new(false),
                receive: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn set_receive_status(&mut self, is_receive: bool) {
            ownable::Impl::ensure_caller_is_owner(self);

            Lazy::set(&mut self.is_receive, is_receive);
        }

        #[ink(message)]
        pub fn add_accept_caller(&mut self, contract: AccountId) {
            ownable::Impl::ensure_caller_is_owner(self);

            self.receive.insert(contract, ());
        }

        #[ink(message)]
        pub fn del_accept_caller(&mut self, contract: AccountId) {
            ownable::Impl::ensure_caller_is_owner(self);

            self.receive.take(&contract);
        }

        #[ink(message)]
        pub fn get_ownership(&self) -> Option<AccountId> {
            *ownable::Impl::owner(self)
        }

        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            ownable::Impl::renounce_ownership(self)
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            ownable::Impl::transfer_ownership(self, &new_owner)
        }

        #[ink(message, payable)]
        pub fn on_call(
            &mut self,
            operator: AccountId,
            data: Vec<u8>,
        ) -> bool {
            //assert!(
            //    Lazy::get(&self.is_receive),
            //    "ERC721Receiver: Current not receive erc721 token"
            //);

            let value = Self::env().transferred_balance();

            //if self.receive.contains_key(&Self::env().caller()) {
                Self::env().emit_event(CallReceived {
                    operator,
                    value,
                    data,
                });

                true
            //} else {
            //    false
            //}
        }
    }
}
