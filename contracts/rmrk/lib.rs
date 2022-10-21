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
        minting: mint_data::Data,
        #[storage_field]
        metadata: metadata::Data,
        max_supply: u32,
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
            max_supply: u32,
            _price_per_mint: Balance,
            collection_metadata: String,
            token_uri: String,
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
                // _instance._set_attribute(
                //     collection_id.clone(),
                //     String::from("price").into_bytes(),
                //     price_per_mint.into_bytes(),
                // );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("tokenUri").into_bytes(),
                    String::from(token_uri).into_bytes(),
                );
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("collection_metadata").into_bytes(),
                    String::from(collection_metadata).into_bytes(),
                );
                _instance.max_supply = max_supply;
            })
        }
    }
}
