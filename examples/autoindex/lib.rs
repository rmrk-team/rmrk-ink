#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_example_equippable {
    use ink::{
        codegen::{
            Env,
        },
        prelude::{
            vec,
        },
    };
    use openbrush::{
        contracts::{
            access_control::*,
            psp34::extensions::{
                enumerable::*,
                metadata::*,
            },
            reentrancy_guard::*,
        },
        traits::{
            Storage,
            String,
        },
    };

    use rmrk::{
        config,
        errors::Result,
        extensions::{
            MintingAutoIndexData,
            MultiAssetAutoIndexData,
            *,
        },
        query::*,
        storage::*,
        traits::*,
    };

    /// Event emitted when a token transfer occurs.
    // Rmrk contract storage
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Rmrk {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        multiasset: MultiAssetData,
        #[storage_field]
        minting: MintingData,
        #[storage_field]
        multiasset_autoindex: MultiAssetAutoIndexData,
        #[storage_field]
        minting_autoindex: MintingAutoIndexData,
    }

    impl PSP34 for Rmrk {}

    impl AccessControl for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl Minting for Rmrk {}

    impl MintingAutoIndex for Rmrk {}

    impl MultiAsset for Rmrk {}

    impl MultiAssetAutoIndex for Rmrk {}

    impl Query for Rmrk {}

    impl Rmrk {
        /// Instantiate new RMRK contract
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: Option<u64>,
            collection_metadata: String,
        ) -> Self {
            let mut instance = Rmrk::default();
            config::with_admin(&mut instance, Self::env().caller());
            config::with_collection(
                &mut instance,
                name,
                symbol,
                base_uri,
                collection_metadata,
                max_supply,
            );
            instance
        }

        #[ink(message)]
        pub fn mint_autoindex(&mut self) -> Result<()> {
            let _token_id = MintingAutoIndex::mint(self, self.env().caller())?;
            Ok(())
        }

        #[ink(message)]
        pub fn add_asset_autoindex(&mut self) -> Result<()> {
            let _asset_id = MultiAssetAutoIndex::add_asset_entry(
                self,
                None,
                0,
                String::from("ipfs://"),
                vec![],
            )?;
            Ok(())
        }
    }
}
