#![cfg_attr(not(feature = "std"), no_std)]

#[metis_lang::contract]
pub mod erc777_contract {
    use super::super::behavior;
    pub use erc777::{
        Error,
        Result,
    };
    use metis_erc777 as erc777;
    use metis_lang::{
        import,
        metis,
    };

    #[ink(storage)]
    #[import(erc777)]
    pub struct Erc777 {
        erc777: erc777::Data<Erc777>,
    }

    // TODO: gen by marco with erc777 component
    #[cfg(not(feature = "ink-as-dependency"))]
    impl erc777::Impl<Erc777> for Erc777 {}

    type Event = <Erc777 as ink_lang::BaseEvent>::Type;

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[metis(erc777)]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    #[metis(erc777)]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: AccountId,
        pub value: Balance,
    }

    /// Indicate a send of `amount` of tokens from the `from` address to the `to`
    /// address by the `operator` address.
    /// 
    /// NOTE: This event MUST NOT be emitted outside of a send or an ERC-20 transfer process.
    #[ink(event)]
    #[metis(erc777)]
    pub struct Sent {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub from: AccountId,
        #[ink(topic)]
        pub to: AccountId,
        pub amount: Balance,
        pub data: Vec<u8>,
        pub operator_data: Vec<u8>,
    }

    /// Indicate the minting of `amount` of tokens to the `to` address by
    /// the `operator` address.
    ///
    /// NOTE: This event MUST NOT be emitted outside of a mint process.
    #[ink(event)]
    #[metis(erc777)]
    pub struct Minted {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub to: AccountId,
        pub amount: Balance,
        pub data: Vec<u8>,
        pub operator_data: Vec<u8>,
    }

    /// Indicate the burning of `amount` of tokens from the `from` address
    /// by the `operator` address.
    /// 
    /// NOTE: This event MUST NOT be emitted outside of a burn process.
    #[ink(event)]
    #[metis(erc777)]
    pub struct Burned {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub from: AccountId,
        pub amount: Balance,
        pub data: Vec<u8>,
        pub operator_data: Vec<u8>,
    }

    /// Indicates the authorization of `operator` as an operator for `holder`.
    /// 
    /// NOTE: This event MUST NOT be emitted outside of an operator authorization process.
    #[ink(event)]
    #[metis(erc777)]
    pub struct AuthorizedOperator {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub token_holder: AccountId,
    }

    /// Indicates the revocation of `operator` as an operator for `holder`.
    /// 
    /// NOTE: This event MUST NOT be emitted outside of an operator revocation process.
    #[ink(event)]
    #[metis(erc777)]
    pub struct RevokedOperator {
        #[ink(topic)]
        pub operator: AccountId,
        #[ink(topic)]
        pub token_holder: AccountId,
    }

    impl behavior::IERC20New<Erc777> for Erc777 {
        fn new_erc20(
            name: String,
            symbol: String,
            decimals: u8,
            initial_supply: Balance,
        ) -> Self {
            Self::new(name, symbol, decimals, initial_supply)
        }

        fn next_call_by(account: AccountId) {
            // Get contract address.
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            // Create call.
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4]));

            data.push_arg(&account.clone());

            // Push the new execution context to set from as caller.
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                account.clone(),
                callee,
                1000000,
                1000000,
                data,
            );
        }
    }

    impl behavior::IERC20Event<Erc777> for Erc777 {
        fn decode_transfer_event(
            event: &ink_env::test::EmittedEvent,
        ) -> (Option<AccountId>, Option<AccountId>, Balance) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                return (from, to, value)
            }
            panic!("encountered unexpected event kind: expected a Transfer event")
        }

        fn decode_approval_event(
            event: &ink_env::test::EmittedEvent,
        ) -> (AccountId, AccountId, Balance) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Approval(Approval {
                owner,
                spender,
                value,
            }) = decoded_event
            {
                return (owner, spender, value)
            }
            panic!("encountered unexpected event kind: expected a Transfer event")
        }

        fn assert_topics(
            event: &ink_env::test::EmittedEvent,
            expected_topics: &Vec<Hash>,
        ) {
            for (n, (actual_topic, expected_topic)) in
                event.topics.iter().zip(expected_topics).enumerate()
            {
                let topic = actual_topic
                    .decode::<Hash>()
                    .expect("encountered invalid topic encoding");
                assert_eq!(topic, *expected_topic, "encountered invalid topic at {}", n);
            }
        }
    }
    // impl
    impl Erc777 {
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            decimals: u8,
            initial_supply: Balance,
        ) -> Self {
            let mut instance = Self {
                erc777: erc777::Data::new(),
            };

            erc777::Impl::init(&mut instance, name, symbol, decimals, initial_supply);
            instance
        }

        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> String {
            erc777::Impl::name(self)
        }

        /// Returns the symbol of the token, usually a shorter version of the name.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            erc777::Impl::symbol(self)
        }

        /// Returns the number of decimals used to get its user representation.
        /// For example, if `decimals` equals `2`, a balance of `505` tokens should
        /// be displayed to a user as `5,05` (`505 / 10 ** 2`).
        ///
        /// Tokens usually opt for a value of 18, imitating the relationship between
        /// Ether and Wei in ETH. This is the value `ERC20` uses, unless this function is
        /// overridden;
        ///
        /// NOTE: This information is only used for _display_ purposes: it in
        /// no way affects any of the arithmetic of the contract
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            erc777::Impl::decimals(self)
        }

        /// Returns the smallest part of the token that is not divisible. This
        /// means all token operations (creation, movement and destruction) must have
        /// amounts that are a multiple of this number.
        ///
        /// For most token contracts, this value will equal 1.
        #[ink(message)]
        pub fn granularity(&self) -> Balance {
            erc777::Impl::granularity(self)
        }

        /// Returns the amount of tokens in existence.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            erc777::Impl::total_supply(self)
        }

        /// Returns the amount of tokens owned by `account`.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            erc777::Impl::balance_of(self, &owner)
        }

        /// Moves `amount` tokens from the caller's account to `recipient`.
        ///
        /// If send or receive hooks are registered for the caller and `recipient`,
        /// the corresponding functions will be called with `data` and empty
        /// `operator_data`. See `erc777_sender` and `erc777_recipient`.
        ///
        /// Emits a `Sent` event.
        ///
        /// Requirements
        ///
        /// - the caller must have at least `amount` tokens.
        /// - `recipient` cannot be the zero address.
        /// - if `recipient` is a contract, it must implement the `erc777_recipient` interface.
        #[ink(message)]
        pub fn send(
            &mut self,
            recipient: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<()> {
            erc777::Impl::send(self, recipient, amount, data)
        }

        /// Moves `amount` tokens from the caller's account to `recipient`.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// Emits a `Transfer` event.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc777::Impl::transfer(self, &to, value)
        }

        /// Destroys `amount` tokens from the caller's account, reducing the
        /// total supply.
        ///
        /// If a send hook is registered for the caller, the corresponding function
        /// will be called with `data` and empty `operator_data`. See `erc777_sender`.
        ///
        /// Emits a `Burned` event.
        ///
        /// Requirements
        ///
        /// - the caller must have at least `amount` tokens.
        #[ink(message)]
        pub fn burn(&mut self, amount: Balance, data: Vec<u8>) -> Result<()> {
            erc777::Impl::burn(self, amount, data)
        }

        /// Returns true if an account is an operator of `token_holder`.
        /// Operators can send and burn tokens on behalf of their owners. All
        /// accounts are their own operator.
        ///
        /// See `operator_send` and `operator_burn`.
        #[ink(message)]
        pub fn is_operator_for(
            &self,
            operator: AccountId,
            token_holder: AccountId,
        ) -> bool {
            erc777::Impl::is_operator_for(self, operator, token_holder)
        }

        /// Make an account an operator of the caller.
        ///
        /// See `is_operator_for`.
        ///
        /// Emits an `AuthorizedOperator` event.
        ///
        /// Requirements
        ///
        /// - `operator` cannot be calling address.
        #[ink(message)]
        pub fn authorize_operator(&mut self, operator: AccountId) {
            erc777::Impl::authorize_operator(self, operator)
        }

        /// Revoke an account's operator status for the caller.
        ///
        /// See `is_operator_for` and `default_operators`.
        ///
        /// Emits a `RevokedOperator` event.
        ///
        /// Requirements
        ///
        /// - `operator` cannot be calling address.
        #[ink(message)]
        pub fn revoke_operator(&mut self, operator: AccountId) {
            erc777::Impl::revoke_operator(self, operator)
        }

        /// Returns the list of default operators. These accounts are operators
        /// for all token holders, even if `authorize_operator` was never called on
        /// them.
        ///
        /// This list is immutable, but individual holders may revoke these via
        /// `revoke_operator`, in which case `is_operator_for` will return false.
        #[ink(message)]
        pub fn default_operators(&self) -> Vec<AccountId> {
            erc777::Impl::default_operators(self)
        }

        /// Moves `amount` tokens from `sender` to `recipient`. The caller must
        /// be an operator of `sender`.
        ///
        /// If send or receive hooks are registered for `sender` and `recipient`,
        /// the corresponding functions will be called with `data` and
        /// `operator_data`. See `erc777_sender` and `erc777_recipient`.
        ///
        /// Emits a `Sent` event.
        ///
        /// Requirements
        ///
        /// - `sender` cannot be the zero address.
        /// - `sender` must have at least `amount` tokens.
        /// - the caller must be an operator for `sender`.
        /// - `recipient` cannot be the zero address.
        /// - if `recipient` is a contract, it must implement the `erc777_recipient` interface.
        #[ink(message)]
        pub fn operator_send(
            &mut self,
            sender: AccountId,
            recipient: AccountId,
            amount: Balance,
            data: Vec<u8>,
            operator_data: Vec<u8>,
        ) -> Result<()> {
            erc777::Impl::operator_send(
                self,
                sender,
                recipient,
                amount,
                data,
                operator_data,
            )
        }

        /// Destroys `amount` tokens from `account`, reducing the total supply.
        /// The caller must be an operator of `account`.
        ///
        /// If a send hook is registered for `account`, the corresponding function
        /// will be called with `data` and `operator_data`. See `erc777_sender`.
        ///
        /// Emits a `Burned` event.
        ///
        /// Requirements
        ///
        /// - `account` cannot be the zero address.
        /// - `account` must have at least `amount` tokens.
        /// - the caller must be an operator for `account`.
        #[ink(message)]
        pub fn operator_burn(
            &mut self,
            account: AccountId,
            amount: Balance,
            data: Vec<u8>,
            operator_data: Vec<u8>,
        ) -> Result<()> {
            erc777::Impl::operator_burn(self, account, amount, data, operator_data)
        }

        /// Returns the remaining number of tokens that `spender` will be
        /// allowed to spend on behalf of `owner` through `transfer_from`. This is
        /// zero by default.
        ///
        /// This value changes when `approve` or `transfer_from` are called.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            erc777::Impl::allowance(self, &owner, &spender)
        }

        /// Sets `amount` as the allowance of `spender` over the caller's tokens.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// IMPORTANT: Beware that changing an allowance with this method brings the risk
        /// that someone may use both the old and the new allowance by unfortunate
        /// transaction ordering. One possible solution to mitigate this race
        /// condition is to first reduce the spender's allowance to 0 and set the
        /// desired value afterwards:
        /// <https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729>
        ///
        /// Emits an `Approval` event.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            erc777::Impl::approve(self, &spender, value)
        }

        /// Moves `amount` tokens from `sender` to `recipient` using the
        /// allowance mechanism. `amount` is then deducted from the caller's
        /// allowance.
        ///
        /// Returns a boolean value indicating whether the operation succeeded.
        ///
        /// Emits a `Transfer` event.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc777::Impl::transfer_from(self, &from, &to, value)
        }

        /// Creates `amount` tokens and assigns them to `account`, increasing
        /// the total supply.
        ///
        /// If a send hook is registered for `account`, the corresponding function
        /// will be called with `operator`, `data` and `operator_data`.
        ///
        /// See `erc777_sender` and `erc777_recipient`.
        ///
        /// Emits `Minted` and `Transfer` events.
        ///
        /// Requirements
        ///
        /// - `account` cannot be the zero address.
        /// - if `account` is a contract, it must implement the `erc777_recipient` interface.
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            erc777::Impl::_mint(self, to, value, Vec::default(), Vec::default())
        }

        /// for test
        #[ink(message)]
        pub fn transfer_internal(
            &mut self,
            spender: AccountId,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc777::Impl::_move(
                self,
                &spender,
                &from,
                &to,
                &value,
                &Vec::default(),
                &Vec::default(),
            )
        }

        /// for test
        #[ink(message)]
        pub fn approve_internal(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            value: Balance,
        ) -> Result<()> {
            erc777::Impl::_approve(self, &owner, &spender, value)
        }
    }
}
