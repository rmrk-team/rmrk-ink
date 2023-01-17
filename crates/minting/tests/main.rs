#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_contract_minting {

    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{
                enumerable::*,
                metadata::*,
            },
            reentrancy_guard::*,
        },
        traits::Storage,
    };

    use rmrk_minting::{
        traits::*,
        MintingData,
    };

    // Rmrk contract storage
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Rmrk {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        minting: MintingData,
    }

    impl PSP34 for Rmrk {}

    impl Ownable for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl Minting for Rmrk {}

    impl Rmrk {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Rmrk| {
                instance._init_with_owner(instance.env().caller());
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        use rmrk_common::errors::RmrkError;
        use rmrk_minting::traits::Internal;

        #[ink::test]
        fn check_supply_overflow_ok() {
            let max_supply = u64::MAX - 1;
            let mut rmrk = Rmrk::new();
            rmrk.minting.last_token_id = max_supply - 1;

            // check case when last_token_id.add(mint_amount) if more than u64::MAX
            assert_eq!(
                rmrk._check_amount(3),
                Err(PSP34Error::Custom(RmrkError::CollectionIsFull.as_str()))
            );

            // check case when mint_amount is 0
            assert_eq!(
                rmrk._check_amount(0),
                Err(PSP34Error::Custom(RmrkError::CannotMintZeroTokens.as_str()))
            );
        }

        #[ink::test]
        fn check_value_overflow_ok() {
            let rmrk = Rmrk::new();
            let transferred_value = u128::MAX;
            let mint_amount = u64::MAX;
            assert_eq!(
                rmrk._check_value(transferred_value, mint_amount),
                Err(PSP34Error::Custom(RmrkError::BadMintValue.as_str()))
            );
        }
    }
}
