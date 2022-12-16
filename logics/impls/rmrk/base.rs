//! RMRK Base implementation

use crate::impls::rmrk::{
    errors::RmrkError,
    types::*,
};
pub use crate::traits::base::Base;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    modifiers,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

impl<T> Base for T
where
    T: Storage<BaseData>,
{
    /// Add one or more parts to the base
    default fn add_part_list(&mut self, part: Vec<Part>) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Mint one or more tokens.
    default fn add_equipable_addresses(
        &mut self,
        part_id: PartId,
        equipable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Mint one or more tokens.
    default fn set_equipable_addresses(
        &mut self,
        part_id: PartId,
        equipable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Remove list of equipable addresses for given Part
    default fn reset_equipable_address(
        &mut self,
        part_id: PartId,
        equipable_address: Vec<AccountId>,
    ) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Sets the is_equippable_by_all flag to true, meaning that any collection may be equipped into the `PartId`
    default fn set_equippable_by_all(&self, part_id: PartId) -> Result<(), PSP34Error> {
        todo!()
    }

    /// Get the Base metadataURI.
    default fn get_base_metadata(&self) -> String {
        todo!()
    }

    /// Get the part type for the given PartId. It can be None, Fixed, Slot.
    default fn get_part_type(&self, part_id: PartId) -> PartType {
        todo!()
    }

    /// Get the list of all parts.
    default fn get_all_parts(&self) -> Vec<PartId> {
        todo!()
    }

    /// Get the part details for the given PartId.
    default fn get_part(&self, part_id: PartId) -> Part {
        todo!()
    }

    /// Check whether the given address is allowed to equip the desired `PartId`.
    default fn is_equippable(&self, part_id: PartId, target_address: AccountId) -> bool {
        todo!()
    }

    /// Checks if is_equippable_by_all is set to true for the given `PartId`
    default fn is_equippable_by_all(&self, part_id: PartId) -> bool {
        todo!()
    }
}
