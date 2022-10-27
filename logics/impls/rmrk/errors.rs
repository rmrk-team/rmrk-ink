use openbrush::contracts::{
    reentrancy_guard::*,
    traits::{ownable::*, pausable::*, psp34::PSP34Error},
};
use uniques_extension::UniquesError;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    PSP34Error(PSP34Error),
    OwnableError(OwnableError),
    PausableError(PausableError),
    ReentrancyGuardError(ReentrancyGuardError),
    UniquesError(UniquesError),
    CannotMintZeroTokens,
    CollectionFullOrLocked,
    MintUnderpriced,
}

impl From<OwnableError> for RmrkError {
    fn from(error: OwnableError) -> Self {
        RmrkError::OwnableError(error)
    }
}

impl From<PausableError> for RmrkError {
    fn from(access: PausableError) -> Self {
        RmrkError::PausableError(access)
    }
}

impl From<PSP34Error> for RmrkError {
    fn from(error: PSP34Error) -> Self {
        RmrkError::PSP34Error(error)
    }
}

impl From<ReentrancyGuardError> for RmrkError {
    fn from(error: ReentrancyGuardError) -> Self {
        RmrkError::ReentrancyGuardError(error)
    }
}
