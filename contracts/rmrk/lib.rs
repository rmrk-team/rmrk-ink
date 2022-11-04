#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_contract {
    // imports from ink!
    use ink_env;
    use ink_prelude::string::{String, ToString};
    use ink_prelude::vec::Vec;
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
    use uniques_extension::*;

    // set CollectionDeposit to the value defined in the node runtime
    pub const COLLECTION_DEPOSIT: Balance = 10 * 1_000_000_000_000_000;

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
            tmp_collection_id: u32,
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
                _instance.minting.price_per_mint = _price_per_mint;


                // assert!(_instance.env().transferred_value() >= COLLECTION_DEPOSIT);
                _instance.minting.rmrk_collection_id = tmp_collection_id;
                // if let Id::Bytes(data) = collection_id {
                    // let collection = u32::from_le_bytes(data[0..4].try_into().unwrap());
                    // _instance.minting.rmrk_collection_id = collection.clone(); TODO use this after uniques supports collection_id as input
                    // TODO uncomment below code when RMRKminting::create is removed
                    // let create_result = UniquesExt::create(collection);
                    // ink_env::debug_println!(
                    //     "####### initializing RMRK contract, create_result: {:?}",
                    //     create_result
                    // );
                // }
            })
        }
    }

    impl PSP34 for Rmrk {
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
            ink_env::debug_println!(
                "####### transfer ({:?},{:?}) to:{:?}",
                self.minting.rmrk_collection_id,
                id,
                to
            );
            self._transfer_token(to, id.clone(), data)?;
            ink_env::debug_println!("####### _transfer_token OK");
            if let Id::U64(token_id) = id {
                if token_id > u32::MAX as u64 {
                    return Err(PSP34Error::Custom("TokenIdOverflow".to_string()));
                }
                // Uniques pallet is defined to take u32 as item/token id
                UniquesExt::transfer(self.minting.rmrk_collection_id, token_id as u32, to)
                    .map_err(|_| PSP34Error::Custom("UniquesTransferFailed".to_string()))?;
                ink_env::debug_println!("####### transfer OK");
                return Ok(());
            }
            ink_env::debug_println!("####### !!!!! TransferFailed");
            Err(PSP34Error::Custom("TransferFailedxxx".to_string()))
        }

        #[ink(message)]
        fn approve(
            &mut self,
            operator: AccountId,
            id: Option<Id>,
            approved: bool,
        ) -> Result<(), PSP34Error> {
            ink_env::debug_println!(
                "####### approve ({:?},{:?}) approved{:?} operator:{:?}",
                self.minting.rmrk_collection_id,
                id,
                approved,
                operator,
            );
            self._approve_for(operator, id.clone(), approved)?;
            ink_env::debug_println!("####### _approve_for OK");
            if let Some(Id::U64(token_id)) = id {
                if token_id > u32::MAX as u64 {
                    return Err(PSP34Error::Custom("TokenIdOverflow".to_string()));
                }
                // Uniques pallet is defined to take u32 as item/token id
                UniquesExt::approve_transfer(
                    self.minting.rmrk_collection_id,
                    token_id as u32,
                    operator,
                )
                .map_err(|_| PSP34Error::Custom("UniquesApproveFailed".to_string()))?;
                ink_env::debug_println!("####### approve OK");
                return Ok(());
            }
            ink_env::debug_println!("####### !!!!! ApproveFailed");
            Err(PSP34Error::Custom("ApproveFailed".to_string()))
        }
    }
}
