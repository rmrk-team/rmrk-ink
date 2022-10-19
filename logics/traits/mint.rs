use openbrush::{
    // contracts::{
    //     reentrancy_guard::*,
    //     traits::{ownable::*, pausable::*, psp34::PSP34Error},
    // },
    modifiers,
    traits::AccountId,
};

use crate::traits::errors::RmrkError;

#[openbrush::wrapper]
pub type RMRKMintableRef = dyn RMRKMintable;

#[openbrush::trait_definition]
pub trait RMRKMintable {
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn mint_multiple(&mut self, owner: AccountId, amount: u16) -> Result<(), RmrkError>;

    // fn nft_mint_directly_to_nft(&self, parent: AccountIdOrCollectionNftTuple) -> Result<(), RmrkError>;
}
