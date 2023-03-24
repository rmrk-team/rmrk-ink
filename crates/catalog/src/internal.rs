use crate::catalog::BaseData;

use rmrk_common::{
    errors::{
        Result,
        RmrkError,
    },
    types::*,
};

use openbrush::traits::Storage;

/// Implement internal helper trait for Base
pub trait Internal {
    fn ensure_only_slot(&self, part_id: PartId) -> Result<Part>;
}
/// Implement internal helper trait for Base
impl<T> Internal for T
where
    T: Storage<BaseData>,
{
    default fn ensure_only_slot(&self, part_id: PartId) -> Result<Part> {
        if let Some(part) = self.data::<BaseData>().parts.get(part_id) {
            if part.part_type != PartType::Slot {
                return Err(RmrkError::PartIsNotSlot.into())
            }
            return Ok(part)
        } else {
            return Err(RmrkError::UnknownPartId.into())
        }
    }
}
