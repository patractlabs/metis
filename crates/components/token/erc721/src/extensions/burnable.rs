use metis_lang::Env;

use crate::{
    Impl as ERC721,
    Result,
    TokenId,
};

/// @title ERC721 Burnable Token
/// @dev ERC721 Token that can be irreversibly burned (destroyed).
pub trait Impl<E>: ERC721<E>
where
    E: Env,
{
    /// @dev Burns `tokenId`. See {ERC721-_burn}.
    ///
    /// Requirements:
    ///
    /// - The caller must own `tokenId` or be an approved operator.
    fn burn(&mut self, token_id: &TokenId) -> Result<()> {
        let caller = &Self::caller();
        assert!(
            ERC721::_is_approved_or_owner(self, caller, token_id),
            "ERC721Burnable: caller is not owner nor approved"
        );
        ERC721::_burn(self, token_id)
    }
}

// No impl this for default
