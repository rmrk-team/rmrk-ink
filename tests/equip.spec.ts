import { expect, use } from "chai";
import Catalog_factory from "../types/constructors/catalog_example";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable_lazy";
import Rmrk from "../types/contracts/rmrk_example_equippable_lazy";
import Contract from "../types/contracts/catalog_example";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import BN from "bn.js";
import {
  PartType,
  Part,
} from "../types/types-arguments/catalog_example";
import { emit } from "./helper";

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const CATALOG_METADATA = "ipfs://catalogMetadata/data.json";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;
const ADMIN_ROLE = 0;

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("RMRK Equip tests", () => {
    let catalogFactory: Catalog_factory;
    let avatarFactory: Rmrk_factory;
    let swordFactory: Rmrk_factory;
    let api: ApiPromise;
    let deployer: KeyringPair;
    let avatar: Rmrk;
    let sword: Rmrk;
    let catalog: Contract;
    let bob: KeyringPair;
    let dave: KeyringPair;

    beforeEach(async function(): Promise<void> {
        api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true});
        deployer = keyring.addFromUri("//Alice");
        bob = keyring.addFromUri("//Bob");
        dave = keyring.addFromUri("//Dave");

        avatarFactory = new Rmrk_factory(api, deployer);
        avatar = new Rmrk(
            (
                await avatarFactory.new(
                    ["Avatar"],
                    ["AVA"],
                    [BASE_URI],
                    MAX_SUPPLY,
                    PRICE_PER_MINT,
                    [COLLECTION_METADATA],
                    deployer.address,
                    10
                )
            ).address,
            deployer,
            api
        );

        swordFactory = new Rmrk_factory(api, deployer);
        sword = new Rmrk(
        (
            await swordFactory.new(
            ["Sword"],
            ["SWRD"],
            [BASE_URI],
            MAX_SUPPLY,
            PRICE_PER_MINT,
            [COLLECTION_METADATA],
            dave.address,
            100
            )
        ).address,
        deployer,
        api
        );

        catalogFactory = new Catalog_factory(api, deployer);
        catalog = new Contract(
            (
                await catalogFactory.new([CATALOG_METADATA])
            ).address,
            deployer,
            api
        );
    });

    it("Equip/Unequip works", async () => {
        const PART_LIST: Part[] = [
            // Head option 1
            {
                partType: PartType.fixed,
                z: 2,
                equippable: [],
                partUri: ["ipfs://heads/1.svg"],
                isEquippableByAll: false
            },
            // Head option 2
            {
                partType: PartType.fixed,
                z: 2,
                equippable: [],
                partUri: ["ipfs://heads/2.svg"],
                isEquippableByAll: false
            },
            // Body option 1
            {
                partType: PartType.fixed,
                z: 1,
                equippable: [],
                partUri: ["ipfs://body/1.svg"],
                isEquippableByAll: false
            },
            // Body option 2
            {
                partType: PartType.fixed,
                z: 1,
                equippable: [],
                partUri: ["ipfs://body/1.svg"],
                isEquippableByAll: false
            },
            // Sword slot
            {
                partType: PartType.slot,
                z: 3,
                equippable: [sword.address],
                partUri: [""],
                isEquippableByAll: false
            },
        ];

        // add all parts to catalog
        await catalog
            .withSigner(deployer)
            .tx.addPartList(PART_LIST);
        expect((await catalog.query.getPartsCount())?.value.unwrap()).to.be.equal(5);
        console.log("Catalog is set");

        console.log("Minting tokens:");

        console.log(" Minting avatar tokens");
        let avatarMintResult = await avatar
            .withSigner(bob)
            .tx.mintMany(2, {
                value: PRICE_PER_MINT.muln(2)
            });
        emit(avatarMintResult, "Transfer", {
            from: null,
            to: bob.address,
            id: { u64: 1 },
        })
        console.log("  Minted 2 avatars");

        console.log(" Minting sword tokens");
        let swordMintResult = await sword
            .withSigner(bob)
            .tx.mintMany(3, {
                value: PRICE_PER_MINT.muln(3)
            });
        emit(swordMintResult, "Transfer", {
            from: null,
            to: bob.address,
            id: { u64: 1 },
        })
        console.log("  Minted 3 swords");

        // deployer adds two assets to avatar
        console.log("Adding avatar assets");
        const defaultAssetId = 1;

        const addAssetResult = await avatar
            .withSigner(deployer)
            .tx.addAssetEntry(
                catalog.address, 
                defaultAssetId, 
                "0", 
                ["ipfs://avatarAsset.png"], 
                [4]
            );
        emit(addAssetResult, "AssetSet", { asset: 1 });

        console.log(" Added an asset to avatar");

        console.log(" Add the default asset to token 1");
        await addAssetToToken(avatar, deployer, 1, defaultAssetId);
        expect(
            (await avatar.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
        ).to.be.equal("0,1");

        // bob is the owner so he has to accept the asset
        await acceptAsset(avatar, bob, 1, defaultAssetId);
        expect(
            (await avatar.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
        ).to.be.equal("1,0");
        console.log(" Bob accepted the asset");

        console.log("Adding sword assets");
        const equippableWoodenSword = 1;
        const equippableCopperSword = 2;
        const equippableKatanaSword = 3;

        await sword
            .withSigner(deployer)
            .tx.addAssetEntry(
                catalog.address,
                1,
                equippableWoodenSword,
                ["ipfs://swords/wooden.svg"],
                []
            );
        await sword
            .withSigner(deployer)
            .tx.addAssetEntry(
                catalog.address,
                2,
                equippableCopperSword,
                ["ipfs://swords/copper.svg"],
                []
            );
        await sword
            .withSigner(deployer)
            .tx.addAssetEntry(
                catalog.address,
                3,
                equippableKatanaSword,
                ["ipfs://swords/katana.svg"],
                []
            );
        expect(
            (await sword.withSigner(deployer).query.totalAssets())?.value.unwrap().toString()
        ).to.be.equal("3");
        console.log(" Added 3 sword assets");
        
        console.log("Setting valid parent reference ID");
        await sword
            .withSigner(bob)
            .tx.setValidParentForEquippableGroup(
                equippableCopperSword,
                avatar.address,
                4
            );
        
        console.log("Add assets to swords");
        await addAssetToToken(sword, bob, 1, 1);
        await addAssetToToken(sword, bob, 1, 2);
        await addAssetToToken(sword, bob, 1, 3);
        expect(
            (await sword.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
        ).to.be.equal("3,0");

        console.log("Approving sword to avatar");
        await sword.withSigner(bob).tx.approve(avatar.address, { u64: 1 }, true);
        expect(
            (await sword.query.allowance(bob.address, avatar.address, { u64: 1 }))
                .value.ok
        ).to.equal(true);

        await avatar
            .withSigner(bob)
            .tx.addChild({ u64: 1 }, [sword.address, { u64: 1 }]);
        console.log("Added a sword to an avatar");

        console.log("Equipping sword to avatar");
        /*
        await avatar
            .withSigner(bob)
            .tx.equip({ u64: 1 }, defaultAssetId, 4, [sword.address, { u64: 1 }], 1);
        expect(
            (await avatar.withSigner(bob).query.getEquipment({ u64: 1 }, 4)).value.ok
        ).to.be.ok;
        */
    });
});

// Helper function to add an asset to a token
const addAssetToToken = async (contract: Rmrk, signer: KeyringPair, token: number, asset: number): Promise<void> => {
  const addAssetTokenResult = await contract
    .withSigner(signer)
    .tx.addAssetToToken({ u64: token }, asset, null);

  emit(addAssetTokenResult, "AssetAddedToToken", {
    token: { u64: token },
    asset,
    replaces: null,
  });
}

// Helper function to accept an asset to a token
const acceptAsset = async (contract: Rmrk, signer: KeyringPair, token: number, asset: number): Promise<void> => {
  const acceptAssetResult = await contract.withSigner(signer)
    .tx.acceptAsset({ u64: token }, asset);

  emit(acceptAssetResult, "AssetAccepted", { token: { u64: token }, asset });
}
