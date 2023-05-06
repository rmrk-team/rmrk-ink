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
        DefaultEnv,
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
    id: Id,
    collection_id: CollectionId,
    token_uri: String,
    assets_pending: Vec<AssetId>,
    assets_accepted: Vec<AssetId>,
    children_pending: Vec<(AccountId, Id)>,
    children_accepted: Vec<(AccountId, Id)>,
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
pub trait Query: DefaultEnv {
    #[ink(message)]
    fn get_asset(&self, asset_id: AssetId) -> Option<Asset> {
        nested_result_unwrap_or_default(
            MultiAssetRef::get_asset_builder(&<Self as DefaultEnv>::env().account_id(), asset_id)
                .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                .try_invoke(),
        )
    }

    #[ink(message)]
    fn get_assets(&self, asset_ids: Vec<AssetId>) -> Vec<Asset> {
        asset_ids
            .into_iter()
            .filter_map(|id| self.get_asset(id))
            .collect()
    }

    #[ink(message)]
    fn get_token(&self, id: Id) -> Token {
        let id_u64 = match id {
            Id::U64(id) => id.clone(),
            _ => panic!("expecting Id::U64"),
        };

        let collection_id = <Self as DefaultEnv>::env().account_id();

        let token_uri = nested_deep_result_unwrap_or_default(
            MintingRef::token_uri_builder(&collection_id, id_u64)
                .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                .try_invoke(),
        );

        let assets_pending = nested_deep_result_unwrap_or_default(
            MultiAssetRef::get_pending_token_assets_builder(&collection_id, id.clone())
                .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                .try_invoke(),
        );

        let assets_accepted = nested_deep_result_unwrap_or_default(
            MultiAssetRef::get_accepted_token_assets_builder(&collection_id, id.clone())
                .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                .try_invoke(),
        );

        let children_pending = nested_result_unwrap_or_default(
            NestingRef::get_pending_children_builder(&collection_id, id.clone())
                .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                .try_invoke(),
        );

        let children_accepted = nested_result_unwrap_or_default(
            NestingRef::get_accepted_children_builder(&collection_id, id.clone())
                .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                .try_invoke(),
        );

        Token {
            id,
            collection_id,
            token_uri: String::from(token_uri),
            assets_pending,
            assets_accepted,
            children_pending,
            children_accepted,
        }
    }

    #[ink(message)]
    fn get_parent_of_child(&self, child_nft: ChildNft) -> Option<Id> {
        let child_collection = child_nft.0;
        let child_id = child_nft.1.clone();

        let parent_collection = nested_result_unwrap_or_default(
            PSP34Ref::owner_of_builder(&child_collection, child_id)
                .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))
                .try_invoke(),
        )?;

        nested_result_unwrap_or_default(
            NestingRef::get_parent_of_child_builder(&parent_collection, child_nft).try_invoke(),
        )
    }
}
