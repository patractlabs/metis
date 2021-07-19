//! @dev ERC1155 token with pausable token transfers, minting and burning.
//! Useful for scenarios such as preventing trades until the end of an evaluation
//! period, or having an emergency switch for freezing all token transfers in the
//! event of a large bug.

use crate::{
    Impl as ERC1155,
    Result,
    TokenId,
};
use ink_prelude::{
    vec::Vec,
};
use metis_lang::Env;

pub trait Impl<E>: ERC1155<E> + metis_pausable::Impl<E>
where
    E: Env,
{
    fn _before_token_transfer(
        &mut self,
        _operator: &E::AccountId,
        _from: &Option<&E::AccountId>,
        _to: &Option<&E::AccountId>,
        _ids: &Vec<TokenId>,
        _amounts: &Vec<E::Balance>,
        _data: &Vec<u8>,
    ) -> Result<()>;
}

impl<E, C> ERC1155<E> for C
where
    C: Impl<E>,
    E: Env,
{
    fn _before_token_transfer(
        &mut self,
        operator: &E::AccountId,
        from: &Option<&E::AccountId>,
        to: &Option<&E::AccountId>,
        ids: &Vec<TokenId>,
        amounts: &Vec<E::Balance>,
        data: &Vec<u8>,
    ) -> Result<()> {
        metis_pausable::Impl::<E>::ensure_not_paused(self);

        Impl::<E>::_before_token_transfer(self, operator, from, to, ids, amounts, data)?;

        Ok(())
    }
}
