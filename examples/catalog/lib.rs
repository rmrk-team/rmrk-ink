#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod catalog_example {
    use openbrush::{
        contracts::access_control::*,
        traits::{
            Storage,
            String,
        },
    };

    use rmrk::{
        errors::Result,
        extensions::*,
        roles::*,
        storage::*,
    };

    // CatalogContract contract storage
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct CatalogContract {
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        catalog: CatalogData,
        #[storage_field]
        catalog_autoindex: CatalogAutoIndexData,
    }

    impl Catalog for CatalogContract {}

    impl CatalogAutoIndex for CatalogContract {}

    impl CatalogContract {
        /// Instantiate new CatalogContract contract
        #[ink(constructor)]
        pub fn new(catalog_metadata: String) -> Result<Self> {
            let mut instance = CatalogContract::default();
            let admin = Self::env().caller();
            instance._init_with_admin(admin);
            instance._setup_role(CONTRIBUTOR, admin);
            Catalog::set_catalog_metadata(&mut instance, catalog_metadata)?;

            Ok(instance)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink::env::test;

        use rmrk::{
            errors::*,
            types::*,
        };

        // use openbrush::contracts::psp34::extensions::enumerable::*;

        const METADATA: &str = "ipfs://myIpfsUri/";
        const EQUIPPABLE_ADDRESS1: [u8; 32] = [1; 32];
        const EQUIPPABLE_ADDRESS2: [u8; 32] = [2; 32];
        const EQUIPPABLE_ADDRESS3: [u8; 32] = [3; 32];
        const PART_ID0: PartId = 0;
        const PART_ID1: PartId = 1;

        #[ink::test]
        fn role_works() {
            let catalog = init();
            let accounts = default_accounts();
            assert!(catalog.has_role(ADMIN, accounts.alice));
        }

        fn init() -> CatalogContract {
            CatalogContract::new(String::from(METADATA).into()).expect("Contract instantiated")
        }

        #[ink::test]
        fn add_parts_to_catalog_works() {
            // const ASSET_URI: &str = "asset_uri/";
            // const ASSET_ID: AssetId = 1;
            // const TOKEN_ID1: Id = Id::U64(1);
            // const TOKEN_ID2: Id = Id::U64(2);

            // Create 2 parts,
            // The first is equippable and can accept 2 equipment from 2 contracts
            // The second is fixed and can't accept any equipment
            let part_list = vec![
                Part {
                    part_type: PartType::Slot,
                    z: 0,
                    equippable: vec![EQUIPPABLE_ADDRESS1.into(), EQUIPPABLE_ADDRESS2.into()],
                    part_uri: String::from("ipfs://backgrounds/1.svg"),
                    is_equippable_by_all: false,
                },
                Part {
                    part_type: PartType::Fixed,
                    z: 0,
                    equippable: vec![],
                    part_uri: String::from("ipfs://backgrounds/2.svg"),
                    is_equippable_by_all: false,
                },
            ];

            let mut catalog = init();

            let part_ids = vec![PART_ID0, PART_ID1];

            // verify add/get parts
            assert!(catalog.get_parts_count() == 0);
            assert!(
                Catalog::add_part_list(&mut catalog, part_ids.clone(), part_list.clone()).is_ok()
            );

            assert_eq!(
                Catalog::add_part_list(&mut catalog, vec![], part_list.clone()),
                Err(RmrkError::BadConfig.into())
            );
            assert_eq!(
                Catalog::add_part_list(&mut catalog, part_ids, vec![]),
                Err(RmrkError::BadConfig.into())
            );
            assert_eq!(catalog.get_parts_count(), part_list.len() as u32);
            assert_eq!(catalog.get_part(PART_ID0).unwrap().z, part_list[0].z);
            assert_eq!(
                catalog.get_part(PART_ID0).unwrap().part_uri,
                part_list[0].part_uri
            );
            assert_eq!(
                catalog.get_part(PART_ID0).unwrap().part_uri,
                part_list[0].part_uri
            );
            assert_eq!(catalog.get_part(PART_ID0).unwrap().equippable.len(), 2);
            assert_eq!(catalog.get_part(PART_ID1).unwrap().equippable.len(), 0);

            // verify array of equippable addresses
            assert!(catalog
                .ensure_equippable(PART_ID0, EQUIPPABLE_ADDRESS1.into())
                .is_ok());
            assert!(catalog
                .ensure_equippable(PART_ID0, EQUIPPABLE_ADDRESS2.into())
                .is_ok());
            assert!(catalog
                .ensure_equippable(PART_ID1, EQUIPPABLE_ADDRESS2.into())
                .is_err());

            // verify setting and resetting equippable list
            assert!(!catalog.is_equippable_by_all(PART_ID0));
            assert!(catalog.set_equippable_by_all(PART_ID0).is_ok());
            assert!(catalog.is_equippable_by_all(PART_ID0));
            assert!(!catalog.is_equippable_by_all(42));
            assert!(catalog.reset_equippable_addresses(PART_ID0).is_ok());
            assert!(!catalog.is_equippable_by_all(PART_ID0));
            assert!(catalog
                .ensure_equippable(PART_ID0, EQUIPPABLE_ADDRESS1.into())
                .is_err());
            assert!(catalog
                .add_equippable_addresses(
                    PART_ID0,
                    vec![EQUIPPABLE_ADDRESS1.into(), EQUIPPABLE_ADDRESS2.into()]
                )
                .is_ok());
            assert!(catalog
                .ensure_equippable(PART_ID0, EQUIPPABLE_ADDRESS1.into())
                .is_ok());
            assert_eq!(
                catalog.add_equippable_addresses(PART_ID1, vec![EQUIPPABLE_ADDRESS1.into()]),
                Err(RmrkError::PartIsNotSlot.into())
            );
            assert_eq!(
                catalog.reset_equippable_addresses(PART_ID1),
                Err(RmrkError::PartIsNotSlot.into())
            );
            assert_eq!(
                catalog.set_equippable_by_all(PART_ID1),
                Err(RmrkError::PartIsNotSlot.into())
            );

            assert!(catalog
                .ensure_equippable(PART_ID0, EQUIPPABLE_ADDRESS3.into())
                .is_err());
        }

        #[ink::test]
        fn test_bad_configuration() {
            let mut catalog = init();

            // Create 2 part lists. Both have 1 part and are invalid,
            let bad_part_list1 = vec![Part {
                part_type: PartType::Fixed,
                z: 0,
                equippable: vec![EQUIPPABLE_ADDRESS1.into()],
                part_uri: String::from("ipfs://backgrounds/2.svg"),
                is_equippable_by_all: false,
            }];
            let bad_part_list2 = vec![Part {
                part_type: PartType::Fixed,
                z: 0,
                equippable: vec![],
                part_uri: String::from("ipfs://backgrounds/2.svg"),
                is_equippable_by_all: true,
            }];

            let part_ids = vec![PART_ID0, PART_ID1];

            assert_eq!(
                Catalog::add_part_list(&mut catalog, part_ids.clone(), bad_part_list1.clone()),
                Err(RmrkError::BadConfig.into())
            );
            assert_eq!(
                Catalog::add_part_list(&mut catalog, part_ids.clone(), bad_part_list2.clone()),
                Err(RmrkError::BadConfig.into())
            );
        }

        #[ink::test]
        fn autoindex_works() {
            let mut catalog = init();

            let part_list = vec![
                Part {
                    part_type: PartType::Slot,
                    z: 0,
                    equippable: vec![EQUIPPABLE_ADDRESS1.into(), EQUIPPABLE_ADDRESS2.into()],
                    part_uri: String::from("ipfs://backgrounds/1.svg"),
                    is_equippable_by_all: false,
                },
                Part {
                    part_type: PartType::Fixed,
                    z: 0,
                    equippable: vec![],
                    part_uri: String::from("ipfs://backgrounds/2.svg"),
                    is_equippable_by_all: false,
                },
                Part {
                    part_type: PartType::Fixed,
                    z: 0,
                    equippable: vec![],
                    part_uri: String::from("ipfs://backgrounds/3.svg"),
                    is_equippable_by_all: false,
                },
            ];

            assert_eq!(
                CatalogAutoIndex::add_part_list(&mut catalog, part_list.clone()),
                Ok((1, 3))
            );

            assert_eq!(catalog.get_parts_count(), part_list.len() as u32);

            assert_eq!(
                CatalogAutoIndex::add_part_list(&mut catalog, part_list.clone()),
                Ok((4, 6))
            );

            assert_eq!(
                CatalogAutoIndex::add_part_list(&mut catalog, vec![]),
                Err(RmrkError::BadConfig.into())
            );

        }

        #[ink::test]
        fn setting_metadata_works() {
            let mut catalog = init();

            assert_eq!(catalog.get_catalog_metadata(), Ok(METADATA.to_string()));
            assert!(catalog
                .set_catalog_metadata(String::from("ipfs://catalog_metadata2"))
                .is_ok());
            assert_eq!(
                catalog.get_catalog_metadata(),
                Ok("ipfs://catalog_metadata2".to_string())
            );
        }

        fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }
    }
}
