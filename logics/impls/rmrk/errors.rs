//! Error definition for RMRK contract
use openbrush::traits::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    CannotMintZeroTokens,
    CollectionIsFull,
    InvalidTokenId,
    BadMintValue,
    WithdrawalFailed,
    AlreadyAddedChild,
    AddingPendingChild,
    InvalidParentId,
    ChildNotFound,
    NotAuthorised,
    InvalidAssetId,
    AssetIdAlreadyExists,
    AssetIdNotFound,
    AlreadyAddedAsset,
    AddingPendingAsset,
    BadPriorityLength,
}

impl RmrkError {
    pub fn as_str(&self) -> String {
        match self {
            RmrkError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            RmrkError::CollectionIsFull => String::from("CollectionIsFull"),
            RmrkError::InvalidTokenId => String::from("InvalidTokenId"),
            RmrkError::BadMintValue => String::from("BadMintValue"),
            RmrkError::WithdrawalFailed => String::from("WithdrawalFailed"),
            RmrkError::AlreadyAddedChild => String::from("AlreadyAddedChild"),
            RmrkError::AddingPendingChild => String::from("AddingPendingChild"),
            RmrkError::InvalidParentId => String::from("InvalidParentId"),
            RmrkError::ChildNotFound => String::from("ChildNotFound"),
            RmrkError::NotAuthorised => String::from("NotAuthorised"),
            RmrkError::InvalidAssetId => String::from("InvalidAssetId"),
            RmrkError::AssetIdAlreadyExists => String::from("AssetIdAlreadyExists"),
            RmrkError::AssetIdNotFound => String::from("AssetIdNotFound"),
            RmrkError::AlreadyAddedAsset => String::from("AlreadyAddedAsset"),
            RmrkError::AddingPendingAsset => String::from("AddingPendingAsset"),
            RmrkError::BadPriorityLength => String::from("BadPriorityLength"),
        }
    }
}
