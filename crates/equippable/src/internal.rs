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

use crate::EquippableData;

use openbrush::{
    contracts::{
        access_control::*,
        psp34::extensions::enumerable::*,
    },
    traits::{
        AccountId,
        Storage,
    },
};

/// Trait definitions for Resource helper functions
pub trait Internal {
    /// Check if slot is already used/equipped.
    fn ensure_token_slot_free(&self, token_id: &Id, part_id: &PartId) -> Result<()>;

    /// Check if asset is already added.
    fn ensure_asset_accepts_slot(&self, asset_id: &AssetId, part_id: &PartId) -> Result<()>;

    /// Used to ensure a token can be equipped into a given parent's slot.
    /// # Arguments:
    ///  * parent Address of the parent token's smart contract
    ///  * tokenId ID of the token we want to equip
    ///  * asset_id ID of the asset associated with the token we want to equip
    ///  * slotId ID of the slot that we want to equip the token into
    /// * @return bool The boolean indicating whether the token with the given asset can be equipped into the desired
    fn ensure_token_can_be_equipped_with_asset_into_slot(
        &self,
        parent_address: AccountId,
        parent_token_id: Id,
        asset_id: AssetId,
        slot_part_id: PartId,
    ) -> Result<()>;

    /// Used to ensure a token is equipped and can be un-equipped.
    fn ensure_equipped(&self, token_id: &Id, slot_part_id: &PartId) -> Result<Equipment>;
}

/// Implement internal helper trait for Equippable
impl<T> Internal for T
where
    T: Storage<EquippableData>
        + Storage<access_control::Data>
        + Storage<MultiAssetData>
        + MultiAsset
        + MultiAssetInternal
        + Utils,
{
    /// Check if slot is already used/equipped.
    default fn ensure_token_slot_free(&self, token_id: &Id, part_id: &PartId) -> Result<()> {
        if (self
            .data::<EquippableData>()
            .equipment
            .get((token_id, part_id)))
        .is_some()
        {
            return Err(RmrkError::SlotAlreadyUsed.into())
        }
        Ok(())
    }
    /// Check if asset is already added.
    default fn ensure_asset_accepts_slot(
        &self,
        asset_id: &AssetId,
        part_id: &PartId,
    ) -> Result<()> {
        let asset = self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
            .ok_or(RmrkError::AssetHasNoParts)?;

        if !asset.part_ids.contains(part_id) {
            return Err(RmrkError::TargetAssetCannotReceiveSlot.into())
        }
        Ok(())
    }

    // Used to ensure a token can be equipped into a given parent's slot. Check in child NFT
    fn ensure_token_can_be_equipped_with_asset_into_slot(
        &self,
        parent_address: AccountId,
        token_id: Id,
        asset_id: AssetId,
        _part_slot_id: PartId,
    ) -> Result<()> {
        let asset = self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
            .ok_or(RmrkError::UnknownEquippableAsset)?;

        if self
            .data::<EquippableData>()
            .valid_parent_slot
            .get((asset.equippable_group_id, parent_address))
            .is_some()
        {
            self.ensure_asset_accepted(&token_id, &asset_id)?;
        } else {
            return Err(RmrkError::UnknownPart.into())
        }

        Ok(())
    }

    /// Used to ensure a token is not equipped and can be un-equipped.
    fn ensure_equipped(&self, token_id: &Id, slot_part_id: &PartId) -> Result<Equipment> {
        if let Some(equipment) = self
            .data::<EquippableData>()
            .equipment
            .get((token_id, slot_part_id))
        {
            Ok(equipment)
        } else {
            Err(RmrkError::NotEquipped.into())
        }
    }
}
