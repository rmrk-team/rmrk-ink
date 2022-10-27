#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{
    // AccountId,
    DefaultEnvironment,
    Environment,
};
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

    // pub fn read_unbonding_period() -> u32 {
    //     ::ink_env::chain_extension::ChainExtensionMethod::build(0002u32)
    //         .input::<()>()
    //         .output::<u32>()
    //         .ignore_error_code()
    //         .call(&())
    // }

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
