#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod rmrk_example_equippable {
    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        prelude::vec::Vec,
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
        query::*,
        storage::*,
        traits::*,
        types::*,
    };

    // Catalog contract storage
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Catalog {
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        base: BaseData,
    }


    impl Base for Catalog {}


    impl Catalog {
        /// Instantiate new Catalog contract
        #[ink(constructor)]
        pub fn new(
            catalog_metadata: String,
        ) -> Self {
            let mut instance = Catalog::default();
            let admin = Self::env().caller();
            instance._init_with_admin(admin);
            instance._setup_role(CONTRIBUTOR, admin);
            Self::setup_base(catalog_metadata);

            instance
        }
    }



    #[cfg(test)]
    mod tests {
        use super::{
            Environment,
            Catalog,
        };

        use ink::env::test;

        use openbrush::{
            contracts::{
                access_control::AccessControlError::*,
                psp34::PSP34Error,
            },
            traits::AccountId,
        };
        use rmrk::{
            errors::*,
            roles::ADMIN,
            traits::{
                Base,
            },
            types::*,
        };

        const METADATA: &str = "ipfs://myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;

        #[ink::test]
        fn init_works() {
            let catalog = init();

            assert_eq!(
                catalog.get_base_metadata(),
                Some(String::from(METADATA))
            );
            assert!(catalog.has_role(ADMIN, accounts.alice));
        }

        fn init() -> Catalog {
            Catalog::new(
                String::from(METADATA)
            )
        }

        #[ink::test]
        fn add_parts_works() {
            let catalog = init(METADATA);

            set_sender(accounts.alice);
            let collection_id = rmrk.collection_id();
            assert!(rmrk.set_base_uri(NEW_BASE_URI.into()).is_ok());
            assert_eq!(
                rmrk.get_attribute(collection_id, String::from("baseUri")),
                Some(String::from(NEW_BASE_URI))
            );
            set_sender(accounts.bob);
            assert_eq!(
                rmrk.set_base_uri(NEW_BASE_URI.into()),
                Err(MissingRole.into())
            );
        }
    }
}
