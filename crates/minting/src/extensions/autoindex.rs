use crate::{
    internal::Internal,
    traits::MintingAutoIndex,
};

use ink::prelude::vec;

use openbrush::{
    contracts::{
        access_control::{
            self,
            only_role,
        },
        psp34::extensions::enumerable::*,
    },
    modifiers,
    traits::{
        AccountId,
        Storage,
    },
};
use rmrk_common::{
    counter::Counter,
    errors::{
        Result,
        RmrkError,
    },
    roles::CONTRIBUTOR,
};

pub const STORAGE_MINTING_AUTOINDEX: u32 = openbrush::storage_unique_key!(MintingAutoIndex);

/// Storage for AutoIndex `Id` counter
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MINTING_AUTOINDEX)]
pub struct MintingAutoIndexData {
    pub token_id: Counter<Id>,
}

impl<T> MintingAutoIndex for T
where
    T: Storage<access_control::Data>
        + Storage<MintingAutoIndexData>
        + Storage<psp34::Data<enumerable::Balances>>
        + psp34::Internal
        + Internal,
{
    /// Mint one token to the specified account, with auto-generated Id
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn mint(&mut self, to: AccountId) -> Result<Id> {
        MintingAutoIndexInternal::mint(self, to)
    }

    /// Mint one or more tokens to the specified account, with auto-generated Ids
    /// The returned range provides the first and last generated Id
    #[modifiers(only_role(CONTRIBUTOR))]
    default fn mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)> {
        MintingAutoIndexInternal::mint_many(self, to, mint_amount)
    }
}

pub trait MintingAutoIndexInternal {
    /// Mint one token to the specified account, with auto-generated Id
    fn mint(&mut self, to: AccountId) -> Result<Id>;

    /// Mint one or more tokens to the specified account, with auto-generated Ids
    fn mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)>;
}

impl<T> MintingAutoIndexInternal for T
where
    T: Storage<MintingAutoIndexData>
        + Storage<psp34::Data<enumerable::Balances>>
        + psp34::Internal
        + Internal,
{
    /// Mint one token to the specified account, with auto-generated Id
    fn mint(&mut self, to: AccountId) -> Result<Id> {
        let next_id = self.data::<MintingAutoIndexData>().token_id.next()?;
        self._check_amount(1)?;
        self._mint_to(to, next_id.clone())?;
        Ok(next_id)
    }

    /// Mint one or more tokens to the specified account, with auto-generated Ids
    /// The returned range provides the first and last generated Id
    fn mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)> {
        let mut token_ids = vec![];

        for i in 0..mint_amount {
            let next_id = <Self as MintingAutoIndexInternal>::mint(self, to)?;

            if i == 0 || i == (mint_amount - 1) {
                token_ids.push(next_id.clone());
            }
        }

        let range_start = token_ids.first().ok_or(RmrkError::BadConfig)?.clone();
        let range_end = token_ids.last().ok_or(RmrkError::BadConfig)?.clone();

        Ok((range_start, range_end))
    }
}
