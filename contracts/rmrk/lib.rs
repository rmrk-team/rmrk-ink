#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_contract {
    // imports from ink!
    use ink_env;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    // imports from openbrush
    use openbrush::{
        contracts::{ownable::*, psp34::extensions::metadata::*, reentrancy_guard::*},
        // modifiers,
        traits::Storage,
    };
    // local imports
    use rmrk::impls::rmrk::*;
    use rmrk::traits::mint::*;
    // chain extension for pallet_uniques
    // use uniques_extension::*;

    // set CollectionDeposit to the value defined for the node runtime
    pub const CollectionDeposit: Balance = 10 * 1_000_000_000_000_000;

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
    impl PSP34 for Rmrk {}
    impl Ownable for Rmrk {}
    impl PSP34Metadata for Rmrk {}
    impl RmrkMintable for Rmrk {}

    impl Rmrk {
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            max_supply: u64,
            _price_per_mint: Balance,
            collection_metadata: String,
            base_uri: String,
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
                    String::from(name).into_bytes(),
                );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("symbol").into_bytes(),
                    String::from(symbol).into_bytes(),
                );

                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("baseUri").into_bytes(),
                    String::from(base_uri).into_bytes(),
                );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("collection_metadata").into_bytes(),
                    String::from(collection_metadata).into_bytes(),
                );
                _instance.minting.max_supply = max_supply;
                _instance.minting.price_per_mint = _price_per_mint;
            })
        }
    }
}
