import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import BN from "bn.js";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable_lazy";
import Rmrk from "../types/contracts/rmrk_example_equippable_lazy";
import Catalog_Factory from "../types/constructors/catalog_example";
import Contract from "../types/contracts/catalog_example";
import {
  PartType,
  Part,
} from "../types/types-arguments/catalog_example";

import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { RmrkError } from "../types/types-returns/rmrk_example_equippable_lazy";
import { emit } from "./helper";

use(chaiAsPromised);

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const CATALOG_METADATA = "ipfs://catalogMetadata/data.json";
const BASE_METADATA = "ipfs://baseMetadata";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("RMRK Merged Equippable", () => {
  let kanariaFactory: Rmrk_factory;
  let gemFactory: Rmrk_factory;
  let catalogFactory: Catalog_Factory;
  let avatarFactory: Rmrk_factory;
  let swordFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let bob: KeyringPair;
  let dave: KeyringPair;
  let kanaria: Rmrk;
  let gem: Rmrk;
  let avatar: Rmrk;
  let sword: Rmrk;
  let catalog: Contract;

  beforeEach(async function(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true });
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

    catalogFactory = new Catalog_Factory(api, deployer);
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

    let PART_IDS = [0, 1, 2, 3, 4]

    const swordSlot = 4;

    // add all parts to catalog
    await catalog
      .withSigner(deployer)
      .tx["catalog::addPartList"](PART_IDS, PART_LIST);
    expect((await catalog.query.getPartsCount())?.value.unwrap()).to.be.equal(5);
    console.log("Catalog is set");

    console.log("Minting tokens");

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
        equippableWoodenSword,
        avatar.address,
        swordSlot
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
    await avatar
      .withSigner(bob)
      .tx.addChild({ u64: 2 }, [sword.address, { u64: 1 }]);
    console.log("Added swords to both avatars");

    console.log("Equipping sword to avatar");
    // This works because wooden sword is allowed to be equipped to the avatar token.
    await avatar
      .withSigner(bob)
      .tx.equip({ u64: 1 }, defaultAssetId, swordSlot, [sword.address, { u64: 1 }], equippableWoodenSword);
    expect(
      (await avatar.withSigner(bob).query.getEquipment({ u64: 1 }, swordSlot)).value.ok
    ).to.be.ok;

    // Fails because copper sword cannot be equipped to the avatar.
    const equipCopperError = await avatar
      .withSigner(bob)
      .query.equip({ u64: 2 }, defaultAssetId, swordSlot, [sword.address, { u64: 1 }], equippableCopperSword)
    expect(equipCopperError.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.unknownPart
    );

    // Fails because Dave is not the token owner.
    const notOwnerError = await avatar
      .withSigner(dave)
      .query.equip({ u64: 2 }, defaultAssetId, swordSlot, [sword.address, { u64: 1 }], equippableWoodenSword)
    expect(notOwnerError.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.notTokenOwner
    );

    // Fails because of non-existent sword asset. 
    const nonExistentAsset = await avatar
      .withSigner(bob)
      .query.equip({ u64: 2 }, defaultAssetId, swordSlot, [sword.address, { u64: 1 }], 7)
    expect(nonExistentAsset.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.unknownEquippableAsset
    );

    // Fails because wrong slot id.
    const wrongSlotId = await avatar
      .withSigner(bob)
      .query.equip({ u64: 2 }, defaultAssetId, 1, [sword.address, { u64: 1 }], 7)
    expect(wrongSlotId.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.targetAssetCannotReceiveSlot
    );

    // Cannot be equipped when slot isn't free.
    const slotAlreadyUsed = await avatar
      .withSigner(bob)
      .query.equip({ u64: 1 }, defaultAssetId, swordSlot, [sword.address, { u64: 1 }], 7)
    expect(slotAlreadyUsed.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.slotAlreadyUsed
    );

    // Now we ensure that unequip also works.

    // Dave cannot unequip
    const daveCannotUnequip = await avatar
      .withSigner(dave)
      .query.unequip({ u64: 1 }, swordSlot)
    expect(daveCannotUnequip.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.notTokenOwner
    );

    // Cannot unequip if it is not equipped.
    const notEquipped = await avatar
      .withSigner(bob)
      .query.unequip({ u64: 2 }, swordSlot)
    expect(notEquipped.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.notEquipped
    );

    await avatar
      .withSigner(bob)
      .tx.unequip({ u64: 1 }, swordSlot);
    expect(
      (await avatar.withSigner(bob).query.getEquipment({ u64: 1 }, swordSlot)).value.ok
    ).to.be.null;
  });

  it("Merged Equippable user journey", async () => {
    // This test follows MergedEquippable user story.
    // https://github.com/rmrk-team/evm-sample-contracts/tree/master/contracts/MergedEquippable
    // The scenario is different only when it comes to procedure of nesting child tokens,
    // but the end result is the same.

    const mintingKanariaCnt = 5;

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
      // Head option 1
      {
        partType: PartType.fixed,
        z: 3,
        equippable: [],
        partUri: ["ipfs://heads/1.svg"],
        isEquippableByAll: false,
      },
      // Head option 2
      {
        partType: PartType.fixed,
        z: 3,
        equippable: [],
        partUri: ["ipfs://heads/2.svg"],
        isEquippableByAll: false,
      },
      // Body option 1
      {
        partType: PartType.fixed,
        z: 2,
        equippable: [],
        partUri: ["ipfs://body/1.svg"],
        isEquippableByAll: false,
      },
      // Body option 2
      {
        partType: PartType.fixed,
        z: 2,
        equippable: [],
        partUri: ["ipfs://body/2.svg"],
        isEquippableByAll: false,
      },
      // Wings option 1
      {
        partType: PartType.fixed,
        z: 1,
        equippable: [],
        partUri: ["ipfs://wings/1.svg"],
        isEquippableByAll: false,
      },
      // Wings option 2
      {
        partType: PartType.fixed,
        z: 1,
        equippable: [],
        partUri: ["ipfs://wings/2.svg"],
        isEquippableByAll: false,
      },
      // Gem slot 1
      {
        partType: PartType.slot,
        z: 4,
        equippable: [gem.address],
        partUri: [""],
        isEquippableByAll: false,
      },
      // Gem slot 2
      {
        partType: PartType.slot,
        z: 4,
        equippable: [gem.address],
        partUri: [""],
        isEquippableByAll: false,
      },
      // Gem slot 3
      {
        partType: PartType.slot,
        z: 4,
        equippable: [gem.address],
        partUri: [""],
        isEquippableByAll: false,
      },
    ];

    const PART_IDS = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    // add parts to catalog
    await catalog
      .withSigner(deployer)
      .tx["catalog::addPartList"](PART_IDS, PART_LIST);
    expect((await catalog.query.getPartsCount())?.value.unwrap()).to.be.equal(11);
    console.log("Catalog is set");

    // minting tokens
    console.log("Minting tokens");

    // bob mints 5 kanaria
    console.log("Minting Kanaria tokens");
    let kanariaMintResult = await kanaria
      .withSigner(bob)
      .tx.mintMany(mintingKanariaCnt, {
        value: PRICE_PER_MINT.muln(mintingKanariaCnt),
      });
    emit(kanariaMintResult, "Transfer", {
      from: null,
      to: bob.address,
      id: { u64: 1 },
    });
    console.log(`Minted ${mintingKanariaCnt} kanarias`);

    // bob mints 15 gem
    console.log("Minting Gem tokens");
    for (let i = 1; i < 16; i++) {
      const gemMintResult = await gem.withSigner(bob).tx.mint({
        value: PRICE_PER_MINT,
      });
      emit(gemMintResult, "Transfer", {
        from: null,
        to: bob.address,
        id: { u64: i },
      });
    }
    expect((await gem.query.balanceOf(bob.address)).value.unwrap()).to.equal(15);

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
    // emit(addAssetResult, 'AssetSet', { asset: 2 });
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

    // We'll add 4 assets for each gem, a full version and 3 versions matching each slot.
    // We will have only 2 types of gems -> 4x2: 8 assets.
    console.log("Adding Gem assets");
    console.log("Adding asset entries");
    const equippableRefIdLeftGem = 1;
    const equippableRefIdMidGem = 2;
    const equippableRefIdRightGem = 3;
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(catalog.address, 1, 0, ["ipfs://gems/typeA/full.svg"], []);
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        catalog.address,
        2,
        equippableRefIdLeftGem,
        ["ipfs://gems/typeA/left.svg"],
        [],
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        catalog.address,
        3,
        equippableRefIdMidGem,
        ["ipfs://gems/typeA/mid.svg"],
        [],
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        catalog.address,
        4,
        equippableRefIdRightGem,
        ["ipfs://gems/typeA/right.svg"],
        [],
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(catalog.address, 5, 0, ["ipfs://gems/typeB/full.svg"], []
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        catalog.address,
        6,
        equippableRefIdLeftGem,
        ["ipfs://gems/typeB/left.svg"],
        [],
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        catalog.address,
        7,
        equippableRefIdMidGem,
        ["ipfs://gems/typeB/mid.svg"],
        [],
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        catalog.address,
        8,
        equippableRefIdRightGem,
        ["ipfs://gems/typeB/right.svg"],
        [],
      );
    expect(
      (await gem.withSigner(deployer).query.totalAssets())?.value.unwrap().toString()
    ).to.be.equal("8");
    console.log(
      "Added 8 gem assets. 2 Types of gems with full, left, mid and right versions."
    );

    // 9, 10 and 11 are the slot part ids for the gems, defined on the catalog.
    // e.g. Any asset on gem, which sets its equippableRefId to equippableRefIdLeftGem
    //      will be considered a valid equip into any kanaria on slot 9 (left gem).
    console.log("Setting valid parent reference IDs");
    await gem
      .withSigner(bob)
      .tx.setValidParentForEquippableGroup(
        equippableRefIdLeftGem,
        kanaria.address,
        8);
    await gem
      .withSigner(bob)
      .tx.setValidParentForEquippableGroup(
        equippableRefIdMidGem,
        kanaria.address,
        9);
    await gem
      .withSigner(bob)
      .tx.setValidParentForEquippableGroup(
        equippableRefIdRightGem,
        kanaria.address,
        10);

    // We add assets of type A to gem 1 and 2, and type Bto gem 3. Both are nested into the first kanaria
    // This means gems 1 and 2 will have the same asset, which is totally valid.
    // Assets are accepted by default since the caller (bob) is token owner, and acceptAsset() does not need to be called
    console.log("Add assets to tokens");
    await addAssetToToken(gem, bob, 1, 1)
    await addAssetToToken(gem, bob, 1, 2)
    await addAssetToToken(gem, bob, 1, 3)
    await addAssetToToken(gem, bob, 1, 4)
    await addAssetToToken(gem, bob, 2, 1)
    await addAssetToToken(gem, bob, 2, 2)
    await addAssetToToken(gem, bob, 2, 3)
    await addAssetToToken(gem, bob, 2, 4)
    await addAssetToToken(gem, bob, 3, 5)
    await addAssetToToken(gem, bob, 3, 6)
    await addAssetToToken(gem, bob, 3, 7)
    await addAssetToToken(gem, bob, 3, 8)
    expect(
      (await gem.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("4,0");
    expect(
      (await gem.query.totalTokenAssets({ u64: 2 }))?.value.unwrap().ok.toString()
    ).to.be.equal("4,0");
    expect(
      (await gem.query.totalTokenAssets({ u64: 3 }))?.value.unwrap().ok.toString()
    ).to.be.equal("4,0");
    console.log("Added 4 assets to each of 3 gems.");
    // Assets are accepted by default since the caller (bob) is token owner, and acceptAsset() does not need to be called
    console.log("Accepted 4 assets to each of 3 gems.");

    // bob approves kanaria Contract on gem (for nesting gem on kanaria)
    for (let i = 1; i < 16; i++) {
      await gem.withSigner(bob).tx.approve(kanaria.address, { u64: i }, true);
      expect(
        (await gem.query.allowance(bob.address, kanaria.address, { u64: 1 }))
          .value.ok
      ).to.equal(true);
    }
    // bob adds 3 gem nfts to bob's 5 kanaria nfts (kanaria is now parent of gem tokens)
    for (let k = 1; k < 6; k++) {
      for (let g = 1; g < 4; g++) {
        const res = await kanaria
          .withSigner(bob)
          .tx.addChild({ u64: k }, [gem.address, { u64: g }]);
        const balance = (
          await kanaria.query.childrenBalance({ u64: k })
        )?.value.ok.toString();
      }
      expect(
        (await kanaria.query.childrenBalance({ u64: k }))?.value.unwrap().ok.toString()
      ).to.be.equal("3,0");
    }
    console.log(`Added 3 gems into each kanaria`);

    // Equipping
    console.log("Equipping gems to kanaria");
    await kanaria
      .withSigner(bob)
      .tx.equip({ u64: 1 }, assetComposedId, 8, [gem.address, { u64: 1 }], 2);
    await kanaria
      .withSigner(bob)
      .tx.equip({ u64: 1 }, assetComposedId, 9, [gem.address, { u64: 2 }], 3);
    await kanaria
      .withSigner(bob)
      .tx.equip({ u64: 1 }, assetComposedId, 10, [gem.address, { u64: 3 }], 8);

    expect(
      (await kanaria.withSigner(bob).query.getEquipment({ u64: 1 }, 8)).value.ok
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
