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
use metis_erc20::{
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

// For Events
pub trait IERC20Event<E>
where
    E: Env,
{
    fn decode_transfer_event(
        event: &ink_env::test::EmittedEvent,
    ) -> (Option<E::AccountId>, Option<E::AccountId>, E::Balance);

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

pub struct Erc20BehaviorChecker<
    'a,
    Contract: Env + IERC20<Contract> + IERC20Event<Contract>,
> {
    erc20: &'a mut Contract,

    init_amount: Contract::Balance,
    default_account: Contract::AccountId,
    alice: Contract::AccountId,
    bob: Contract::AccountId,
}

impl<'a, Contract: Env + IERC20<Contract> + IERC20Event<Contract>>
    Erc20BehaviorChecker<'a, Contract>
{
    pub fn new(
        erc20: &'a mut Contract,
        init_amount: Contract::Balance,
        default_account: Contract::AccountId,
        alice: Contract::AccountId,
        bob: Contract::AccountId,
    ) -> Self {
        Self {
            erc20,
            init_amount,
            default_account,
            alice,
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
                value: b"Erc20::Transfer",
                prefix: b"",
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20::Transfer::from",
                value: &expected_from,
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20::Transfer::to",
                value: &expected_to,
            }),
            Contract::encoded_into_hash(&PrefixedValue {
                prefix: b"Erc20::Transfer::value",
                value: &expected_value,
            }),
        ];

        Contract::assert_topics(event, &expected_topics);
    }

    pub fn init_state_should_work(&self) {
        // for emit the init transfer
        let emitted_events = assert_emitted_event_len(1);
        self.assert_transfer_event(
            &emitted_events[0],
            None,
            Some(self.default_account.clone()),
            self.init_amount,
        );

        // for metadatas
        assert_eq!(
            String::from("MockErc20Token"),
            self.erc20.name(),
            "name should be default"
        );

        assert_eq!(
            String::from("MET"),
            self.erc20.symbol(),
            "symbol should be default"
        );
        assert_eq!(18, self.erc20.decimals(), "default decimals should be 18");

        // for init amount
        assert_eq!(
            self.init_amount,
            self.erc20.total_supply(),
            "total amount should be default"
        );

        assert_eq!(
            self.init_amount,
            self.erc20.balance_of(self.default_account.clone()),
            "default account balance_of should be default"
        );

        assert_eq!(
            Contract::Balance::from(0_u8),
            self.erc20.balance_of(self.bob.clone()),
            "others accounts balance should be 0"
        );
    }

    pub fn should_behave_like_erc20_transfer(& mut self){

    }
}
