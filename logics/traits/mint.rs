use crate::traits::errors::RmrkError;
// use openbrush::contracts::psp34::*;
use openbrush::{modifiers, traits::AccountId};

#[openbrush::wrapper]
pub type RmrkMintableRef = dyn RmrkMintable;

#[openbrush::trait_definition]
pub trait RmrkMintable {
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn mint(&mut self, to: AccountId, mint_amount: u128) -> Result<(), RmrkError>;

    // fn nft_mint_directly_to_nft(&self, parent: AccountIdOrCollectionNftTuple) -> Result<(), RmrkError>;
}
