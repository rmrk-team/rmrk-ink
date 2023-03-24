//! RMRK Equippable implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::inline_fn_without_body)]

pub mod internal;
pub mod traits;

use internal::Internal;

use rmrk_catalog::traits::CatalogRef;
use rmrk_common::{
    errors::{
        Result,
        RmrkError,
    },
    types::*,
    utils::Utils,
};

use rmrk_multiasset::{
    internal::Internal as MultiAssetInternal,
    traits::MultiAsset,
    MultiAssetData,
};

use traits::{
    Equippable,
    EquippableEvents,
};

use ink::storage::Mapping;

use openbrush::{
    contracts::{
        access_control,
        psp34::extensions::enumerable::*,
    },
    traits::{
        AccountId,
        Storage,
    },
};

pub const STORAGE_EQUIPMENT_KEY: u32 = openbrush::storage_unique_key!(EquipmentData);

/// Used to link tokens with Equipment
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_EQUIPMENT_KEY)]
pub struct EquippableData {
    pub equipment: Mapping<(Id, PartId), Equipment>,
    pub valid_parent_slot: Mapping<(EquippableGroupId, AccountId), PartId>,
}

impl<T> Equippable for T
where
    T: Storage<EquippableData>
        + Storage<access_control::Data>
        + Storage<MultiAssetData>
        + MultiAsset
        + MultiAssetInternal
        + Internal
        + Utils,
{
    /// Used to equip a child nft into a token.
    default fn equip(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        slot_part_id: PartId,
        child_nft: ChildNft,
        child_asset_id: AssetId,
    ) -> Result<()> {
        let token_owner = self.ensure_exists_and_get_owner(&token_id)?;
        self.ensure_token_owner(token_owner)?;
        self.ensure_asset_accepts_slot(&asset_id, &slot_part_id)?;
        self.ensure_token_slot_free(&token_id, &slot_part_id)?;

        // TODO Cross contract call to check from child prespective. Implement as issue#33
        // EquippableRef::ensure_token_can_be_equipped_with_asset_into_slot(child_nft.0, Self::env().account_id(),
        //     child_nft.1,
        //     child_asset_id,
        //     slot_part_id)?;

        // Check from base perspective. If catalog for this asset is None, then it is not equippable.
        match self
            .data::<MultiAssetData>()
            .asset_catalog_address
            .get(&asset_id)
            .ok_or(RmrkError::CatalogNotFoundForAsset)?
        {
            Some(catalog_address) => {
                CatalogRef::ensure_equippable(&catalog_address, slot_part_id, child_nft.0)?;
            }
            None => return Err(RmrkError::AssetIdNotEquippable.into()),
        }

        // insert equipment
        let equipment = Equipment {
            asset_id,
            child_asset_id,
            child_nft: child_nft.clone(),
        };
        self.data::<EquippableData>()
            .equipment
            .insert((token_id.clone(), slot_part_id), &equipment);

        self.emit_child_asset_equipped(token_id, asset_id, slot_part_id, child_nft, child_asset_id);
        Ok(())
    }

    /// Used to unequip child from parent token.
    default fn unequip(&mut self, token_id: Id, slot_part_id: PartId) -> Result<()> {
        let token_owner = self.ensure_exists_and_get_owner(&token_id)?;
        self.ensure_token_owner(token_owner)?;
        let equipment = self.ensure_equipped(&token_id, &slot_part_id)?;

        self.data::<EquippableData>()
            .equipment
            .remove((token_id.clone(), slot_part_id));

        self.emit_child_asset_unequipped(token_id, equipment.asset_id, slot_part_id);
        Ok(())
    }

    /// Used to declare that the assets belonging to a given `equippableGroupId` are equippable into the `Slot`
    /// associated with the `partId` of the collection at the specified `parentAddress`
    default fn set_valid_parent_for_equippable_group(
        &mut self,
        equippable_group_id: EquippableGroupId,
        parent_address: AccountId,
        part_id: PartId,
    ) -> Result<()> {
        self.data::<EquippableData>()
            .valid_parent_slot
            .insert((equippable_group_id, parent_address), &part_id);
        self.emit_valid_parent_equippable_group_set(equippable_group_id, part_id, parent_address);

        Ok(())
    }

    /// Used to get the Equipment object equipped into the specified slot of the desired token.
    default fn get_equipment(&self, token_id: Id, slot_part_id: PartId) -> Option<Equipment> {
        self.data::<EquippableData>()
            .equipment
            .get((token_id, slot_part_id))
    }

    /// Used to get the asset and equippable data associated with given `asset_id`.
    default fn get_asset_and_equippable_data(
        &self,
        token_id: Id,
        asset_id: AssetId,
    ) -> Result<Asset> {
        self.ensure_asset_accepted(&token_id, &asset_id)?;

        if let Some(asset) = self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
        {
            return Ok(asset)
        } else {
            return Err(RmrkError::AssetIdNotFound.into())
        }
    }
}

impl<T> EquippableEvents for T {
    /// Used to notify listeners that a child's asset has been equipped into one of its parent assets.
    default fn emit_child_asset_equipped(
        &self,
        _token_id: Id,
        _asset_id: AssetId,
        _slot_part_id: PartId,
        _child_nft: ChildNft,
        _child_asset_id: AssetId,
    ) {
    }

    /// Used to notify listeners that an asset object at `asset_id` is added to token's pending asset
    default fn emit_child_asset_unequipped(
        &self,
        _token_id: Id,
        _asset_id: AssetId,
        _slot_part_id: PartId,
    ) {
    }

    /// Used to notify listeners that the assets belonging to a `equippableGroupId` have been marked as
    /// equippable into a given slot and parent
    default fn emit_valid_parent_equippable_group_set(
        &self,
        _group_id: EquippableGroupId,
        _slot_part_id: PartId,
        _parent_address: AccountId,
    ) {
    }
}
