pub use super::{
    module::Data,
    EventEmit,
    Result,
};
use metis_lang::Env;

/// Extension of {ERC20} that allows token holders to destroy both their own
/// tokens and those that they have an allowance for, in a way that can be
/// recognized off-chain (via event analysis).
pub trait Impl<E>: super::hookable::Impl<E> where E: Env {}

impl<E: Env, I: Impl<E>> super::hookable::Impl<E> for I
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
    fn _before_token_transfer(
        &mut self,
        _from: &E::AccountId,
        _to: &E::AccountId,
        _amount: E::Balance,
    ) -> Result<()> {
        Ok(())
    }
}

// No impl this for default
// impl<E: Env, T: Storage<E, Data<E>> + EventEmit<E>> ImplBurnable<E> for T {}
