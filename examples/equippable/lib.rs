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

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    /// Event emitted when a new child is added.
    #[ink(event)]
    pub struct ChildAdded {
        #[ink(topic)]
        to: Id,
        #[ink(topic)]
        collection: AccountId,
        #[ink(topic)]
        child: Id,
    }

    /// Event emitted when a child is accepted.
    #[ink(event)]
    pub struct ChildAccepted {
        #[ink(topic)]
        parent: Id,
        #[ink(topic)]
        collection: AccountId,
        #[ink(topic)]
        child: Id,
    }

    /// Event emitted when a child is removed.
    #[ink(event)]
    pub struct ChildRemoved {
        #[ink(topic)]
        parent: Id,
        #[ink(topic)]
        child_collection: AccountId,
        #[ink(topic)]
        child_token_id: Id,
    }

    /// Event emitted when a child is rejected.
    #[ink(event)]
    pub struct ChildRejected {
        #[ink(topic)]
        parent: Id,
        #[ink(topic)]
        child_collection: AccountId,
        #[ink(topic)]
        child_token_id: Id,
    }

    /// Event emitted when new asset is set for the collection.
    #[ink(event)]
    pub struct AssetSet {
        #[ink(topic)]
        asset: AssetId,
    }
    /// Event emitted when the asset is added to the token.
    #[ink(event)]
    pub struct AssetAddedToToken {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
        #[ink(topic)]
        replaces: Option<AssetId>,
    }

    /// Event emitted when the asset is accepted.
    #[ink(event)]
    pub struct AssetAccepted {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
    }

    /// Event emitted when the asset is rejected.
    #[ink(event)]
    pub struct AssetRejected {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
    }

    /// Event emitted when the asset is removed.
    #[ink(event)]
    pub struct AssetRemoved {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
    }

    /// Event emitted when the asset is removed.
    #[ink(event)]
    pub struct AssetPrioritySet {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        priorities: Vec<AssetId>,
    }

    /// Event emitted when the asset is equipped.
    #[ink(event)]
    pub struct AssetEquipped {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
        #[ink(topic)]
        child: Id,
        #[ink(topic)]
        child_asset: AssetId,
    }

    /// Event emitted when the asset is un-equipped.
    #[ink(event)]
    pub struct AssetUnEquipped {
        #[ink(topic)]
        token: Id,
        #[ink(topic)]
        asset: AssetId,
        #[ink(topic)]
        slot: SlotId,
    }

    /// Used to notify listeners that the assets belonging to a `equippableGroupId` have been marked as
    /// equippable into a given slot and parent
    #[ink(event)]
    pub struct ParentEquippableGroupSet {
        #[ink(topic)]
        group: EquippableGroupId,
        #[ink(topic)]
        slot: SlotId,
        #[ink(topic)]
        parent: AccountId,
    }

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
        nesting: NestingData,
        #[storage_field]
        multiasset: MultiAssetData,
        #[storage_field]
        minting: MintingData,
        #[storage_field]
        equippable: EquippableData,
    }

    impl PSP34 for Rmrk {}

    impl AccessControl for Rmrk {}

    impl PSP34Metadata for Rmrk {}

    impl PSP34Enumerable for Rmrk {}

    impl Minting for Rmrk {}

    impl Nesting for Rmrk {}

    impl MultiAsset for Rmrk {}

    impl Equippable for Rmrk {}

    impl Query for Rmrk {}

    impl Rmrk {
        /// Instantiate new RMRK contract
        #[allow(clippy::too_many_arguments)]
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
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
    }

    impl psp34::Internal for Rmrk {
        /// Emit Transfer event
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        /// Emit Approval event
        fn _emit_approval_event(
            &self,
            from: AccountId,
            to: AccountId,
            id: Option<Id>,
            approved: bool,
        ) {
            self.env().emit_event(Approval {
                from,
                to,
                id,
                approved,
            });
        }
    }

    impl NestingEvents for Rmrk {
        /// Emit ChildAdded event
        fn _emit_added_child_event(&self, to: &Id, collection: &AccountId, child: &Id) {
            self.env().emit_event(ChildAdded {
                to: to.clone(),
                collection: *collection,
                child: child.clone(),
            });
        }

        /// Emit ChildAccepted event
        fn _emit_child_accepted_event(&self, parent: &Id, collection: &AccountId, child: &Id) {
            self.env().emit_event(ChildAccepted {
                parent: parent.clone(),
                collection: *collection,
                child: child.clone(),
            });
        }

        /// Emit ChildRemoved event
        fn _emit_child_removed_event(
            &self,
            parent: &Id,
            child_collection: &AccountId,
            child_token_id: &Id,
        ) {
            self.env().emit_event(ChildRemoved {
                parent: parent.clone(),
                child_collection: *child_collection,
                child_token_id: child_token_id.clone(),
            });
        }

        /// Emit ChildRejected event
        fn _emit_child_rejected_event(
            &self,
            parent: &Id,
            child_collection: &AccountId,
            child_token_id: &Id,
        ) {
            self.env().emit_event(ChildRejected {
                parent: parent.clone(),
                child_collection: *child_collection,
                child_token_id: child_token_id.clone(),
            });
        }
    }

    impl MultiAssetEvents for Rmrk {
        /// Used to notify listeners that an asset object is initialized at `assetId`.
        fn _emit_asset_set_event(&self, asset_id: &AssetId) {
            self.env().emit_event(AssetSet { asset: *asset_id });
        }

        /// Used to notify listeners that an asset object at `assetId` is added to token's pending asset array.
        fn _emit_asset_added_to_token_event(
            &self,
            token_id: &Id,
            asset_id: &AssetId,
            replaces_id: &Option<AssetId>,
        ) {
            self.env().emit_event(AssetAddedToToken {
                token: token_id.clone(),
                asset: *asset_id,
                replaces: *replaces_id,
            });
        }

        /// Used to notify listeners that an asset object at `assetId` is accepted by the token and migrated
        fn _emit_asset_accepted_event(&self, token_id: &Id, asset_id: &AssetId) {
            self.env().emit_event(AssetAccepted {
                token: token_id.clone(),
                asset: *asset_id,
            });
        }

        /// Used to notify listeners that an asset object at `assetId` is rejected from token and is dropped from the pending assets array of the token.
        fn _emit_asset_rejected_event(&self, token_id: &Id, asset_id: &AssetId) {
            self.env().emit_event(AssetRejected {
                token: token_id.clone(),
                asset: *asset_id,
            });
        }

        /// Used to notify listeners that an asset object at `assetId` is removed from token
        fn _emit_asset_removed_event(&self, token_id: &Id, asset_id: &AssetId) {
            self.env().emit_event(AssetRemoved {
                token: token_id.clone(),
                asset: *asset_id,
            });
        }

        /// Used to notify listeners that token's prioritiy array is reordered.
        fn _emit_asset_priority_set_event(&self, token_id: &Id, priorities: Vec<AssetId>) {
            self.env().emit_event(AssetPrioritySet {
                token: token_id.clone(),
                priorities,
            });
        }
    }

    impl EquippableEvents for Rmrk {
        /// Used to notify listeners that a child's asset has been equipped into one of its parent assets.
        fn emit_child_asset_equipped(
            &self,
            token_id: Id,
            asset_id: AssetId,
            _slot_part_id: PartId,
            child_nft: ChildNft,
            child_asset_id: AssetId,
        ) {
            self.env().emit_event(AssetEquipped {
                token: token_id,
                asset: asset_id,
                child: child_nft.1,
                child_asset: child_asset_id,
            });
        }

        /// Used to notify listeners that a child's asset has been un-equipped from one of its parent assets.
        fn emit_child_asset_unequipped(
            &self,
            token_id: Id,
            asset_id: AssetId,
            slot_part_id: PartId,
        ) {
            self.env().emit_event(AssetUnEquipped {
                token: token_id,
                asset: asset_id,
                slot: slot_part_id,
            });
        }

        /// Used to notify listeners that the assets belonging to a `equippableGroupId` have been marked as
        /// equippable into a given slot and parent
        fn emit_valid_parent_equippable_group_set(
            &self,
            group_id: EquippableGroupId,
            slot_part_id: PartId,
            parent_address: AccountId,
        ) {
            self.env().emit_event(ParentEquippableGroupSet {
                group: group_id,
                slot: slot_part_id,
                parent: parent_address,
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{
            Environment,
            Rmrk,
        };

        use openbrush::{
            contracts::{
                access_control::{
                    AccessControlError::*,
                    *,
                },
                psp34::extensions::{
                    enumerable::*,
                    metadata::*,
                },
            },
            traits::{
                AccountId,
                String,
            },
        };

        use ink::env::test;

        use rmrk::{
            roles::ADMIN,
            traits::Minting,
            utils::Utils,
        };

        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;

        #[ink::test]
        fn init_works() {
            let rmrk = init();
            let collection_id = rmrk.collection_id();
            assert_eq!(
                rmrk.get_attribute(collection_id.clone(), String::from("name")),
                Some(String::from("Rmrk Project"))
            );
            assert_eq!(
                rmrk.get_attribute(collection_id.clone(), String::from("symbol")),
                Some(String::from("RMK"))
            );
            assert_eq!(
                rmrk.get_attribute(collection_id, String::from("baseUri")),
                Some(String::from(BASE_URI))
            );
            assert_eq!(rmrk.max_supply(), MAX_SUPPLY);
        }

        fn init() -> Rmrk {
            Rmrk::new(
                String::from("Rmrk Project"),
                String::from("RMK"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                String::from(BASE_URI),
            )
        }

        #[ink::test]
        fn owner_is_set() {
            let accounts = default_accounts();
            let rmrk = init();
            assert!(rmrk.has_role(ADMIN, accounts.alice));
        }

        #[ink::test]
        fn set_base_uri_works() {
            let accounts = default_accounts();
            const NEW_BASE_URI: &str = "new_uri/";
            let mut rmrk = init();

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

        fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }
    }
}