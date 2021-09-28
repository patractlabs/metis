#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
mod erc721_receiver {
    use metis_lang::{
        import,
        metis,
    };

    use metis_ownable as ownable;
    use metis_receiver_erc721 as receiver_erc721;

    use metis_erc721::TokenId;

    #[ink(storage)]
    #[import(receiver_erc721, ownable)]
    pub struct Erc721Receiver {
        ownable: ownable::Data<Erc721Receiver>,
        receiver_erc721: receiver_erc721::Data,

        value: bool,
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
        pub fn new(init_value: bool) -> Self {
            let mut instance = Self {
                value: init_value,
                ownable: ownable::Data::default(),
                receiver_erc721: receiver_erc721::Data::default(),
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
        pub fn on_erc721_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            token_id: TokenId,
            data: Vec<u8>,
        ) -> [u8; 4] {
            receiver_erc721::Impl::on_erc721_received(
                self, operator, from, token_id, data,
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut receiver = Erc721Receiver::new(false);
            assert_eq!(
                receiver.on_erc721_received(
                    AccountId::default(),
                    AccountId::default(),
                    TokenId::default(),
                    vec![]
                ),
                metis_lang::selector_id!(on_erc721_received)
            );
        }
    }
}
