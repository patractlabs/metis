use crate::{
    Impl as ERC721,
    TokenId,
};

use ink_prelude::string::String;
use metis_lang::{
    Env,
    Storage,
};

#[cfg(not(feature = "ink-as-dependency"))]
use ::ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};

#[cfg_attr(feature = "std", derive(::ink_storage::traits::StorageLayout))]
#[derive(Debug, SpreadLayout)]
pub struct Data {
    /// Mapping from token ID to index of the owner tokens list
    url_storage: StorageHashMap<TokenId, String>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            url_storage: StorageHashMap::default(),
        }
    }
}

pub trait Impl<E>: ERC721<E> + Storage<E, Data>
where
    E: Env,
{
    /// Returns the Uniform Resource Identifier (URI) for `token_id` token.
    fn token_url(&self, token_id: &TokenId) -> String {
        assert!(
            self._exists(token_id),
            "ERC721Metadata: URI query for nonexistent token"
        );

        let token_uri = Storage::<E, Data>::get(self).url_storage.get(token_id);
        let base_url = self._base_url().clone();

        // If there is no base URI, return the token URI.
        if base_url.len() == 0 {
            return token_uri.unwrap_or(&String::from("")).clone()
        }

        // If both are set, concatenate the baseURI and tokenURI (via abi.encodePacked).
        if let Some(token_url) = token_uri {
            if token_url.len() > 0 {
                let mut res = base_url.clone();
                res.push_str(&token_url);
                return res
            }
        }

        ERC721::token_url(self, token_id)
    }
}
