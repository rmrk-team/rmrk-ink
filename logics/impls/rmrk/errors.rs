//! Error definition for RMRK contract

use openbrush::traits::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    AddingPendingAsset,
    AddingPendingChild,
    AddressNotEquippable,
    AlreadyAddedAsset,
    AlreadyAddedChild,
    AssetHasNoParts,
    AssetIdAlreadyExists,
    AssetIdNotFound,
    BadConfig,
    BadMintValue,
    BadPriorityLength,
    CannotMintZeroTokens,
    ChildNotFound,
    CollectionIsFull,
    InvalidAssetId,
    InvalidParentId,
    InvalidTokenId,
    NotEquipped,
    NotTokenOwner,
    PartIsNotSlot,
    SlotAlreayUsed,
    TargetAssetCannotReceiveSlot,
    UnknownEquippableAsset,
    UnknownPart,
    UnknownPartId,
    WithdrawalFailed,
}

impl RmrkError {
    pub fn as_str(&self) -> String {
        match self {
            RmrkError::AddingPendingAsset => String::from("AddingPendingAsset"),
            RmrkError::AddingPendingChild => String::from("AddingPendingChild"),
            RmrkError::AddressNotEquippable => String::from("AddressNotEquippable"),
            RmrkError::AlreadyAddedAsset => String::from("AlreadyAddedAsset"),
            RmrkError::AlreadyAddedChild => String::from("AlreadyAddedChild"),
            RmrkError::AssetHasNoParts => String::from("AssetHasNoParts"),
            RmrkError::AssetIdAlreadyExists => String::from("AssetIdAlreadyExists"),
            RmrkError::AssetIdNotFound => String::from("AssetIdNotFound"),
            RmrkError::BadConfig => String::from("BadConfig"),
            RmrkError::BadMintValue => String::from("BadMintValue"),
            RmrkError::BadPriorityLength => String::from("BadPriorityLength"),
            RmrkError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            RmrkError::ChildNotFound => String::from("ChildNotFound"),
            RmrkError::CollectionIsFull => String::from("CollectionIsFull"),
            RmrkError::InvalidAssetId => String::from("InvalidAssetId"),
            RmrkError::InvalidParentId => String::from("InvalidParentId"),
            RmrkError::InvalidTokenId => String::from("InvalidTokenId"),
            RmrkError::NotEquipped => String::from("NotEquipped"),
            RmrkError::NotTokenOwner => String::from("NotTokenOwner"),
            RmrkError::PartIsNotSlot => String::from("PartIsNotSlot"),
            RmrkError::SlotAlreayUsed => String::from("SlotAlreayUsed"),
            RmrkError::TargetAssetCannotReceiveSlot => String::from("TargetAssetCannotReceiveSlot"),
            RmrkError::UnknownEquippableAsset => String::from("UnknownEquippableAsset"),
            RmrkError::UnknownPart => String::from("UnknownPart"),
            RmrkError::UnknownPartId => String::from("UnknownPartId"),
            RmrkError::WithdrawalFailed => String::from("WithdrawalFailed"),
        }
    }
}
