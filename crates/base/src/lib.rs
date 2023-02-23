//! RMRK Base implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::inline_fn_without_body)]

pub mod internal;
pub mod traits;

use internal::Internal;
use traits::Base;

use rmrk_common::{
    errors::{
        Result,
        RmrkError,
    },
    roles::CONTRIBUTOR,
    types::*,
};

use ink::{
    prelude::{
        string::String as PreludeString,
        vec::Vec,
    },
    storage::Mapping,
};

use openbrush::{
    contracts::access_control::*,
    modifiers,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

pub const STORAGE_BASE_KEY: u32 = openbrush::storage_unique_key!(BaseData);

/// The structure used to describe the Base
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_BASE_KEY)]
pub struct BaseData {
    /// List of all parts of Base.
    pub part_ids: Vec<PartId>,

    /// Mapping for all part details.
    pub parts: Mapping<PartId, Part>,

    /// Counter for assigning new parts to Base.
    pub next_part_id: PartId,

    /// Metadata for Base
    pub base_metadata_uri: String,
}

impl<T> Base for T
where
    T: Storage<BaseData> + Storage<access_control::Data>,
{
    /// Add one or more parts to the base
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn add_part_list(&mut self, parts: Vec<Part>) -> Result<()> {
        for part in parts {
            let part_id = self.data::<BaseData>().next_part_id;

            if part.part_type == PartType::Fixed
                && (part.equippable.len() != 0 || part.is_equippable_by_all)
            {
                return Err(RmrkError::BadConfig.into())
            }
            self.data::<BaseData>().parts.insert(part_id, &part);
            self.data::<BaseData>().part_ids.push(part_id);
            self.data::<BaseData>().next_part_id += 1;
        }

        Ok(())
    }

    /// Add collection address(es) that can be used to equip given `PartId`.
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn add_equippable_addresses(
        &mut self,
        part_id: PartId,
        equippable_address: Vec<AccountId>,
    ) -> Result<()> {
        let mut part = self.ensure_only_slot(part_id)?;
        part.equippable.extend(equippable_address);
        self.data::<BaseData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Remove list of equippable addresses for given Part
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn reset_equippable_addresses(&mut self, part_id: PartId) -> Result<()> {
        let mut part = self.ensure_only_slot(part_id)?;
        part.is_equippable_by_all = false;
        part.equippable.clear();
        self.data::<BaseData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Sets the is_equippable_by_all flag to true, meaning that any collection may be equipped into the `PartId`
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn set_equippable_by_all(&mut self, part_id: PartId) -> Result<()> {
        let mut part = self.ensure_only_slot(part_id)?;
        part.is_equippable_by_all = true;
        self.data::<BaseData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Sets the metadata URI for Base
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn setup_base(&mut self, base_metadata: String) -> Result<()> {
        self.data::<BaseData>().base_metadata_uri = base_metadata;

        Ok(())
    }

    /// Get the Base metadataURI.
    default fn get_base_metadata(&self) -> PreludeString {
        match PreludeString::from_utf8(self.data::<BaseData>().base_metadata_uri.clone()) {
            Ok(m) => m,
            _ => PreludeString::from(""),
        }
    }

    /// Get the number of parts.
    default fn get_parts_count(&self) -> PartId {
        self.data::<BaseData>().next_part_id
    }

    /// Get the part details for the given PartId.
    default fn get_part(&self, part_id: PartId) -> Option<Part> {
        self.data::<BaseData>().parts.get(part_id)
    }

    /// Check whether the given address is allowed to equip the desired `PartId`.
    default fn ensure_equippable(&self, part_id: PartId, target_address: AccountId) -> Result<()> {
        if let Some(part) = self.data::<BaseData>().parts.get(part_id) {
            if !part.equippable.contains(&target_address) {
                return Err(RmrkError::AddressNotEquippable.into())
            }
        }

        Ok(())
    }

    /// Checks if the given `PartId` can be equipped by any collection
    default fn is_equippable_by_all(&self, part_id: PartId) -> bool {
        if let Some(part) = self.data::<BaseData>().parts.get(part_id) {
            return part.is_equippable_by_all
        }

        return false
    }
}
