#![allow(clippy::inline_fn_without_body)]

use crate::traits::{
    MintingRef,
    MultiAssetRef,
    NestingRef,
};

use ink::prelude::vec::Vec;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        String,
    },
};
use rmrk_common::{
    errors::Error,
    types::*,
};

#[derive(scale::Encode, scale::Decode, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Token {
    id: u64,
    collection_id: CollectionId,
    token_uri: String,
    assets_pending: Vec<AssetId>,
    assets_accepted: Vec<AssetId>,
    children_pending: Vec<(AccountId, u64)>,
    children_accepted: Vec<(AccountId, u64)>,
}

fn unsafe_id_to_u64(id: Id) -> u64 {
    match id {
        Id::U64(id) => id,
        _ => panic!("Id not u64"),
    }
}

fn unpack_children_id(children: Vec<(AccountId, Id)>) -> Vec<(AccountId, u64)> {
    children
        .into_iter()
        .map(|(account_id, id)| (account_id, unsafe_id_to_u64(id)))
        .collect()
}

fn nested_result_unwrap_or_default<T: Default>(
    res: Result<Result<T, ink::LangError>, ink::env::Error>,
) -> T {
    match res {
        Ok(Ok(v)) => v,
        _ => Default::default(),
    }
}

fn nested_deep_result_unwrap_or_default<T: Default>(
    res: Result<Result<Result<T, Error>, ink::LangError>, ink::env::Error>,
) -> T {
    match res {
        Ok(Ok(Ok(v))) => v,
        _ => Default::default(),
    }
}

#[openbrush::wrapper]
pub type QueryRef = dyn Query;

#[openbrush::trait_definition]
pub trait Query {
    #[ink(message)]
    fn get_asset(&self, collection_id: AccountId, asset_id: AssetId) -> Option<Asset> {
        nested_result_unwrap_or_default(
            MultiAssetRef::get_asset_builder(&collection_id, asset_id).try_invoke(),
        )
    }

    #[ink(message)]
    fn get_assets(&self, collection_id: AccountId, asset_ids: Vec<AssetId>) -> Vec<Asset> {
        asset_ids
            .into_iter()
            .filter_map(|id| self.get_asset(collection_id, id))
            .collect()
    }

    #[ink(message)]
    fn get_token(&self, collection_id: AccountId, id_u64: u64) -> Token {
        let id = Id::U64(id_u64);

        let token_uri = MintingRef::token_uri(&collection_id, id_u64).unwrap_or_default();

        let assets_pending = nested_deep_result_unwrap_or_default(
            MultiAssetRef::get_pending_token_assets_builder(&collection_id, id.clone())
                .try_invoke(),
        );

        let assets_accepted = nested_deep_result_unwrap_or_default(
            MultiAssetRef::get_accepted_token_assets_builder(&collection_id, id.clone())
                .try_invoke(),
        );

        let children_pending = nested_result_unwrap_or_default(
            NestingRef::get_pending_children_builder(&collection_id, id.clone()).try_invoke(),
        );

        let children_accepted = nested_result_unwrap_or_default(
            NestingRef::get_accepted_children_builder(&collection_id, id).try_invoke(),
        );

        Token {
            id: id_u64,
            collection_id,
            token_uri: String::from(token_uri),
            assets_pending,
            assets_accepted,
            children_pending: unpack_children_id(children_pending),
            children_accepted: unpack_children_id(children_accepted),
        }
    }

    #[ink(message)]
    fn get_parent_of_child(&self, child_nft: ChildNft) -> Option<Id> {
        let child_collection = child_nft.clone().0;
        let child_id = child_nft.clone().1;

        let parent_collection = nested_result_unwrap_or_default(
            PSP34Ref::owner_of_builder(&child_collection, child_id)
                .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                .try_invoke(),
        )?;

        nested_result_unwrap_or_default(
            NestingRef::get_parent_of_child_in_collection_builder(&parent_collection, child_nft)
                .try_invoke(),
        )
    }
}
