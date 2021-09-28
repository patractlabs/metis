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
    use metis_receiver_erc1155 as erc1155_receiver;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ::ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };

    #[ink(storage)]
    #[import(ownable, erc1155_receiver)]
    pub struct Erc1155Receiver {
        ownable: ownable::Data<Erc1155Receiver>,
        erc1155_receiver: erc1155_receiver::Data,

        is_receive: Lazy<bool>,
        receivers: StorageHashMap<(AccountId, TokenId), ()>,
    }

    /// Emitted when contract is received the erc1155 `token_id` token is transferred from `from` to `to`.
    #[ink(event)]
    #[metis(erc1155_receiver)]
    pub struct Erc1155Received {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub from: Option<AccountId>,
        pub ids: Vec<TokenId>,
        pub values: Vec<Balance>,
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

    impl Erc1155Receiver {
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
                erc1155_receiver: erc1155_receiver::Data::default(),

                is_receive: Lazy::new(false),
                receivers: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn set_receive_status(&mut self, is_receive: bool) {
            ownable::Impl::ensure_caller_is_owner(self);

            Lazy::set(&mut self.is_receive, is_receive);
        }

        #[ink(message)]
        pub fn add_accept_token(&mut self, contract: AccountId, id: TokenId) {
            ownable::Impl::ensure_caller_is_owner(self);

            self.receivers.insert((contract, id), ());
        }

        #[ink(message)]
        pub fn del_accept_token(&mut self, contract: AccountId, id: TokenId) {
            ownable::Impl::ensure_caller_is_owner(self);

            self.receivers.take(&(contract, id));
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
        pub fn on_erc1155_received(
            &mut self,
            operator: AccountId,
            from: Option<AccountId>,
            id: TokenId,
            value: Balance,
            data: Vec<u8>,
        ) -> [u8; 4] {
            assert!(
                *Lazy::get(&self.is_receive),
                "ERC721Receiver: Current not receive erc721 token"
            );

            if self.receivers.contains_key(&(Self::env().caller(), id)) {
                erc1155_receiver::Impl::on_erc1155_received(
                    self, operator, from, id, value, data,
                )
            } else {
                [0u8, 0u8, 0u8, 0u8]
            }
        }

        #[ink(message)]
        pub fn on_erc1155_batch_received(
            &mut self,
            operator: AccountId,
            from: Option<AccountId>,
            ids: Vec<TokenId>,
            values: Vec<Balance>,
            data: Vec<u8>,
        ) -> [u8; 4] {
            assert!(
                *Lazy::get(&self.is_receive),
                "ERC721Receiver: Current not receive erc721 token"
            );

            if self
                .receivers
                .contains_key(&(Self::env().caller(), TokenId::default()))
            {
                Self::env().emit_event(Erc1155Received {
                    operator,
                    from,
                    ids,
                    values,
                    data,
                });

                metis_lang::selector_id!(on_erc1155_batch_received)
            } else {
                [0u8, 0u8, 0u8, 0u8]
            }
        }
    }
}
