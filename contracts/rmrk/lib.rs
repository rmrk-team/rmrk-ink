#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_contract {
    // imports from ink!
    use ink_env;
    use ink_prelude::string::String;
    // use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    // imports from openbrush
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{enumerable::*, metadata::*},
            reentrancy_guard::*,
        },
        // modifiers,
        traits::Storage,
    };
    // local imports
    use rmrk::impls::rmrk::*;
    use rmrk::traits::mint::*;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Rmrk {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        minting: data::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    // Section contains default implementation without any modifications
    impl Ownable for Rmrk {}
    impl PSP34Metadata for Rmrk {}
    impl RmrkMintable for Rmrk {}
    // impl PSP34Enumerable for Rmrk {}

    impl Rmrk {
        #[ink(constructor, payable)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            price_per_mint: Balance,
            collection_metadata: String,
            _royalty_receiver: AccountId,
            _royalty: u8,
        ) -> Self {
            ink_env::debug_println!("####### initializing RMRK contract");
            ink_lang::codegen::initialize_contract(|_instance: &mut Rmrk| {
                _instance._init_with_owner(_instance.env().caller());
                let collection_id = _instance.collection_id();
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("name").into_bytes(),
                    name.into_bytes(),
                );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("symbol").into_bytes(),
                    symbol.into_bytes(),
                );

                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("baseUri").into_bytes(),
                    base_uri.into_bytes(),
                );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("collection_metadata").into_bytes(),
                    collection_metadata.into_bytes(),
                );
                _instance.minting.max_supply = max_supply;
                _instance.minting.price_per_mint = price_per_mint;
            })
        }
    }

    impl PSP34 for Rmrk {}

    #[cfg(test)]
    mod tests {
        // use super::*;
        // // use crate::rmrk_contract::Rmrk;
        // use ink_lang as ink;
        // use ink_env::Environment;
        // use ink_env::test;

        use super::*;
        use ink_env::test;
        use ink_lang as ink;

        // use crate::rmrk_contract::PSP34Error::*;

        // use openbrush::{
        //     contracts::{
        //         psp34::{
        //             extensions::{enumerable::*, metadata::*},
        //         },
        //     },
        //     traits::{AccountId, Balance},
        // };
        // use rmrk::impls::rmrk::psp34_custom::*;

        const PRICE: Balance = 100_000_000_000_000_000;
        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;

        #[ink::test]
        fn init_works() {
            let rmrk_contract = init();
            let collection_id = rmrk_contract.collection_id();
            assert_eq!(
                rmrk_contract
                    .get_attribute(collection_id.clone(), String::from("name").into_bytes()),
                Some(String::from("Remark Project").into_bytes())
            );
            assert_eq!(
                rmrk_contract
                    .get_attribute(collection_id.clone(), String::from("symbol").into_bytes()),
                Some(String::from("RMK").into_bytes())
            );
            assert_eq!(
                rmrk_contract.get_attribute(collection_id, String::from("baseUri").into_bytes()),
                Some(String::from(BASE_URI).into_bytes())
            );
            assert_eq!(rmrk_contract.max_supply(), MAX_SUPPLY);
            // assert_eq!(rmrk_contract.price(), PRICE);
        }

        fn init() -> Rmrk {
            let accounts = default_accounts();
            Rmrk::new(
                String::from("Remark Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                PRICE,
                String::from(BASE_URI),
                accounts.eve,
                0,
            )
        }

        #[ink::test]
        fn mint_single_works() {
            let mut rmrk_contract = init();
            let accounts = default_accounts();
            assert_eq!(rmrk_contract.owner(), accounts.alice);
            set_sender(accounts.bob);

            assert_eq!(rmrk_contract.total_supply(), 0);
            test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
            assert!(rmrk_contract.mint_next().is_ok());
            assert_eq!(rmrk_contract.total_supply(), 1);
            assert_eq!(rmrk_contract.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(rmrk_contract.balance_of(accounts.bob), 1);
            // assert_eq!(
            //     rmrk_contract.owners_token_by_index(accounts.bob, 0),
            //     Ok(Id::U64(1))
            // );
            assert_eq!(rmrk_contract.minting.last_token_id, 1);
            // assert_eq!(1, ink_env::test::recorded_events().count());
        }
        fn default_accounts() -> test::DefaultAccounts<ink_env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink_env::test::set_caller::<Environment>(sender);
        }
    }
}
