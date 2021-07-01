use crate::{
    Impl as ERC721,
    Result,
    TokenId,
};

use metis_lang::Env;

pub trait Impl<E>: ERC721<E> + metis_pausable::Impl<E>
where
    E: Env,
{
    fn _base_url(&self) -> String;
    fn _before_token_transfer(
        &mut self,
        from: Option<E::AccountId>,
        to: Option<E::AccountId>,
        token_id: &TokenId,
    ) -> Result<()>;
}

impl<E, C> ERC721<E> for C
where
    C: Impl<E>,
    E: Env,
{
    /// @dev Hook that is called before any token transfer. This includes minting
    /// and burning.
    ///
    /// Calling conditions:
    ///
    /// - When `from` and `to` are both non-zero, ``from``'s `tokenId` will be
    /// transferred to `to`.
    /// - When `from` is zero, `tokenId` will be minted for `to`.
    /// - When `to` is zero, ``from``'s `tokenId` will be burned.
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _before_token_transfer(
        &mut self,
        _from: Option<E::AccountId>,
        _to: Option<E::AccountId>,
        _token_id: &TokenId,
    ) -> Result<()> {
        metis_pausable::Impl::<E>::ensure_not_paused(self);

        Ok(())
    }

    fn _base_url(&self) -> String {
        Impl::_base_url(self)
    }
}
