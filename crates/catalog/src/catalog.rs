//! RMRK Catalog implementation

use crate::{
    internal::Internal,
    traits::Catalog,
};

use rmrk_common::{
    errors::{
        Error,
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

pub const STORAGE_BASE_KEY: u32 = openbrush::storage_unique_key!(CatalogData);

/// The structure used to describe the Catalog
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_BASE_KEY)]
pub struct CatalogData {
    /// List of all parts of Catalog.
    pub part_ids: Vec<PartId>,

    /// Mapping for all part details.
    pub parts: Mapping<PartId, Part>,

    /// Metadata for Catalog
    pub catalog_metadata: String,
}

impl<T> Catalog for T
where
    T: Storage<CatalogData> + Storage<access_control::Data>,
{
    /// Add one or more parts to the Catalog
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn add_part_list(&mut self, part_ids: Vec<PartId>, parts: Vec<Part>) -> Result<()> {
        if part_ids.len() != parts.len() {
            return Err(RmrkError::BadConfig.into())
        }

        for (i, part) in parts.iter().enumerate() {
            let part_id = part_ids
                .get(i)
                .ok_or(Error::Rmrk(RmrkError::BadConfig))?
                .to_owned();

            if part.part_type == PartType::Fixed
                && (!part.equippable.is_empty() || part.is_equippable_by_all)
            {
                return Err(RmrkError::BadConfig.into())
            }
            self.data::<CatalogData>().parts.insert(part_id, part);
            self.data::<CatalogData>().part_ids.push(part_id);
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
        self.data::<CatalogData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Remove list of equippable addresses for given Part
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn reset_equippable_addresses(&mut self, part_id: PartId) -> Result<()> {
        let mut part = self.ensure_only_slot(part_id)?;
        part.is_equippable_by_all = false;
        part.equippable.clear();
        self.data::<CatalogData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Sets the is_equippable_by_all flag to true, meaning that any collection may be equipped into the `PartId`
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn set_equippable_by_all(&mut self, part_id: PartId) -> Result<()> {
        let mut part = self.ensure_only_slot(part_id)?;
        part.is_equippable_by_all = true;
        self.data::<CatalogData>().parts.insert(part_id, &part);

        Ok(())
    }

    /// Sets the metadata URI for Catalog
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn set_catalog_metadata(&mut self, catalog_metadata: String) -> Result<()> {
        self.data::<CatalogData>().catalog_metadata = catalog_metadata;

        Ok(())
    }

    /// Get the Catalog metadataURI.
    default fn get_catalog_metadata(&self) -> Result<PreludeString> {
        PreludeString::from_utf8(self.data::<CatalogData>().catalog_metadata.clone())
            .map_err(|_| RmrkError::UriNotFound.into())
    }

    /// Get the number of parts.
    default fn get_parts_count(&self) -> u32 {
        self.data::<CatalogData>().part_ids.len() as u32
    }

    /// Get the part details for the given PartId.
    default fn get_part(&self, part_id: PartId) -> Option<Part> {
        self.data::<CatalogData>().parts.get(part_id)
    }

    /// Check whether the given address is allowed to equip the desired `PartId`.
    default fn ensure_equippable(&self, part_id: PartId, target_address: AccountId) -> Result<()> {
        if let Some(part) = self.data::<CatalogData>().parts.get(part_id) {
            if !part.equippable.contains(&target_address) {
                return Err(RmrkError::AddressNotEquippable.into())
            }
        }

        Ok(())
    }

    /// Checks if the given `PartId` can be equipped by any collection
    default fn is_equippable_by_all(&self, part_id: PartId) -> bool {
        if let Some(part) = self.data::<CatalogData>().parts.get(part_id) {
            return part.is_equippable_by_all
        }

        false
    }
}
