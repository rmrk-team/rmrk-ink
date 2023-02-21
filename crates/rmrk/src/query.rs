use crate::traits::{
    BaseRef,
    MintingRef,
    MultiAssetRef,
    NestingRef,
};

use ink_env::AccountId;
use ink_prelude::vec::Vec;
use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::String,
};
use rmrk_common::{
    errors::Error,
    types::*,
};

#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
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

fn nested_result_unwrap_or_default<T: Default>(res: Result<Result<T, Error>, ink_env::Error>) -> T {
    match res {
        Ok(Ok(v)) => v,
        _ => Default::default(),
    }
}

#[openbrush::wrapper]
pub type QueryRef = dyn Query;

#[openbrush::trait_definition]
pub trait Query {
    #[ink(message)]
    fn get_part(&self, collection_id: AccountId, part_id: PartId) -> Option<Part> {
        BaseRef::get_part_builder(&collection_id, part_id)
            .fire()
            .ok()
            .flatten()
    }

    #[ink(message)]
    fn get_asset(&self, collection_id: AccountId, asset_id: AssetId) -> Option<Asset> {
        MultiAssetRef::get_asset_builder(&collection_id, asset_id)
            .fire()
            .ok()
            .flatten()
    }

    #[ink(message)]
    fn get_parts(&self, collection_id: AccountId, part_ids: Vec<PartId>) -> Vec<Part> {
        part_ids
            .into_iter()
            .filter_map(|id| self.get_part(collection_id, id))
            .collect()
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

        let assets_pending = nested_result_unwrap_or_default(
            MultiAssetRef::get_pending_token_assets_builder(&collection_id, id.clone()).fire(),
        );

        let assets_accepted = nested_result_unwrap_or_default(
            MultiAssetRef::get_accepted_token_assets_builder(&collection_id, id.clone()).fire(),
        );

        let children_pending = NestingRef::get_pending_children_builder(&collection_id, id.clone())
            .fire()
            .unwrap_or_default();

        let children_accepted =
            NestingRef::get_accepted_children_builder(&collection_id, id.clone())
                .fire()
                .unwrap_or_default();

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
}
