//! Error definition for RMRK contract

use ink::prelude::string::{
    String,
    ToString,
};
use openbrush::contracts::{
    access_control::AccessControlError,
    psp34::PSP34Error,
    reentrancy_guard::ReentrancyGuardError,
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Rmrk(RmrkError),
    PSP34(PSP34Error),
    AccessControl(AccessControlError),
    Reentrancy(ReentrancyGuardError),
}

impl From<RmrkError> for Error {
    fn from(err: RmrkError) -> Self {
        Self::Rmrk(err)
    }
}

impl From<PSP34Error> for Error {
    fn from(err: PSP34Error) -> Self {
        Self::PSP34(err)
    }
}

impl From<AccessControlError> for Error {
    fn from(err: AccessControlError) -> Self {
        Self::AccessControl(err)
    }
}

impl From<ReentrancyGuardError> for Error {
    fn from(err: ReentrancyGuardError) -> Self {
        Self::Reentrancy(err)
    }
}

// Flatten errors to a common PSP34Error type containing a String representation
impl From<Error> for PSP34Error {
    fn from(err: Error) -> Self {
        match err {
            Error::PSP34(err) => err,
            Error::Rmrk(err) => PSP34Error::Custom(err.to_string().into()),
            Error::AccessControl(AccessControlError::InvalidCaller) => {
                PSP34Error::Custom(String::from("InvalidCaller").into())
            }
            Error::AccessControl(AccessControlError::MissingRole) => {
                PSP34Error::Custom(String::from("MissingRole").into())
            }
            Error::AccessControl(AccessControlError::RoleRedundant) => {
                PSP34Error::Custom(String::from("RoleRedundant").into())
            }

            Error::Reentrancy(ReentrancyGuardError::ReentrantCall) => {
                PSP34Error::Custom(String::from("ReentrantCall").into())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    AcceptedAssetsMissing,
    AddingPendingAsset,
    AddingPendingChild,
    AddressNotEquippable,
    AlreadyAddedAsset,
    AlreadyAddedChild,
    AssetHasNoParts,
    AssetIdAlreadyExists,
    AssetIdNotFound,
    AssetIdNotEquippable,
    BadConfig,
    BadMintValue,
    BadPriorityLength,
    CannotMintZeroTokens,
    CatalogNotFoundForAsset,
    ChildNotFound,
    UriNotFound,
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

impl ToString for RmrkError {
    fn to_string(&self) -> String {
        match self {
            RmrkError::AcceptedAssetsMissing => String::from("AcceptedAssetsMissing"),
            RmrkError::AddingPendingAsset => String::from("AddingPendingAsset"),
            RmrkError::AddingPendingChild => String::from("AddingPendingChild"),
            RmrkError::AddressNotEquippable => String::from("AddressNotEquippable"),
            RmrkError::AlreadyAddedAsset => String::from("AlreadyAddedAsset"),
            RmrkError::AlreadyAddedChild => String::from("AlreadyAddedChild"),
            RmrkError::AssetHasNoParts => String::from("AssetHasNoParts"),
            RmrkError::AssetIdAlreadyExists => String::from("AssetIdAlreadyExists"),
            RmrkError::AssetIdNotFound => String::from("AssetIdNotFound"),
            RmrkError::AssetIdNotEquippable => String::from("AssetIdNotEquippable"),
            RmrkError::BadConfig => String::from("BadConfig"),
            RmrkError::BadMintValue => String::from("BadMintValue"),
            RmrkError::BadPriorityLength => String::from("BadPriorityLength"),
            RmrkError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            RmrkError::CatalogNotFoundForAsset => String::from("CatalogNotFoundForAsset"),
            RmrkError::ChildNotFound => String::from("ChildNotFound"),
            RmrkError::UriNotFound => String::from("UriNotFound"),
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
