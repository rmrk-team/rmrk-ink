//! Error definition for RMRK contract
use openbrush::traits::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    CannotMintZeroTokens,
    CollectionIsFull,
    BadMintValue,
    WithdrawalFailed,
    AlreadyAddedChild,
    AddingPendingChild,
    InvalidParentId,
    ChildNotFound,
    NotAuthorised,
}

impl RmrkError {
    pub fn as_str(&self) -> String {
        match self {
            RmrkError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            RmrkError::CollectionIsFull => String::from("CollectionIsFull"),
            RmrkError::BadMintValue => String::from("BadMintValue"),
            RmrkError::WithdrawalFailed => String::from("WithdrawalFailed"),
            RmrkError::AlreadyAddedChild => String::from("AlreadyAddedChild"),
            RmrkError::AddingPendingChild => String::from("AddingPendingChild"),
            RmrkError::InvalidParentId => String::from("InvalidParentId"),
            RmrkError::ChildNotFound => String::from("ChildNotFound"),
            RmrkError::NotAuthorised => String::from("NotAuthorised"),
        }
    }
}
