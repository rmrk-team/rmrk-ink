import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import { encodeAddress } from "@polkadot/keyring";
import BN from "bn.js";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable_lazy";
import Rmrk from "../types/contracts/rmrk_example_equippable_lazy";
import {
  PartType,
  Part,
} from "../types/types-arguments/rmrk_example_equippable_lazy";

import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
// import { AccountId } from '../types/types-arguments/rmrk_example_equippable_lazy';
import { ReturnNumber } from "@supercolony/typechain-types";

use(chaiAsPromised);

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const BASE_METADATA = "ipfs://baseMetadata";
const TOKEN_URI_1 = "ipfs://tokenUriPrefix/1.json";
const TOKEN_URI_5 = "ipfs://tokenUriPrefix/5.json";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("RMRK Merged Equippable", () => {
  let kanariaFactory: Rmrk_factory;
  let gemFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let bob: KeyringPair;
  let dave: KeyringPair;
  let kanaria: Rmrk;
  let gem: Rmrk;

  const ZERO_ADDRESS = encodeAddress(
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  );
  let gasRequired: bigint;

  async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
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
  }

  it("Merged Equippable user journey", async () => {
    await setup();
    // This test follows MergedEquippable user story.
    // https://github.com/rmrk-team/evm-sample-contracts/tree/master/contracts/MergedEquippable
    // The scenario is different only when it comes to procedure of nesting child tokens,
    // but the end result is the same.

    const mintingKanariaCnt = 5;

    // set Base metadata
    console.log("Setting up Base");
    const setupBaseGas = (await gem.query.setupBase([BASE_METADATA]))
      .gasRequired;
    await gem
      .withSigner(deployer)
      .tx.setupBase([BASE_METADATA], { gasLimit: setupBaseGas * 2n });
    // define 2 test Parts
    const PART_LIST: Part[] = [
      // Background option 1
      {
        partType: PartType.fixed,
        z: 0,
        equippable: [],
        metadataUri: ["ipfs://backgrounds/1.svg"],
        isEquippableByAll: false,
      },
      // Background option 2
      {
        partType: PartType.fixed,
        z: 0,
        equippable: [],
        metadataUri: ["ipfs://backgrounds/2.svg"],
        isEquippableByAll: false,
      },
      // Head option 1
      {
        partType: PartType.fixed,
        z: 3,
        equippable: [],
        metadataUri: ["ipfs://heads/1.svg"],
        isEquippableByAll: false,
      },
      // Head option 2
      {
        partType: PartType.fixed,
        z: 3,
        equippable: [],
        metadataUri: ["ipfs://heads/2.svg"],
        isEquippableByAll: false,
      },
      // Body option 1
      {
        partType: PartType.fixed,
        z: 2,
        equippable: [],
        metadataUri: ["ipfs://body/1.svg"],
        isEquippableByAll: false,
      },
      // Body option 2
      {
        partType: PartType.fixed,
        z: 2,
        equippable: [],
        metadataUri: ["ipfs://body/2.svg"],
        isEquippableByAll: false,
      },
      // Wings option 1
      {
        partType: PartType.fixed,
        z: 1,
        equippable: [],
        metadataUri: ["ipfs://wings/1.svg"],
        isEquippableByAll: false,
      },
      // Wings option 2
      {
        partType: PartType.fixed,
        z: 1,
        equippable: [],
        metadataUri: ["ipfs://wings/2.svg"],
        isEquippableByAll: false,
      },
      // Gem slot 1
      {
        partType: PartType.slot,
        z: 4,
        equippable: [gem.address],
        metadataUri: [""],
        isEquippableByAll: false,
      },
      // Gem slot 2
      {
        partType: PartType.slot,
        z: 4,
        equippable: [gem.address],
        metadataUri: [""],
        isEquippableByAll: false,
      },
      // Gem slot 3
      {
        partType: PartType.slot,
        z: 4,
        equippable: [gem.address],
        metadataUri: [""],
        isEquippableByAll: false,
      },
    ];

    // add parts to base
    const addPartListGas = (
      await gem.withSigner(deployer).query.addPartList(PART_LIST)
    ).gasRequired;
    await gem
      .withSigner(deployer)
      .tx.addPartList(PART_LIST, { gasLimit: addPartListGas * 2n });
    expect((await gem.query.getPartsCount())?.value).to.be.equal(11);
    console.log("Base is set");

    // minting tokens
    console.log("Minting tokens");

    // bob mints 5 kanaria
    console.log("Minting Kanaria tokens");
    const { gasRequired } = await kanaria
      .withSigner(bob)
      .query.mintMany(mintingKanariaCnt);
    let kanariaMintResult = await kanaria
      .withSigner(bob)
      .tx.mintMany(mintingKanariaCnt, {
        value: PRICE_PER_MINT.muln(mintingKanariaCnt),
        gasLimit: gasRequired * 2n,
      });
    emit(kanariaMintResult, "Transfer", {
      from: null,
      to: bob.address,
      id: { u64: 1 },
    });
    console.log(`Minted ${mintingKanariaCnt} kanarias`);

    // bob mints 15 gem
    console.log("Minting Gem tokens");
    const gasRequiredGem = (await gem.withSigner(bob).query.mint()).gasRequired;
    for (let i = 1; i < 16; i++) {
      const gemMintResult = await gem.withSigner(bob).tx.mint({
        value: PRICE_PER_MINT,
        gasLimit: gasRequiredGem * 2n,
      });
      emit(gemMintResult, "Transfer", {
        from: null,
        to: bob.address,
        id: { u64: i },
      });
    }
    expect((await gem.query.balanceOf(bob.address)).value).to.equal(15);

    // deployer adds two assets for kanaria
    console.log("Adding Kanaria assets");
    const assetDefaultId = 1;
    const assetComposedId = 2;
    const assetEntryGas = (
      await kanaria
        .withSigner(deployer)
        .query.addAssetEntry(
          assetDefaultId,
          "0",
          ["ipfs://kanariaAsset1.png"],
          []
        )
    ).gasRequired;
    const addAssetResult = await kanaria
      .withSigner(deployer)
      .tx.addAssetEntry(assetDefaultId, "0", ["ipfs://kanariaAsset1.png"], [], {
        gasLimit: assetEntryGas * 2n,
      });
    emit(addAssetResult, "AssetSet", { asset: 1 });
    expect(
      await kanaria
        .withSigner(deployer)
        .tx.addAssetEntry(
          assetComposedId,
          "0",
          ["ipfs://kanariaAsset2.json"],
          [0, 2, 4, 6, 8, 9, 10],
          { gasLimit: assetEntryGas * 2n }
        )
    ).to.be.ok;
    // emit(addAssetResult, 'AssetSet', { asset: 2 });
    expect(
      (await kanaria.withSigner(deployer).query.totalAssets())?.value.toString()
    ).to.be.equal("2");
    console.log("Added 2 asset entries to Kanaria");

    // add both assets to token 1
    console.log("Add assets to token 1");
    const assetAddGas = (
      await kanaria
        .withSigner(deployer)
        .query.addAssetToToken({ u64: 1 }, assetDefaultId, null)
    ).gasRequired;
    const addAssetTokenResult = await kanaria
      .withSigner(deployer)
      .tx.addAssetToToken({ u64: 1 }, assetDefaultId, null, {
        gasLimit: assetAddGas * 2n,
      });
    emit(addAssetTokenResult, "AssetAddedToToken", {
      token: { u64: 1 },
      asset: 1,
      replaces: null,
    });
    await kanaria
      .withSigner(deployer)
      .tx.addAssetToToken({ u64: 1 }, assetComposedId, null, {
        gasLimit: assetAddGas * 2n,
      });
    expect(
      (await kanaria.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()
    ).to.be.equal("0,2");

    // bob accepts both assets
    const assetAcceptGas = (
      await kanaria
        .withSigner(bob)
        .query.acceptAsset({ u64: 1 }, assetDefaultId)
    ).gasRequired;
    await kanaria
      .withSigner(bob)
      .tx.acceptAsset({ u64: 1 }, assetDefaultId, { gasLimit: assetAcceptGas });
    await kanaria.withSigner(bob).tx.acceptAsset({ u64: 1 }, assetComposedId, {
      gasLimit: assetAcceptGas * 2n,
    });
    expect(
      (await kanaria.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()
    ).to.be.equal("2,0");
    console.log("Assets accepted");

    // We'll add 4 assets for each gem, a full version and 3 versions matching each slot.
    // We will have only 2 types of gems -> 4x2: 8 assets.
    console.log("Adding Gem assets");
    console.log("Adding asset entries");
    const equippableRefIdLeftGem = 1;
    const equippableRefIdMidGem = 2;
    const equippableRefIdRightGem = 3;
    const gemAssetAddGas = (
      await gem
        .withSigner(deployer)
        .query.addAssetEntry(0, 0, ["ipfs://gems/typeA/full.svg"], [])
    ).gasRequired;
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(1, 0, ["ipfs://gems/typeA/full.svg"], [], {
        gasLimit: gemAssetAddGas,
      });
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        2,
        equippableRefIdLeftGem,
        ["ipfs://gems/typeA/left.svg"],
        [],
        { gasLimit: gemAssetAddGas * 2n }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        3,
        equippableRefIdMidGem,
        ["ipfs://gems/typeA/mid.svg"],
        [],
        { gasLimit: gemAssetAddGas * 2n }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        4,
        equippableRefIdRightGem,
        ["ipfs://gems/typeA/right.svg"],
        [],
        { gasLimit: gemAssetAddGas * 2n }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(5, 0, ["ipfs://gems/typeB/full.svg"], [], {
        gasLimit: gemAssetAddGas * 2n,
      });
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        6,
        equippableRefIdLeftGem,
        ["ipfs://gems/typeB/left.svg"],
        [],
        { gasLimit: gemAssetAddGas * 2n }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        7,
        equippableRefIdMidGem,
        ["ipfs://gems/typeB/mid.svg"],
        [],
        { gasLimit: gemAssetAddGas * 2n }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        8,
        equippableRefIdRightGem,
        ["ipfs://gems/typeB/right.svg"],
        [],
        { gasLimit: gemAssetAddGas * 2n }
      );
    expect(
      (await gem.withSigner(deployer).query.totalAssets())?.value.toString()
    ).to.be.equal("8");
    console.log(
      "Added 8 gem assets. 2 Types of gems with full, left, mid and right versions."
    );

    // 9, 10 and 11 are the slot part ids for the gems, defined on the base.
    // e.g. Any asset on gem, which sets its equippableRefId to equippableRefIdLeftGem
    //      will be considered a valid equip into any kanaria on slot 9 (left gem).
    console.log("Setting valid parent reference IDs");
    const validParentGas = (
      await gem
        .withSigner(bob)
        .query.setValidParentForEquippableGroup(
          equippableRefIdLeftGem,
          kanaria.address,
          8
        )
    ).gasRequired;
    await gem
      .withSigner(bob)
      .tx.setValidParentForEquippableGroup(
        equippableRefIdLeftGem,
        kanaria.address,
        8,
        { gasLimit: validParentGas }
      );
    await gem
      .withSigner(bob)
      .tx.setValidParentForEquippableGroup(
        equippableRefIdMidGem,
        kanaria.address,
        9,
        { gasLimit: validParentGas }
      );
    await gem
      .withSigner(bob)
      .tx.setValidParentForEquippableGroup(
        equippableRefIdRightGem,
        kanaria.address,
        10,
        { gasLimit: validParentGas }
      );

    // We add assets of type A to gem 1 and 2, and type Bto gem 3. Both are nested into the first kanaria
    // This means gems 1 and 2 will have the same asset, which is totally valid.
    // Assets are accepted by default since the caller (bob) is token owner, and acceptAsset() does not need to be called
    console.log("Add assets to tokens");
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 1 }, 2, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 1 }, 3, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 1 }, 4, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 2 }, 1, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 2 }, 2, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 2 }, 3, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 2 }, 4, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 3 }, 5, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 3 }, 6, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 3 }, 7, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 1 }, 1, null, { gasLimit: assetAddGas * 2n });
    await gem
      .withSigner(bob)
      .tx.addAssetToToken({ u64: 3 }, 8, null, { gasLimit: assetAddGas * 2n });
    expect(
      (await gem.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()
    ).to.be.equal("4,0");
    expect(
      (await gem.query.totalTokenAssets({ u64: 2 }))?.value.ok.toString()
    ).to.be.equal("4,0");
    expect(
      (await gem.query.totalTokenAssets({ u64: 3 }))?.value.ok.toString()
    ).to.be.equal("4,0");
    console.log("Added 4 assets to each of 3 gems.");
    // Assets are accepted by default since the caller (bob) is token owner, and acceptAsset() does not need to be called
    console.log("Accepted 4 assets to each of 3 gems.");

    // bob approves kanaria Contract on gem (for nesting gem on kanaria)
    const approveGas = (
      await gem.withSigner(bob).query.approve(kanaria.address, { u64: 1 }, true)
    ).gasRequired;
    for (let i = 1; i < 16; i++) {
      await gem.withSigner(bob).tx.approve(kanaria.address, { u64: i }, true, {
        gasLimit: approveGas,
      });
      expect(
        (await gem.query.allowance(bob.address, kanaria.address, { u64: 1 }))
          .value
      ).to.equal(true);
    }
    // bob adds 3 gem nfts to bob's 5 kanaria nfts (kanaria is now parent of gem tokens)
    const addgemGas = (
      await kanaria
        .withSigner(bob)
        .query.addChild({ u64: 1 }, [gem.address, { u64: 1 }])
    ).gasRequired;
    for (let k = 1; k < 6; k++) {
      for (let g = 1; g < 4; g++) {
        const res = await kanaria
          .withSigner(bob)
          .tx.addChild({ u64: k }, [gem.address, { u64: g }], {
            gasLimit: addgemGas * 2n,
          });
        const balance = (
          await kanaria.query.childrenBalance({ u64: k })
        )?.value.ok.toString();
      }
      expect(
        (await kanaria.query.childrenBalance({ u64: k }))?.value.ok.toString()
      ).to.be.equal("3,0");
    }
    console.log(`Added 3 gems into each kanaria`);

    // Equipping
    console.log("Equipping gems to kanaria");
    const equipGas = (
      await kanaria
        .withSigner(bob)
        .query.equip(
          { u64: 1 },
          assetComposedId,
          8,
          [gem.address, { u64: 2 }],
          2
        )
    ).gasRequired;
    await kanaria
      .withSigner(bob)
      .tx.equip({ u64: 1 }, assetComposedId, 8, [gem.address, { u64: 1 }], 2, {
        gasLimit: equipGas,
      });
    await kanaria
      .withSigner(bob)
      .tx.equip({ u64: 1 }, assetComposedId, 9, [gem.address, { u64: 2 }], 3, {
        gasLimit: equipGas * 2n,
      });
    await kanaria
      .withSigner(bob)
      .tx.equip({ u64: 1 }, assetComposedId, 10, [gem.address, { u64: 3 }], 8, {
        gasLimit: equipGas * 2n,
      });
    expect(
      (await kanaria.withSigner(bob).query.getEquipment({ u64: 1 }, 8)).value
    ).to.be.ok;
    expect(
      (await kanaria.withSigner(bob).query.getEquipment({ u64: 1 }, 9)).value
    ).to.be.ok;
    expect(
      (await kanaria.withSigner(bob).query.getEquipment({ u64: 1 }, 10)).value
    ).to.be.ok;
    console.log("Equipped 3 gems into first kanaria");
  });
});

// Helper function to parse Events
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function emit(result: { events?: any }, name: string, args: any): void {
  const event = result.events.find(
    (event: { name: string }) => event.name === name
  );
  for (const key of Object.keys(event.args)) {
    if (event.args[key] instanceof ReturnNumber) {
      event.args[key] = event.args[key].toNumber();
    }
  }
  expect(event).eql({ name, args });
}

// Helper function to convert error code to string
function hex2a(psp34CustomError: any): string {
  var hex = psp34CustomError.toString(); //force conversion
  var str = "";
  for (var i = 0; i < hex.length; i += 2)
    str += String.fromCharCode(parseInt(hex.substr(i, 2), 16));
  return str.substring(1);
}
