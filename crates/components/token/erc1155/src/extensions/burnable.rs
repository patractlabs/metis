use metis_lang::Env;

use crate::{
    Impl as ERC1155,
    Result,
    TokenId,
};
use ink_prelude::vec::Vec;

/// @dev Extension of {ERC1155} that allows token holders to destroy both their
/// own tokens and those that they have been approved to use.
pub trait Impl<E>: ERC1155<E>
where
    E: Env,
{
    /// @dev Burns `id` by `value`
    ///
    /// Requirements:
    ///
    /// - The caller must own `id` or be an approved operator.
    fn burn(
        &mut self,
        account: E::AccountId,
        id: TokenId,
        value: E::Balance,
    ) -> Result<()> {
        let caller = Self::caller();
        assert!(
            account == caller || self.is_approved_for_all(&account, &caller),
            "ERC1155: caller is not owner nor approved"
        );

        ERC1155::_burn(self, account, id, value)
    }

    /// @dev Burns Batch `ids` by `values`
    ///
    /// Requirements:
    ///
    /// - The caller must own `id` or be an approved operator.
    fn burn_batch(
        &mut self,
        account: E::AccountId,
        ids: Vec<TokenId>,
        values: Vec<E::Balance>,
    ) -> Result<()> {
        let caller = Self::caller();
        assert!(
            account == caller || self.is_approved_for_all(&account, &caller),
            "ERC1155: caller is not owner nor approved"
        );

        ERC1155::_burn_batch(self, account, ids, values)
    }
}

// No impl this for default
