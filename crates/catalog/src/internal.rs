use crate::catalog::CatalogData;

use rmrk_common::{
    errors::{
        Result,
        RmrkError,
    },
    types::*,
};

use openbrush::traits::Storage;

/// Implement internal helper trait for Catalog
pub trait Internal {
    fn ensure_only_slot(&self, part_id: PartId) -> Result<Part>;
}
/// Implement internal helper trait for Catalog
impl<T> Internal for T
where
    T: Storage<CatalogData>,
{
    default fn ensure_only_slot(&self, part_id: PartId) -> Result<Part> {
        if let Some(part) = self.data::<CatalogData>().parts.get(part_id) {
            if part.part_type != PartType::Slot {
                return Err(RmrkError::PartIsNotSlot.into())
            }
            Ok(part)
        } else {
            Err(RmrkError::UnknownPartId.into())
        }
    }
}
