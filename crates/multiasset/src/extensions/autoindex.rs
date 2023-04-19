use crate::MultiAsset;

use ink::prelude::vec::Vec;

use openbrush::traits::{
    AccountId,
    Storage,
    String,
};
use rmrk_common::{
    counter::Counter,
    errors::Result,
    types::*,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(MultiAssetAutoIndex);

/// Storage for AutoIndex `AssetId` counter
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct MultiAssetAutoIndexData {
    pub asset_id: Counter<AssetId>,
}

#[openbrush::wrapper]
pub type MultiAssetAutoIndexRef = dyn MultiAssetAutoIndex;

#[openbrush::trait_definition]
pub trait MultiAssetAutoIndex {
    /// Add an asset entry, with auto-geenrated AssetId
    #[ink(message)]
    fn add_asset_entry(
        &mut self,
        catalog_address: Option<AccountId>,
        equippable_group_id: EquippableGroupId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<AssetId>;
}

impl<T> MultiAssetAutoIndex for T
where
    T: Storage<MultiAssetAutoIndexData> + MultiAsset,
{
    /// Add an asset entry, with auto-geenrated AssetId
    default fn add_asset_entry(
        &mut self,
        catalog_address: Option<AccountId>,
        equippable_group_id: EquippableGroupId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<AssetId> {
        let next_id = self.data::<MultiAssetAutoIndexData>().asset_id.next()?;
        MultiAsset::add_asset_entry(
            self,
            catalog_address,
            next_id,
            equippable_group_id,
            asset_uri,
            part_ids,
        )?;
        Ok(next_id)
    }
}
