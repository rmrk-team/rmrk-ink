#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
use scale::{
    Decode,
    Encode,
    // HasCompact,
};

// type Balance = <DefaultEnvironment as Environment>::Balance;

pub struct UniquesExt;

impl UniquesExt {
    /// Create new collection in Uniques pallet
    pub fn create(collection_id: u32) -> Result<(), UniquesError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x000200A0)
            // .input::<(Vec<u8>, Option<u32>, Vec<u8>)>()
            .input::<u32>()
            .output::<()>()
            .handle_error_code::<UniquesError>()
            .call(&(collection_id))
    }

    /// Mint an item in Uniques pallet
    pub fn mint(collection_id: u32, item_id: u32, to: AccountId) -> Result<(), UniquesError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x000200A1)
            // .input::<(Vec<u8>, Option<u32>, Vec<u8>)>()
            .input::<(u32, u32, AccountId)>()
            .output::<()>()
            .handle_error_code::<UniquesError>()
            .call(&(collection_id, item_id, to))
    }

    /// Transfer an item in Uniques pallet
    pub fn transfer(collection_id: u32, item_id: u32, to: AccountId) -> Result<(), UniquesError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x000200A2)
            .input::<(u32, u32, AccountId)>()
            .output::<()>()
            .handle_error_code::<UniquesError>()
            .call(&(collection_id, item_id, to))
    }
}

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
pub enum UniquesError {
    /// Success
    Success = 0,
    /// The signing account has no permission to do the operation.
    NoPermission,
    /// The given item ID is unknown.
    UnknownCollection,
    /// The item ID has already been used for an item.
    AlreadyExists,
    /// The owner turned out to be different to what was expected.
    WrongOwner,
    /// Invalid witness data given.
    BadWitness,
    /// The item ID is already taken.
    InUse,
    /// The item or collection is frozen.
    Frozen,
    /// The delegate turned out to be different to what was expected.
    WrongDelegate,
    /// There is no delegate approved.
    NoDelegate,
    /// No approval exists that would allow the transfer.
    Unapproved,
    /// The named owner has not signed ownership of the collection is acceptable.
    Unaccepted,
    /// The item is locked.
    Locked,
    /// All items have been minted.
    MaxSupplyReached,
    /// The max supply has already been set.
    MaxSupplyAlreadySet,
    /// The provided max supply is less to the amount of items a collection already has.
    MaxSupplyTooSmall,
    /// The `CollectionId` in `NextCollectionId` is not being used.
    ///
    /// This means that you can directly proceed to call `create`.
    NextIdNotUsed,
    /// The given item ID is unknown.
    UnknownItem,
    /// Item is not for sale.
    NotForSale,
    /// The provided bid is too low.
    BidTooLow,
    /// Unknown error
    UnknownError = 99,
    UnImplemented = 100,
}

impl ink_env::chain_extension::FromStatusCode for UniquesError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::NoPermission),
            2 => Err(Self::UnknownCollection),
            3 => Err(Self::AlreadyExists),
            4 => Err(Self::WrongOwner),
            5 => Err(Self::BadWitness),
            6 => Err(Self::InUse),
            7 => Err(Self::Frozen),

            99 => Err(Self::UnknownError),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for UniquesError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
