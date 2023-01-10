//! This module enables multiasset capability of RMRK

use crate::impls::rmrk::{
    errors::RmrkError,
    types::*,
};
pub use crate::traits::{
    multiasset::{
        Internal,
        MultiAsset,
        MultiAssetEvents,
    },
    utils::Utils,
};
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::enumerable::*,
    },
    modifiers,
    traits::{
        Storage,
        String,
    },
};

/// Implement internal helper trait for MultiAsset
impl<T> Internal for T
where
    T: Storage<MultiAssetData> + Storage<psp34::Data<enumerable::Balances>> + Utils,
{
    /// Check if asset is already accepted
    default fn ensure_not_accepted(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self.data::<MultiAssetData>().accepted_assets.get(token_id) {
            if children.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AlreadyAddedAsset.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already pending
    default fn ensure_not_pending(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().pending_assets.get(token_id) {
            if assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AddingPendingAsset.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already pending
    default fn ensure_pending(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().pending_assets.get(token_id) {
            if !assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AssetIdNotFound.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already accepted
    default fn ensure_asset_accepted(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().accepted_assets.get(token_id) {
            if !assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AssetIdNotFound.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Add the asset to the list of accepted assets
    default fn add_to_accepted_assets(&mut self, token_id: &Id, asset_id: &AssetId) {
        let mut assets = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(&token_id)
            .unwrap_or(Vec::new());
        if !assets.contains(&asset_id) {
            assets.push(*asset_id);
            self.data::<MultiAssetData>()
                .accepted_assets
                .insert(&token_id, &assets);
        }
        self._emit_asset_accepted_event(token_id, asset_id);
    }

    /// Add the asset to the list of pending assets
    default fn add_to_pending_assets(&mut self, token_id: &Id, asset_id: &AssetId) {
        let mut assets = self
            .data::<MultiAssetData>()
            .pending_assets
            .get(&token_id)
            .unwrap_or(Vec::new());
        if !assets.contains(&asset_id) {
            assets.push(*asset_id);
            self.data::<MultiAssetData>()
                .pending_assets
                .insert(&token_id, &assets);
        }
    }

    /// remove the asset from the list of pending assets
    default fn remove_from_pending_assets(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        let mut assets = self
            .data::<MultiAssetData>()
            .pending_assets
            .get(&token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidAssetId.as_str(),
            )))?;

        let index = assets
            .iter()
            .position(|a| a == asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidTokenId.as_str(),
            )))?;
        assets.remove(index);

        self.data::<MultiAssetData>()
            .pending_assets
            .insert(&token_id, &assets);

        Ok(())
    }

    /// Remove the asset from the list of accepted assets
    default fn remove_from_accepted_assets(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        let mut assets = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(&token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidAssetId.as_str(),
            )))?;

        let index = assets
            .iter()
            .position(|a| a == asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidTokenId.as_str(),
            )))?;
        assets.remove(index);

        self.data::<MultiAssetData>()
            .accepted_assets
            .insert(&token_id, &assets);

        Ok(())
    }
}

impl<T> MultiAsset for T
where
    T: Storage<MultiAssetData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<ownable::Data>
        + Utils,
{
    /// Used to add a asset entry.
    #[modifiers(only_owner)]
    fn add_asset_entry(
        &mut self,
        asset_id: AssetId,
        equippable_group_id: EquippableGroupId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<(), PSP34Error> {
        self.ensure_asset_id_is_available(asset_id)?;
        self.data::<MultiAssetData>()
            .collection_asset_entries
            .insert(
                asset_id,
                &Asset {
                    equippable_group_id,
                    asset_uri,
                    part_ids: part_ids.clone(),
                },
            );
        self.data::<MultiAssetData>()
            .collection_asset_ids
            .push(asset_id);
        self._emit_asset_set_event(&asset_id);

        Ok(())
    }

    /// Used to add an asset to a token.
    fn add_asset_to_token(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        _replaces_asset_with_id: Option<Id>, // TODO implement replacement
    ) -> Result<(), PSP34Error> {
        // Check if asset id is valid
        self.data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::AssetIdNotFound.as_str(),
            )))?;
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_not_accepted(&token_id, &asset_id)?;
        self.ensure_not_pending(&token_id, &asset_id)?;

        self._emit_asset_added_to_token_event(&token_id, &asset_id, None);
        let caller = Self::env().caller();
        if caller == token_owner {
            self.add_to_accepted_assets(&token_id, &asset_id);
        } else {
            self.add_to_pending_assets(&token_id, &asset_id);
        }

        Ok(())
    }

    /// Accepts an asset from the pending array of given token.
    fn accept_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error> {
        self.ensure_pending(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists(&token_id)?;
        let caller = Self::env().caller();
        if caller == token_owner {
            self.remove_from_pending_assets(&token_id, &asset_id)?;
            self.add_to_accepted_assets(&token_id, &asset_id);
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::NotTokenOwner.as_str(),
            )))
        }
        Ok(())
    }

    /// Rejects an asset from the pending array of given token.
    fn reject_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error> {
        self.ensure_pending(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_token_owner(token_owner)?;

        self.remove_from_pending_assets(&token_id, &asset_id)?;

        self._emit_asset_rejected_event(&token_id, &asset_id);
        Ok(())
    }

    /// Remove an asset from the pending array of given token.
    fn remove_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error> {
        self.ensure_asset_accepted(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_token_owner(token_owner)?;

        self.remove_from_accepted_assets(&token_id, &asset_id)?;

        self._emit_asset_removed_event(&token_id, &asset_id);
        Ok(())
    }

    /// Used to specify the priorities for a given token's active assets.
    fn set_priority(&mut self, token_id: Id, priorities: Vec<AssetId>) -> Result<(), PSP34Error> {
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_token_owner(token_owner)?;
        if let Some(accepted_assets) = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(token_id.clone())
        {
            if accepted_assets.len() != priorities.len() {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::BadPriorityLength.as_str(),
                )))
            }
            for asset in priorities.clone() {
                if !accepted_assets.contains(&asset) {
                    return Err(PSP34Error::Custom(String::from(
                        RmrkError::AssetIdNotFound.as_str(),
                    )))
                }
            }
        }

        self.data::<MultiAssetData>()
            .accepted_assets
            .insert(&token_id, &priorities);
        self._emit_asset_priority_set_event(&token_id, priorities);
        Ok(())
    }

    /// Used to retrieve the total number of asset entries
    fn total_assets(&self) -> u32 {
        self.data::<MultiAssetData>().collection_asset_ids.len() as u32
    }

    /// Used to retrieve the total number of assets per token
    fn total_token_assets(&self, token_id: Id) -> Result<(u64, u64), PSP34Error> {
        self.ensure_exists(&token_id)?;

        let accepted_assets_on_token =
            match self.data::<MultiAssetData>().accepted_assets.get(&token_id) {
                Some(assets) => assets.len() as u64,
                None => 0,
            };

        let pending_assets_on_token =
            match self.data::<MultiAssetData>().pending_assets.get(&token_id) {
                Some(assets) => assets.len() as u64,
                None => 0,
            };

        Ok((accepted_assets_on_token, pending_assets_on_token))
    }

    /// Check that asset id does not already exist.
    default fn ensure_asset_id_is_available(&self, asset_id: AssetId) -> Result<(), PSP34Error> {
        if self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
            .is_some()
        {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::AssetIdAlreadyExists.as_str(),
            )))
        }
        return Ok(())
    }

    /// Used to retrieve asset's uri
    default fn get_asset_uri(&self, asset_id: AssetId) -> Option<String> {
        if let Some(asset) = self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .get(asset_id)
        {
            return Some(asset.asset_uri)
        }
        return None
    }

    /// Fetch all accepted assets for the token_id
    fn get_accepted_token_assets(&self, token_id: Id) -> Result<Option<Vec<AssetId>>, PSP34Error> {
        self.ensure_exists(&token_id)?;
        Ok(self.data::<MultiAssetData>().accepted_assets.get(&token_id))
    }
}

/// Event trait for MultiAssets
impl<T> MultiAssetEvents for T
where
    T: Storage<MultiAssetData>,
{
    /// Used to notify listeners that an asset object is initialized at `assetId`.
    default fn _emit_asset_set_event(&self, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is added to token's pending asset array.
    default fn _emit_asset_added_to_token_event(
        &self,
        _token_id: &Id,
        _asset_id: &AssetId,
        _replaces_id: Option<Id>,
    ) {
    }

    /// Used to notify listeners that an asset object at `assetId` is accepted by the token and migrated
    default fn _emit_asset_accepted_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is rejected from token and is dropped from the pending assets array of the token.
    default fn _emit_asset_rejected_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is removed from token
    default fn _emit_asset_removed_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that token's prioritiy array is reordered.
    default fn _emit_asset_priority_set_event(&self, _token_id: &Id, _priorities: Vec<AssetId>) {}
}
