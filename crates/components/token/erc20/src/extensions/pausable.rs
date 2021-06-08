pub use super::super::{
    module::Data,
    EventEmit,
};
use metis_lang::Env;

use crate::erc20::Result;

pub trait Impl<E>: crate::hookable::Impl<E> + metis_pausable::Impl<E>
where
    E: Env,
{}

impl<E: Env, I: Impl<E>> crate::hookable::Impl<E> for I
{
    /// Hook that is called before any transfer of tokens. This includes
    /// minting and burning.
    ///
    /// Calling conditions:
    ///
    /// - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// will be to transferred to `to`.
    /// - when `from` is zero, `amount` tokens will be minted for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens will be burned.
    /// - `from` and `to` are never both zero.
    fn before_token_transfer(
        &mut self,
        _from: &E::AccountId,
        _to: &E::AccountId,
        _amount: E::Balance,
    ) -> Result<()> {
        metis_pausable::Impl::<E>::ensure_not_paused(self);

        Ok(())
    }
}
