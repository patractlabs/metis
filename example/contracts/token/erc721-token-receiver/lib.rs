#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod contract {
    use ink_prelude::vec::Vec;
    use metis_erc721::TokenId;
    use metis_lang::{
        import,
        metis,
    };

    use metis_ownable as ownable;
    use metis_receiver_erc721 as receiver_erc721;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ::ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };

    #[ink(storage)]
    #[import(ownable, receiver_erc721)]
    pub struct Erc721Receiver {
        ownable: ownable::Data<Erc721Receiver>,
        receiver_erc721: receiver_erc721::Data,

        is_receive: Lazy<bool>,
        erc721_receive: StorageHashMap<AccountId, ()>,
    }

    /// Emitted when contract is received the erc721 `token_id` token is transferred from `from` to `to`.
    #[ink(event)]
    #[metis(receiver_erc721)]
    pub struct Erc721Received {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub from: AccountId,
        #[ink(topic)]
        pub token_id: TokenId,
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

    impl Erc721Receiver {
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
                erc721_receive: StorageHashMap::new(),
                receiver_erc721: receiver_erc721::Data::default(),
            }
        }

        #[ink(message)]
        pub fn set_receive_status(&mut self, is_receive: bool) {
            ownable::Impl::ensure_caller_is_owner(self);

            Lazy::set(&mut self.is_receive, is_receive);
        }

        #[ink(message)]
        pub fn add_accept_token(&mut self, contract: AccountId) {
            ownable::Impl::ensure_caller_is_owner(self);

            self.erc721_receive.insert(contract, ());
        }

        #[ink(message)]
        pub fn del_accept_token(&mut self, contract: AccountId) {
            ownable::Impl::ensure_caller_is_owner(self);

            self.erc721_receive.take(&contract);
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

        #[ink(message)]
        pub fn on_erc721_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            token_id: TokenId,
            data: Vec<u8>,
        ) -> [u8; 4] {
            assert!(
                *Lazy::get(&self.is_receive),
                "ERC721Receiver: Current not receive erc721 token"
            );

            if self.erc721_receive.contains_key(&Self::env().caller()) {
                receiver_erc721::Impl::on_erc721_received(
                    self, operator, from, token_id, data,
                )
            } else {
                [0u8, 0u8, 0u8, 0u8]
            }
        }
    }
}
