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
    let kanariaFactory: Rmrk_factory;
    let gemFactory: Rmrk_factory;
    let api: ApiPromise;
    let deployer: KeyringPair;
    let kanaria: Rmrk;
    let gem: Rmrk;
    let catalog: Contract;
    let bob: KeyringPair;
    let dave: KeyringPair;

    beforeEach(async function(): Promise<void> {
        api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true});
        deployer = keyring.addFromUri("//Alice");
        bob = keyring.addFromUri("//Bob");
        dave = keyring.addFromUri("//Dave");

        kanariaFactory = new Rmrk_factory(api, deployer);
        kanaria = new Rmrk(
            (
                await kanariaFactory.new(
                    ["Kanaria"],
                    ["KAN"],
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

        gemFactory = new Rmrk_factory(api, deployer);
        gem = new Rmrk(
        (
            await gemFactory.new(
            ["Gem"],
            ["GM"],
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
        // define all parts of the NFT.
        const PART_LIST: Part[] = [
            // Background option 1
            {
                partType: PartType.fixed,
                z: 0,
                equippable: [],
                partUri: ["ipfs://backgrounds/1.svg"],
                isEquippableByAll: false,
            },
            // Background option 2
            {
                partType: PartType.fixed,
                z: 0,
                equippable: [],
                partUri: ["ipfs://backgrounds/2.svg"],
                isEquippableByAll: false,
            },
        ];

        await catalog
            .withSigner(deployer)
            .tx.addPartList(PART_LIST);
        expect((await catalog.query.getPartsCount())?.value.unwrap()).to.be.equal(2);
        console.log("Catalog is set");

        // minting tokens
        console.log("Minting tokens");

        // bob mints 5 kanaria
        console.log("Minting Kanaria tokens");
        let kanariaMintResult = await kanaria
        .withSigner(bob)
        .tx.mintMany(5, {
            value: PRICE_PER_MINT.muln(5),
        });
        emit(kanariaMintResult, "Transfer", {
        from: null,
        to: bob.address,
        id: { u64: 1 },
        });
        console.log(`Minted 5 kanarias`);

        // deployer adds two assets for kanaria
        console.log("Adding Kanaria assets");
        const assetDefaultId = 1;
        const assetComposedId = 2;
        const addAssetResult = await kanaria
        .withSigner(deployer)
        .tx.addAssetEntry(catalog.address, assetDefaultId, "0", ["ipfs://kanariaAsset1.png"], [], {
        });
        emit(addAssetResult, "AssetSet", { asset: 1 });
        expect(
        await kanaria
            .withSigner(deployer)
            .tx.addAssetEntry(
            catalog.address,
            assetComposedId,
            "0",
            ["ipfs://kanariaAsset2.json"],
            [0, 2, 4, 6, 8, 9, 10],
            )
        ).to.be.ok;

        expect(
        (await kanaria.withSigner(deployer).query.totalAssets())?.value.unwrap().toString()
        ).to.be.equal("2");
        console.log("Added 2 asset entries to Kanaria");

        // add both assets to token 1
        console.log("Add assets to token 1");
        await addAssetToToken(kanaria, deployer, 1, assetDefaultId)
        await addAssetToToken(kanaria, deployer, 1, assetComposedId)
        expect(
        (await kanaria.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
        ).to.be.equal("0,2");

        // bob accepts both assets
        await acceptAsset(kanaria, bob, 1, assetDefaultId);
        await acceptAsset(kanaria, bob, 1, assetComposedId);
        expect(
        (await kanaria.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
        ).to.be.equal("2,0");
        console.log("Assets accepted");


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
