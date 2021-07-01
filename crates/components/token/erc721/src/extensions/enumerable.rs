//! @dev This implements an optional extension of {ERC721} defined in the EIP that adds
//! enumerability of all the token ids in the contract as well as all token ids owned by each
//! account.

use crate::{
    Impl as ERC721,
    Result,
    TokenId,
};

use metis_lang::{
    Env,
    Storage,
};

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::{
        HashMap as StorageHashMap,
        Vec as StorageVec,
    },
    traits::SpreadLayout,
};

#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data<E: Env> {
    /// Mapping from owner to list of owned token IDs
    owned_tokens: StorageHashMap<(E::AccountId, E::Balance), TokenId>,

    /// Mapping from token ID to index of the owner tokens list
    owned_tokens_index: StorageHashMap<TokenId, E::Balance>,

    /// Array with all token ids, used for enumeration
    all_tokens: StorageVec<TokenId>,

    /// Mapping from token id to position in the allTokens array
    all_tokens_index: StorageHashMap<TokenId, u32>,
}

impl<E> Default for Data<E>
where
    E: Env,
{
    fn default() -> Self {
        Self {
            owned_tokens: StorageHashMap::default(),
            owned_tokens_index: StorageHashMap::default(),
            all_tokens: StorageVec::default(),
            all_tokens_index: StorageHashMap::default(),
        }
    }
}

pub trait _Impl<E>: ERC721<E> + Storage<E, Data<E>>
where
    E: Env,
{
    /// @dev Private function to add a token to this extension's ownership-tracking data structures.
    /// @param to address representing the new owner of the given token ID
    /// @param token_id uint256 ID of the token to be added to the tokens list of the given address
    fn _add_token_to_owner_enumeration(&mut self, to: E::AccountId, token_id: &TokenId) {
        let length = ERC721::balance_of(self, &to);

        Storage::<E, Data<E>>::get_mut(self)
            .owned_tokens
            .insert((to, length.clone()), token_id.clone());
        Storage::<E, Data<E>>::get_mut(self)
            .owned_tokens_index
            .insert(token_id.clone(), length);
    }

    /// @dev Private function to add a token to this extension's token tracking data structures.
    /// @param token_id uint256 ID of the token to be added to the tokens list
    fn _add_token_to_all_tokens_enumeration(&mut self, token_id: TokenId) {
        let len = Storage::<E, Data<E>>::get_mut(self).all_tokens.len();

        Storage::<E, Data<E>>::get_mut(self)
            .all_tokens_index
            .insert(token_id.clone(), len);
        Storage::<E, Data<E>>::get_mut(self)
            .all_tokens
            .push(token_id);
    }

    /// @dev Private function to remove a token from this extension's ownership-tracking data structures. Note that
    /// while the token is not assigned a new owner, the `owned_tokens_index` mapping is _not_ updated: this allows for
    /// gas optimizations e.g. when performing a transfer operation (avoiding double writes).
    /// This has O(1) time complexity, but alters the order of the owned_tokens array.
    /// @param from address representing the previous owner of the given token ID
    /// @param token_id uint256 ID of the token to be removed from the tokens list of the given address
    fn _remove_token_from_owner_enumeration(
        &mut self,
        from: E::AccountId,
        token_id: &TokenId,
    ) {
        // To prevent a gap in from's tokens array, we store the last token in the index of the token to delete, and
        // then delete the last slot (swap and pop).

        let last_token_index = ERC721::balance_of(self, &from) - E::Balance::from(1_u32);

        let data = Storage::<E, Data<E>>::get_mut(self);
        let token_index = data
            .owned_tokens_index
            .get(token_id)
            .expect("ERC721Enumerable: no found index for token id")
            .clone();

        // When the token to delete is the last token, the swap operation is unnecessary
        if token_index != last_token_index {
            let last_token_id = data
                .owned_tokens
                .get(&(from.clone(), last_token_index.clone()))
                .expect("ERC721Enumerable: no found token id from owned_tokens")
                .clone();

            data.owned_tokens
                .insert((from.clone(), token_index.clone()), last_token_id); // Move the last token to the slot of the to-delete token
            data.owned_tokens_index.insert(last_token_id, token_index); // Update the moved token's index
        }

        // This also deletes the contents at the last position of the array
        data.owned_tokens_index.take(token_id);
        data.owned_tokens.take(&(from, last_token_index));
    }

    /// @dev Private function to remove a token from this extension's token tracking data structures.
    /// This has O(1) time complexity, but alters the order of the all_tokens array.
    /// @param token_id uint256 ID of the token to be removed from the tokens list
    fn _remove_token_from_all_tokens_enumeration(&mut self, token_id: &TokenId) {
        // To prevent a gap in the tokens array, we store the last token in the index of the token to delete, and
        // then delete the last slot (swap and pop).

        let data = Storage::<E, Data<E>>::get_mut(self);

        let last_token_index = data.all_tokens.len() - 1;
        let token_index = data
            .all_tokens_index
            .get(token_id)
            .expect("ERC721Enumerable: no found index in all token")
            .clone();

        // When the token to delete is the last token, the swap operation is unnecessary. However, since this occurs so
        // rarely (when the last minted token is burnt) that we still do the swap here to avoid the gas cost of adding
        // an 'if' statement (like in _removeTokenFromOwnerEnumeration)
        let last_token_id = data
            .all_tokens
            .get(last_token_index)
            .expect("ERC721Enumerable: no found last index in all token")
            .clone();

        data.all_tokens
            .set(token_index, last_token_id)
            .expect("ERC721Enumerable: set token index panic"); // Move the last token to the slot of the to-delete token
        data.all_tokens_index.insert(last_token_id, token_index); // Update the moved token's index

        // This also deletes the contents at the last position of the array
        data.all_tokens_index.take(token_id);
        data.all_tokens.pop();
    }
}

pub trait Impl<E>: _Impl<E>
where
    E: Env,
{
    fn before_token_transfer(
        &mut self,
        from: Option<E::AccountId>,
        to: Option<E::AccountId>,
        token_id: &TokenId,
    ) -> Result<()> {
        if from.is_none() {
            _Impl::_add_token_to_all_tokens_enumeration(self, token_id.clone());
        } else if from != to {
            _Impl::_remove_token_from_owner_enumeration(
                self,
                from.clone().expect("ERC721Enumerable: none from get"),
                token_id,
            );
        }

        if to.is_none() {
            _Impl::_remove_token_from_all_tokens_enumeration(self, token_id);
        } else if to != from {
            _Impl::_add_token_to_owner_enumeration(
                self,
                to.expect("ERC721Enumerable: none to get"),
                token_id,
            );
        }

        Ok(())
    }
}

impl<E, C> _Impl<E> for C
where
    C: Impl<E>,
    E: Env,
{
}
