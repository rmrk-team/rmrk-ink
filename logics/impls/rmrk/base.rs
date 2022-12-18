//! RMRK Base implementation

use crate::impls::rmrk::{
    errors::RmrkError,
    types::*,
};
pub use crate::traits::base::{
    Base,
    Internal,
};
use ink_prelude::{
    string::String as PreludeString,
    vec::Vec,
};
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

/// Implement internal helper trait for MultiAsset
impl<T> Internal for T
where
    T: Storage<BaseData>,
{
    default fn ensure_only_slot(&self, part_id: PartId) -> Result<(), PSP34Error> {
        if let Some(part) = self.data::<BaseData>().parts.get(part_id) {
            if part.part_type != PartType::Slot {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::PartIsNotSlot.as_str(),
                )))
            }
            return Ok(())
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::UnknownPartId.as_str(),
            )))
        }
        // match self.data::<BaseData>().parts.get(part_id) {
        //     Some(ref p) => Ok(p),
        //     None => Err(PSP34Error::Custom(String::from(
        //                 RmrkError::PartIsNotSlot.as_str(),
        //             )))
        // }
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
            self.data::<BaseData>().parts.insert(part_id, &part);
            self.data::<BaseData>().part_ids.push(part_id);
            self.data::<BaseData>().next_part_id += 1;
        }

        Ok(())
    }

    /// Add collection address(es) that can be used to equip given `PartId`.
    #[modifiers(only_owner)]
    default fn add_equipable_addresses(
        &mut self,
        part_id: PartId,
        equipable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error> {
        self.ensure_only_slot(part_id)?;
        if let Some(mut part) = self.data::<BaseData>().parts.get(part_id) {
            for address in equipable_address {
                part.equippable.push(address);
            }
        }

        Ok(())
    }

    /// Remove list of equipable addresses for given Part
    #[modifiers(only_owner)]
    default fn reset_equipable_addresses(&mut self, part_id: PartId) -> Result<(), PSP34Error> {
        self.ensure_only_slot(part_id)?;
        if let Some(mut part) = self.data::<BaseData>().parts.get(part_id) {
            part.is_equippable_by_all = false;
            part.equippable.clear();
            self.data::<BaseData>().parts.insert(part_id, &part);
        }

        Ok(())
    }

    /// Sets the is_equippable_by_all flag to true, meaning that any collection may be equipped into the `PartId`
    #[modifiers(only_owner)]
    default fn set_equippable_by_all(&mut self, part_id: PartId) -> Result<(), PSP34Error> {
        self.ensure_only_slot(part_id)?;
        if let Some(mut part) = self.data::<BaseData>().parts.get(part_id) {
            part.is_equippable_by_all = true;
            self.data::<BaseData>().parts.insert(part_id, &part);
        }

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
        PreludeString::from_utf8(self.data::<BaseData>().base_metadata_uri.clone()).unwrap()
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
    default fn is_equippable(&self, part_id: PartId, target_address: AccountId) -> bool {
        if let Some(part) = self.data::<BaseData>().parts.get(part_id) {
            if part.equippable.contains(&target_address) {
                return true
            }
        }

        return false
    }

    /// Checks if the given `PartId` can be equipped by any collection
    default fn is_equippable_by_all(&self, part_id: PartId) -> bool {
        if let Some(part) = self.data::<BaseData>().parts.get(part_id) {
            return part.is_equippable_by_all
        }

        return false
    }
}

