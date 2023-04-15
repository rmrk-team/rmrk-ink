use crate::{
    MultiAsset,
    MultiAssetData,
};

use ink::{
    prelude::vec::Vec,
    storage::Mapping,
};

use openbrush::{
    contracts::{
        access_control::*,
        psp34::extensions::enumerable::*,
    },
    modifiers,
    traits::{
        AccountId,
        Storage,
        String,
    },
};
use rmrk_common::{
    counter::Counter,
    errors::{
        Error,
        Result,
        RmrkError,
    },
    types::*,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(MultiAssetIncremental);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct MultiAssetIncrementalData {
    pub counter: Counter,
}

#[openbrush::wrapper]
pub type MultiAssetEnumerableRef = dyn MultiAssetIncremental;

#[openbrush::trait_definition]
pub trait MultiAssetIncremental {
    #[ink(message)]
    fn add_asset_entry(
        &mut self,
        catalog_address: Option<AccountId>,
        equippable_group_id: EquippableGroupId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<AssetId>;
}

impl<T> MultiAssetIncremental for T
where
    T: Storage<MultiAssetIncrementalData> + MultiAsset,
{
    default fn add_asset_entry(
        &mut self,
        catalog_address: Option<AccountId>,
        equippable_group_id: EquippableGroupId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<AssetId> {
        let next_id = self.data::<MultiAssetIncrementalData>().counter.next()?;
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
