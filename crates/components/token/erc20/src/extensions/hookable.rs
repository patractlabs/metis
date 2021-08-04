use metis_lang::Env;

use crate::erc20::Result;

pub trait Impl<E>: crate::Impl<E>
where
    E: Env,
{
    /// Hook that is called before any transfer of tokens. This will call in hook
    fn before_token_transfer(
        &mut self,
        from: &E::AccountId,
        to: &E::AccountId,
        amount: &E::Balance,
    ) -> Result<()>;
}

impl<E: Env, I: Impl<E>> crate::Impl<E> for I {
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
        from: &E::AccountId,
        to: &E::AccountId,
        amount: &E::Balance,
    ) -> Result<()> {
        Impl::<E>::before_token_transfer(self, from, to, amount)
    }
}
