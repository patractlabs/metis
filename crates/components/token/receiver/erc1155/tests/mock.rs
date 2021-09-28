#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
mod erc1155_receiver {
    use metis_lang::{
        import,
        metis,
    };

    use metis_ownable as ownable;
    use metis_receiver_erc1155 as receiver_erc1155;

    use metis_erc1155::TokenId;

    #[ink(storage)]
    #[import(receiver_erc1155, ownable)]
    pub struct Erc1155Receiver {
        ownable: ownable::Data<Erc1155Receiver>,
        receiver_erc1155: receiver_erc1155::Data,

        value: bool,
    }

    /// Emitted when contract is received the erc1155 `token_id` token is transferred from `from` to `to`.
    #[ink(event)]
    #[metis(receiver_erc1155)]
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
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self {
                value: init_value,
                ownable: ownable::Data::default(),
                receiver_erc1155: receiver_erc1155::Data::default(),
            };

            ownable::Impl::init(&mut instance);

            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(false)
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
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
            receiver_erc1155::Impl::on_erc1155_received(
                self, operator, from, id, value, data,
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut receiver = Erc1155Receiver::new(false);
            assert_eq!(
                receiver.on_erc1155_received(
                    AccountId::default(),
                    Some(AccountId::default()),
                    TokenId::default(),
                    Balance::default(),
                    vec![]
                ),
                metis_lang::selector_id!(on_erc1155_received)
            );
        }
    }
}
