#![cfg_attr(not(feature = "std"), no_std)]

use crate::utils::event::*;
use ink_env::{
    hash::{
        Blake2x256,
        CryptoHash,
        HashOutput,
    },
    Clear,
};
use ink_lang as ink;
use ink_prelude::string::String;
use metis_erc777::{
    Error,
    Impl,
    Result,
};
pub use metis_lang::Env;

// TODO: current ink! trait_definition is in developing, so this just use for test
/// Trait implemented by all ERC-20 respecting smart contracts.
pub trait IERC20<E>
where
    E: Env,
{
    /// Returns the name of the token.
    fn name(&self) -> String;

    /// Returns the symbol of the token, usually a shorter version of the name.
    fn symbol(&self) -> String;

    /// Returns the number of decimals used to get its user representation.
    /// For example, if `decimals` equals `2`, a balance of `505` tokens should
    /// be displayed to a user as `5,05` (`505 / 10 ** 2`).
    ///
    /// Tokens usually opt for a value of 18, imitating the relationship between
    /// Ether and Wei in ETH. This is the value {ERC20} uses, unless this function is
    /// overridden;
    ///
    /// NOTE: This information is only used for _display_ purposes: it in
    /// no way affects any of the arithmetic of the contract
    fn decimals(&self) -> u8;

    /// Returns the total token supply.
    fn total_supply(&self) -> E::Balance;

    /// Returns the account balance for the specified `owner`.
    fn balance_of(&self, owner: E::AccountId) -> E::Balance;

    /// Returns the remaining number of tokens that `spender` will be
    /// allowed to spend on behalf of `owner` through {transferFrom}. This is
    /// zero by default.
    ///
    /// This value changes when {approve} or {transferFrom} are called.
    fn allowance(&self, owner: E::AccountId, spender: E::AccountId) -> E::Balance;

    /// Moves `amount` tokens from the caller's account to `recipient`.
    ///
    /// Returns a boolean value indicating whether the operation succeeded.
    ///
    /// Emits a {Transfer} event.
    fn transfer(&mut self, to: E::AccountId, value: E::Balance) -> Result<()>;

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
    /// Emits an {Approval} event.
    fn approve(&mut self, spender: E::AccountId, value: E::Balance) -> Result<()>;

    /// Moves `amount` tokens from `sender` to `recipient` using the
    /// allowance mechanism. `amount` is then deducted from the caller's
    /// allowance.
    ///
    /// Returns a boolean value indicating whether the operation succeeded.
    ///
    /// Emits a {Transfer} event.
    fn transfer_from(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        value: E::Balance,
    ) -> Result<()>;
}

pub trait IERC20New<E>
where
    E: Env,
{
    fn new_erc20(
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: E::Balance,
    ) -> Self;
    fn next_call_by(account: E::AccountId);
}

// For Events
pub trait IERC20Event<E>: IERC20New<E>
where
    E: Env,
{
    fn decode_transfer_event(
        event: &ink_env::test::EmittedEvent,
    ) -> (Option<E::AccountId>, Option<E::AccountId>, E::Balance);

    fn decode_approval_event(
        event: &ink_env::test::EmittedEvent,
    ) -> (E::AccountId, E::AccountId, E::Balance);

    fn assert_topics(event: &ink_env::test::EmittedEvent, expected_topics: &Vec<E::Hash>);

    fn encoded_into_hash<T>(entity: &T) -> E::Hash
    where
        T: scale::Encode,
    {
        let mut result = E::Hash::clear();
        let len_result = result.as_ref().len();
        let encoded = entity.encode();
        let len_encoded = encoded.len();
        if len_encoded <= len_result {
            result.as_mut()[..len_encoded].copy_from_slice(&encoded);
            return result
        }
        let mut hash_output = <<Blake2x256 as HashOutput>::Type as Default>::default();
        <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
        let copy_len = core::cmp::min(hash_output.len(), len_result);
        result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
        result
    }
}

impl<E, I> IERC20<E> for I
where
    E: Env,
    I: Impl<E> + ink::BaseEvent,
    <I as ink::BaseEvent>::Type: scale::Decode,
{
    fn name(&self) -> String {
        Impl::name(self)
    }

    fn symbol(&self) -> String {
        Impl::symbol(self)
    }

    fn decimals(&self) -> u8 {
        Impl::decimals(self)
    }

    fn total_supply(&self) -> E::Balance {
        Impl::total_supply(self)
    }

    fn balance_of(&self, owner: E::AccountId) -> E::Balance {
        Impl::balance_of(self, &owner)
    }

    fn allowance(&self, owner: E::AccountId, spender: E::AccountId) -> E::Balance {
        Impl::allowance(self, &owner, &spender)
    }

    fn transfer(&mut self, to: E::AccountId, value: E::Balance) -> Result<()> {
        Impl::transfer(self, &to, value)
    }

    fn approve(&mut self, spender: E::AccountId, value: E::Balance) -> Result<()> {
        Impl::approve(self, &spender, value)
    }

    fn transfer_from(
        &mut self,
        from: E::AccountId,
        to: E::AccountId,
        value: E::Balance,
    ) -> Result<()> {
        Impl::transfer_from(self, &from, &to, value)
    }
}

pub struct Erc20BehaviorChecker<Contract: Env + IERC20<Contract> + IERC20Event<Contract>>
{
    init_amount: Contract::Balance,
    default_account: Contract::AccountId,
    zero_account: Contract::AccountId,
    bob: Contract::AccountId,
}

impl<Contract: Env + IERC20<Contract> + IERC20Event<Contract>>
    Erc20BehaviorChecker<Contract>
{
    pub fn new(
        init_amount: Contract::Balance,
        default_account: Contract::AccountId,
        zero_account: Contract::AccountId,
        bob: Contract::AccountId,
    ) -> Self {
        Self {
            init_amount,
            default_account,
            zero_account,
            bob,
        }
    }

    fn assert_transfer_event(
        &self,
        event: &ink_env::test::EmittedEvent,
        expected_from: Option<Contract::AccountId>,
        expected_to: Option<Contract::AccountId>,
        expected_value: Contract::Balance,
    ) {
        let (from, to, value) = Contract::decode_transfer_event(event);
        assert_eq!(from, expected_from, "encountered invalid Transfer.from");
        assert_eq!(to, expected_to, "encountered invalid Transfer.to");
        assert_eq!(value, expected_value, "encountered invalid Trasfer.value");

        let expected_topics = vec![
            Contract::encoded_into_hash(&PrefixedValue {
                value: b"Erc777::Transfer",
                prefix: b"",
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc777::Transfer::from",
                value: &expected_from,
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc777::Transfer::to",
                value: &expected_to,
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc777::Transfer::value",
                value: &expected_value,
            }),
        ];

        Contract::assert_topics(event, &expected_topics);
    }

    fn assert_approval_event(
        &self,
        event: &ink_env::test::EmittedEvent,
        expected_owner: &Contract::AccountId,
        expected_spender: &Contract::AccountId,
        expected_value: &Contract::Balance,
    ) {
        let (owner, spender, value) = Contract::decode_approval_event(event);
        assert_eq!(&owner, expected_owner, "encountered invalid Approval.owner");
        assert_eq!(
            &spender, expected_spender,
            "encountered invalid Approval.spender"
        );
        assert_eq!(&value, expected_value, "encountered invalid Approval.value");

        let expected_topics = vec![
            Contract::encoded_into_hash(&PrefixedValue {
                value: b"Erc777::Approval",
                prefix: b"",
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc777::Approval::owner",
                value: expected_owner,
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc777::Approval::spender",
                value: expected_spender,
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc777::Approval::value",
                value: expected_value,
            }),
        ];

        Contract::assert_topics(event, &expected_topics);
    }

    pub fn should_erc20_behavior_work<T>(
        init_amount: Contract::Balance,
        default_account: Contract::AccountId,
        zero_account: Contract::AccountId,
        bob: Contract::AccountId,
        transfer_fn: T,
    ) where
        T: Fn(
            &mut Contract,
            &Contract::AccountId,
            &Contract::AccountId,
            Contract::Balance,
        ) -> Result<()>,
    {
        let mut checker = Erc20BehaviorChecker::new(
            init_amount.clone(),
            default_account.clone(),
            zero_account.clone(),
            bob.clone(),
        );

        checker.init_state_should_work();
        checker.should_behave_like_erc20_transfer(
            default_account,
            bob,
            init_amount,
            transfer_fn,
        );
        checker.should_behave_like_erc20_transfer_from();
        checker.should_behave_like_erc20_approve_should_ok();
    }

    fn init_state_should_work(&self) {
        Contract::next_call_by(self.default_account.clone());
        let erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            self.init_amount,
        );

        let events = get_emitted_events();

        // for emit the init transfer
        self.assert_transfer_event(
            &events[0],
            None,
            Some(self.default_account.clone()),
            self.init_amount,
        );

        // for metadatas
        assert_eq!(
            String::from("MockErc20Token"),
            erc20.name(),
            "name should be default"
        );

        assert_eq!(
            String::from("MET"),
            erc20.symbol(),
            "symbol should be default"
        );
        assert_eq!(18, erc20.decimals(), "default decimals should be 18");

        // for init amount
        assert_eq!(
            self.init_amount,
            erc20.total_supply(),
            "total amount should be default"
        );

        assert_eq!(
            self.init_amount,
            erc20.balance_of(self.default_account.clone()),
            "default account balance_of should be default"
        );

        assert_eq!(
            Contract::Balance::from(0_u8),
            erc20.balance_of(self.bob.clone()),
            "others accounts balance should be 0"
        );
    }

    fn should_behave_like_erc20_transfer<T>(
        &mut self,
        from: Contract::AccountId,
        to: Contract::AccountId,
        balance: Contract::Balance,
        transfer: T,
    ) where
        T: Fn(
            &mut Contract,
            &Contract::AccountId,
            &Contract::AccountId,
            Contract::Balance,
        ) -> Result<()>,
    {
        // when the recipient is not the zero address
        // when the sender does not have enough balance
        let large_amt = balance + Contract::Balance::from(1_u8);

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            balance,
        );

        assert_eq!(
            transfer(&mut erc20, &from, &to, large_amt),
            Err(Error::InsufficientBalance),
            "when the sender does not have enough balance"
        );

        // when the sender transfers all balance
        assert_eq!(
            transfer(&mut erc20, &from, &to, balance),
            Ok(()),
            "when the sender transfers all balance"
        );

        assert_eq!(
            erc20.balance_of(from.clone()),
            Contract::Balance::from(0_u8),
            "from amount should be 0",
        );

        assert_eq!(
            erc20.balance_of(to.clone()),
            balance,
            "to amount should be balance"
        );

        // emits a transfer event
        self.assert_transfer_event(
            &get_last_emitted_event(),
            Some(from.clone()),
            Some(to.clone()),
            balance,
        );

        // when the sender transfers zero tokens
        assert_eq!(
            transfer(&mut erc20, &to, &from, Contract::Balance::from(0_u8)),
            Ok(()),
            "when the sender transfers all balance"
        );

        assert_eq!(
            erc20.balance_of(from.clone()),
            Contract::Balance::from(0_u8),
            "from amount should be 0 as no changed",
        );

        assert_eq!(
            erc20.balance_of(to.clone()),
            balance,
            "to amount should be balance as no changed"
        );

        // emits a transfer event
        self.assert_transfer_event(
            &get_last_emitted_event(),
            Some(to.clone()),
            Some(from.clone()),
            Contract::Balance::from(0_u8),
        );
    }

    fn should_behave_like_erc20_transfer_from(&mut self) {
        self._transfer_from_request_amount_should_ok();
        self._transfer_from_approved_no_enough_should_ok();
        self._transfer_from_to_zero_account_should_err();
        self._transfer_from_from_is_zero_account_should_err();
    }

    fn _transfer_from_request_amount_should_ok(&mut self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // when the spender has enough approved balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount.clone()),
            Ok(()),
            "approve from spender to to should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount);

        // now allowance from to is amount
        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount,
            "the spender allowance"
        );

        // call transfer_from by to
        Contract::next_call_by(to.clone());
        assert_eq!(
            erc20.transfer_from(from.clone(), to.clone(), amount),
            Ok(()),
            "transfers the requested amount should ok"
        );

        let emitted_events = get_emitted_events();

        self.assert_transfer_event(
            &emitted_events[emitted_events.len() - 1],
            Some(from.clone()),
            Some(to.clone()),
            amount,
        );

        self.assert_approval_event(
            &emitted_events[emitted_events.len() - 3],
            &from,
            &to,
            &Contract::Balance::from(0_u8),
        );

        // check ok
        assert_eq!(
            erc20.balance_of(from.clone()),
            Contract::Balance::from(0_u8),
            "from amount should be 0 as no changed",
        );

        assert_eq!(
            erc20.balance_of(to.clone()),
            self.init_amount,
            "to amount should be balance as no changed"
        );

        // decreases the spender allowance
        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            Contract::Balance::from(0_u8),
            "decreases the spender allowance should ok"
        );
    }

    fn _transfer_from_approved_no_enough_should_ok(&mut self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        let amount_less = amount.clone() - Contract::Balance::from(1_u8);

        // when the spender has enough approved balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount_less.clone()),
            Ok(()),
            "approve from spender to to should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount_less);

        // call transfer_from by to should error
        Contract::next_call_by(to.clone());
        assert_eq!(
            erc20.transfer_from(from.clone(), to.clone(), amount),
            Err(Error::InsufficientAllowance),
            "the token owner has enough balance should error"
        );

        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.transfer_from(
                from.clone(),
                to.clone(),
                amount.clone() + Contract::Balance::from(1_u8)
            ),
            Err(Error::InsufficientAllowance),
            "the token owner does not have enough balance should error"
        );
    }

    fn _transfer_from_to_zero_account_should_err(&mut self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // when the spender has enough approved balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount.clone()),
            Ok(()),
            "approve from spender to to should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount);

        // now allowance from to is amount
        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount,
            "the spender allowance"
        );

        // call transfer_from by to
        Contract::next_call_by(to.clone());
        assert_eq!(
            erc20.transfer_from(from.clone(), self.zero_account.clone(), amount),
            Err(Error::AccountIsZero),
            "transfer to the zero address should error"
        );

        // check ok
        assert_eq!(
            erc20.balance_of(from.clone()),
            amount,
            "from amount should be 0 as no changed",
        );

        assert_eq!(
            erc20.balance_of(to.clone()),
            Contract::Balance::from(0_u8),
            "to amount should be balance as no changed"
        );

        // decreases the spender allowance
        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount,
            "decreases the spender allowance should ok"
        );
    }

    fn _transfer_from_from_is_zero_account_should_err(&mut self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // when the spender has enough approved balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount.clone()),
            Ok(()),
            "approve from spender to to should ok"
        );

        // call transfer_from by to
        Contract::next_call_by(to.clone());
        assert_eq!(
            erc20.transfer_from(self.zero_account.clone(), to.clone(), amount),
            Err(Error::InsufficientAllowance),
            "transfer to the zero address should error"
        );

        // check ok
        assert_eq!(
            erc20.balance_of(from.clone()),
            amount,
            "from amount should be 0 as no changed",
        );

        assert_eq!(
            erc20.balance_of(to.clone()),
            Contract::Balance::from(0_u8),
            "to amount should be balance as no changed"
        );

        // decreases the spender allowance
        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount,
            "decreases the spender allowance should ok"
        );
    }

    fn should_behave_like_erc20_approve_should_ok(&self) {
        self._approve_have_enough_balance_should_ok();
        self._approve_when_no_approved_before();
        self._approve_when_had_approved_should_replace();

        self._approve_sender_no_enough();
        self._approve_sender_no_enough_when_no_approved_before();
        self._approve_sender_no_enough_when_had_approved_should_replace();

        self._approve_to_zero_account_should_error();
    }

    fn _approve_have_enough_balance_should_ok(&self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // when the spender has enough approved balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount.clone()),
            Ok(()),
            "approve from spender to to should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount);
    }

    fn _approve_when_no_approved_before(&self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // when the spender has enough approved balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount.clone()),
            Ok(()),
            "approve from spender to to should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount);

        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount.clone(),
            "the spender allowance should ok"
        );
    }

    fn _approve_when_had_approved_should_replace(&self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();
        let amount_1 = Contract::Balance::from(1_u8);

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // approve 1 balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount_1.clone()),
            Ok(()),
            "approve from spender to to with 1 should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount_1);

        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount_1.clone(),
            "the spender allowance should ok"
        );

        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount.clone()),
            Ok(()),
            "approve from spender to to with all amount should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount);

        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount.clone(),
            "the spender allowance should ok"
        );
    }

    fn _approve_sender_no_enough(&self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();
        let amount_add_1 = amount + Contract::Balance::from(1_u8);

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // approve 1 balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount_add_1.clone()),
            Ok(()),
            "approve from spender to to with amount + 1 should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount_add_1);
    }

    fn _approve_sender_no_enough_when_no_approved_before(&self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();
        let amount_add_1 = amount + Contract::Balance::from(1_u8);

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // when the spender has enough approved balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount_add_1.clone()),
            Ok(()),
            "approve from spender to to should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount_add_1);

        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount_add_1.clone(),
            "the spender allowance should ok"
        );
    }

    fn _approve_sender_no_enough_when_had_approved_should_replace(&self) {
        let from = self.default_account.clone();
        let to = self.bob.clone();
        let amount = self.init_amount.clone();
        let amount_1 = Contract::Balance::from(1_u8);
        let amount_add_1 = amount + Contract::Balance::from(1_u8);

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // approve 1 balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount_1.clone()),
            Ok(()),
            "approve from spender to to with 1 should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount_1);

        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount_1.clone(),
            "the spender allowance should ok"
        );

        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount_add_1.clone()),
            Ok(()),
            "approve from spender to to with all amount_add_1 should ok"
        );

        self.assert_approval_event(&get_last_emitted_event(), &from, &to, &amount_add_1);

        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            amount_add_1.clone(),
            "the spender allowance should ok"
        );
    }

    fn _approve_to_zero_account_should_error(&self) {
        let from = self.default_account.clone();
        let to = self.zero_account.clone();
        let amount = self.init_amount.clone();

        Contract::next_call_by(from.clone());
        let mut erc20 = Contract::new_erc20(
            String::from("MockErc20Token"),
            String::from("MET"),
            18_u8,
            amount.clone(),
        );

        // approve 1 balance
        Contract::next_call_by(from.clone());
        assert_eq!(
            erc20.approve(to.clone(), amount.clone()),
            Err(Error::AccountIsZero),
            "approve from spender to zero with amount should error"
        );

        assert_eq!(
            erc20.allowance(from.clone(), to.clone()),
            Contract::Balance::from(0_u8),
            "the spender allowance should be 0"
        );
    }
}
