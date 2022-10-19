#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk {
    // imports from ink!
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    // imports from openbrush
    use openbrush::{
        contracts::{
            ownable::*, psp34::extensions::metadata::*, psp34::extensions::mintable::*,
            reentrancy_guard::*,
        },
        modifiers,
        traits::Storage,
    };
    // local imports
    // use rmrk::traits::mint::RMRKMintable;
    // use rmrk::traits::errors::RmrkError;
    // use rmrk::impls::*;

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
        metadata: metadata::Data,
    }

    // Section contains default implementation without any modifications
    impl PSP34 for Rmrk {}
    impl Ownable for Rmrk {}
    impl PSP34Mintable for Rmrk {
        #[ink(message)]
        #[modifiers(non_reentrant)]
        fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._mint_to(account, id)
        }
    }
    impl PSP34Metadata for Rmrk {}
    // impl RMRKMintable for Rmrk {}

    impl Rmrk {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Rmrk| {
                _instance._init_with_owner(_instance.env().caller());
                _instance
                    ._mint_to(_instance.env().caller(), Id::U8(1))
                    .expect("Can mint");
                let collection_id = _instance.collection_id();
                _instance._set_attribute(
                    collection_id.clone(),
                    String::from("name").into_bytes(),
                    String::from("MyPSP34").into_bytes(),
                );
                _instance._set_attribute(
                    collection_id,
                    String::from("symbol").into_bytes(),
                    String::from("MPSP").into_bytes(),
                );
            })
        }
    }
}
