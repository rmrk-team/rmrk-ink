//! RMRK Base implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod traits;

use traits::{
    Base,
    Internal,
};

use rmrk_common::{
    errors::RmrkError,
    types::*,
};

use ink_prelude::{
    string::String as PreludeString,
    vec::Vec,
};
use ink_storage::Mapping;

use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::enumerable::*,
    },
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

/// Implement internal helper trait for Base
impl<T> Internal for T
where
    T: Storage<BaseData>,
{
    default fn ensure_only_slot(&self, part_id: PartId) -> Result<Part, PSP34Error> {
        if let Some(part) = self.data::<BaseData>().parts.get(part_id) {
            if part.part_type != PartType::Slot {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::PartIsNotSlot.as_str(),
                )))
            }
            return Ok(part)
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::UnknownPartId.as_str(),
            )))
        }
    }
}
impl<T> Base for T
where
    T: Storage<BaseData> + Storage<ownable::Data>,
{
    /// Add one or more parts to the base
    #[modifiers(only_owner)]
    default fn add_part_list(&mut self, parts: Vec<Part>) -> Result<(), PSP34Error> {
        for part in parts {
            let part_id = self.data::<BaseData>().next_part_id;

            if part.part_type == PartType::Fixed
                && (part.equippable.len() != 0 || part.is_equippable_by_all)
            {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::BadConfig.as_str(),
                )))
            }
            self.data::<BaseData>().parts.insert(part_id, &part);
            self.data::<BaseData>().part_ids.push(part_id);
            self.data::<BaseData>().next_part_id += 1;
        }

        Ok(())
    }

    /// Add collection address(es) that can be used to equip given `PartId`.
    #[modifiers(only_owner)]
    default fn add_equippable_addresses(
        &mut self,
        part_id: PartId,
        equippable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error> {
        let mut part = self.ensure_only_slot(part_id)?;
        part.equippable.extend(equippable_address);
        self.data::<BaseData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Remove list of equippable addresses for given Part
    #[modifiers(only_owner)]
    default fn reset_equippable_addresses(&mut self, part_id: PartId) -> Result<(), PSP34Error> {
        let mut part = self.ensure_only_slot(part_id)?;
        part.is_equippable_by_all = false;
        part.equippable.clear();
        self.data::<BaseData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Sets the is_equippable_by_all flag to true, meaning that any collection may be equipped into the `PartId`
    #[modifiers(only_owner)]
    default fn set_equippable_by_all(&mut self, part_id: PartId) -> Result<(), PSP34Error> {
        let mut part = self.ensure_only_slot(part_id)?;
        part.is_equippable_by_all = true;
        self.data::<BaseData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Sets the metadata URI for Base
    #[modifiers(only_owner)]
    default fn setup_base(&mut self, base_metadata: String) -> Result<(), PSP34Error> {
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
    default fn ensure_equippable(
        &self,
        part_id: PartId,
        target_address: AccountId,
    ) -> Result<(), PSP34Error> {
        if let Some(part) = self.data::<BaseData>().parts.get(part_id) {
            if !part.equippable.contains(&target_address) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AddressNotEquippable.as_str(),
                )))
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
