import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import { encodeAddress } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import BN from "bn.js";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable_lazy";
import Rmrk from "../types/contracts/rmrk_example_equippable_lazy";
import { emit } from "./helper";

use(chaiAsPromised);

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const TOKEN_URI_1 = "ipfs://tokenUriPrefix/1.json";
const TOKEN_URI_5 = "ipfs://tokenUriPrefix/5.json";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;
const ADMIN_ROLE = 0;

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("RMRK Multi Asset tests", () => {
  let kanariaFactory: Rmrk_factory;
  let gemFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let bob: KeyringPair;
  let dave: KeyringPair;
  let kanaria: Rmrk;
  let gem: Rmrk;

  beforeEach(async function (): Promise<void> {
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
  });

  it("Init two rmrk contracts works", async () => {
    expect(
      (await kanaria.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(0);
    expect(
      (await kanaria.query.hasRole(ADMIN_ROLE, deployer.address)).value.ok
    ).to.equal(true);
    expect((await kanaria.query.maxSupply()).value.unwrap()).to.equal(MAX_SUPPLY);
    expect((await kanaria.query.price()).value.unwrap().toString()).to.equal(
      PRICE_PER_MINT.toString()
    );
    const kanariaCollectionId = (await kanaria.query.collectionId()).value;

    expect((await gem.query.totalSupply()).value.unwrap().toNumber()).to.equal(
      0
    );
    expect(
      (await gem.query.hasRole(ADMIN_ROLE, deployer.address)).value.ok
    ).to.equal(true);
    expect((await gem.query.maxSupply()).value.unwrap()).to.equal(MAX_SUPPLY);
    expect((await gem.query.price()).value.unwrap().toString()).to.equal(
      PRICE_PER_MINT.toString()
    );
    const gemCollectionId = (await gem.query.collectionId()).value.ok;
    expect(kanariaCollectionId).to.not.be.equal(gemCollectionId);
  });

  it("Add assets to token and approve them", async () => {
    // This test follows MergesEquippable user story, but without Equipalble tests.
    // https://github.com/rmrk-team/evm-sample-contracts/tree/master/contracts/MergedEquippable
    // The scenarrio is different only when it comes to procedure of nesting child tokens,
    // but the end result is the same.

    // First Bob mints tokens from kanaria and gem contracts.
    // After Deployer (contract owner) adds new assets to gem and kanaria contracts, the same deployer will
    // add those assets to the tokens.
    // Bob accepts new assets on all of his tokens (both kanaria and gem tokens)
    // Bob addds gem tokens (children) to kanaria tokens (parent)
    // Equipping is covered in merged_equippable.spec.ts

    const assetDefaultId = 1;
    const assetComposedId = 2;
    // bob mints 5 kanaria

    const { gasRequired } = await kanaria.withSigner(bob).query.mintMany(5);
    let kanariaMintResult = await kanaria.withSigner(bob).tx.mintMany(5, {
      value: PRICE_PER_MINT.muln(5),
    });
    emit(kanariaMintResult, "Transfer", {
      from: null,
      to: bob.address,
      id: { u64: 1 },
    });

    // bob mints 15 gem
    const gasRequiredGem = (await gem.withSigner(bob).query.mint()).gasRequired;
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

    // deployer adds two asset entries for kanaria
    const assetEntryGas = (
      await kanaria
        .withSigner(deployer)
        .query.addAssetEntry(assetDefaultId, "1", ["ipfs://default.png"], [0])
    ).gasRequired;
    const addAssetResult = await kanaria
      .withSigner(deployer)
      .tx.addAssetEntry(assetDefaultId, "1", ["ipfs://default.png"], [0], {
        gasLimit: assetEntryGas,
      });
    emit(addAssetResult, "AssetSet", { asset: 1 });
    const addAssetResult2 = await kanaria
      .withSigner(deployer)
      .tx.addAssetEntry(
        assetComposedId,
        "1",
        ["ipfs://meta1.json"],
        [1, 3, 5, 7, 9, 10, 11],
        { gasLimit: assetEntryGas }
      );
    emit(addAssetResult2, "AssetSet", { asset: 2 });
    expect(
      (await kanaria.withSigner(deployer).query.totalAssets())?.value.unwrap().toString()
    ).to.be.equal("2");

    // Deployer adds both assets to token 1
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

    // We'll add 4 assets for each gem, a full version and 3 versions matching each slot.
    // We will have only 2 types of gems -> 4x2: 8 assets.
    // This is not composed by others, so fixed and slot parts are never used.
    const equippableRefIdLeftGem = 1;
    const equippableRefIdMidGem = 2;
    const equippableRefIdRightGem = 3;
    const gemAssetAddGas = (
      await gem
        .withSigner(deployer)
        .query.addAssetEntry(0, 0, ["ipfs://gems/typeA/full.svg"], [0])
    ).gasRequired;
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(1, 0, ["ipfs://gems/typeA/full.svg"], [0], {
        gasLimit: gemAssetAddGas,
      });
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        2,
        equippableRefIdLeftGem,
        ["ipfs://gems/typeA/left.svg"],
        [0],
        { gasLimit: gemAssetAddGas }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        3,
        equippableRefIdMidGem,
        ["ipfs://gems/typeA/mid.svg"],
        [0],
        { gasLimit: gemAssetAddGas }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        4,
        equippableRefIdRightGem,
        ["ipfs://gems/typeA/right.svg"],
        [0],
        { gasLimit: gemAssetAddGas }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(5, 0, ["ipfs://gems/typeB/full.svg"], [0], {
        gasLimit: gemAssetAddGas,
      });
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        6,
        equippableRefIdLeftGem,
        ["ipfs://gems/typeB/left.svg"],
        [0],
        { gasLimit: gemAssetAddGas }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        7,
        equippableRefIdMidGem,
        ["ipfs://gems/typeB/mid.svg"],
        [0],
        { gasLimit: gemAssetAddGas }
      );
    await gem
      .withSigner(deployer)
      .tx.addAssetEntry(
        8,
        equippableRefIdRightGem,
        ["ipfs://gems/typeB/right.svg"],
        [0],
        { gasLimit: gemAssetAddGas }
      );
    expect(
      (await gem.withSigner(deployer).query.totalAssets())?.value.unwrap().toString()
    ).to.be.equal("8");

    // We add assets of type A to gem 1 and 2, and type Bto gem 3. Both are nested into the first kanaria
    // This means gems 1 and 2 will have the same asset, which is totally valid.
    await addAssetToToken(gem, deployer, 1, 1)
    await addAssetToToken(gem, deployer, 1, 2)
    await addAssetToToken(gem, deployer, 1, 3)
    await addAssetToToken(gem, deployer, 1, 4)
    await addAssetToToken(gem, deployer, 2, 1)
    await addAssetToToken(gem, deployer, 2, 2)
    await addAssetToToken(gem, deployer, 2, 3)
    await addAssetToToken(gem, deployer, 2, 4)
    await addAssetToToken(gem, deployer, 3, 5)
    await addAssetToToken(gem, deployer, 3, 6)
    await addAssetToToken(gem, deployer, 3, 7)
    await addAssetToToken(gem, deployer, 3, 8)
    expect(
      (await gem.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("0,4");
    expect(
      (await gem.query.totalTokenAssets({ u64: 2 }))?.value.unwrap().ok.toString()
    ).to.be.equal("0,4");
    expect(
      (await gem.query.totalTokenAssets({ u64: 3 }))?.value.unwrap().ok.toString()
    ).to.be.equal("0,4");

    // We accept each asset for all gems
    await acceptAsset(gem, bob, 1, 1);
    await acceptAsset(gem, bob, 1, 2)
    await acceptAsset(gem, bob, 1, 3)
    await acceptAsset(gem, bob, 1, 4)
    await acceptAsset(gem, bob, 2, 1)
    await acceptAsset(gem, bob, 2, 2)
    await acceptAsset(gem, bob, 2, 3)
    await acceptAsset(gem, bob, 2, 4)
    await acceptAsset(gem, bob, 3, 5)
    await acceptAsset(gem, bob, 3, 6)
    await acceptAsset(gem, bob, 3, 7)
    await acceptAsset(gem, bob, 3, 8)

    expect(
      (await gem.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("4,0");
    expect(
      (await gem.query.totalTokenAssets({ u64: 2 }))?.value.unwrap().ok.toString()
    ).to.be.equal("4,0");
    expect(
      (await gem.query.totalTokenAssets({ u64: 3 }))?.value.unwrap().ok.toString()
    ).to.be.equal("4,0");

    // bob approves kanaria Contract on gem (for nesting gem on kanaria)
    for (let i = 1; i < 16; i++) {
      let approveGas = (
        await gem.withSigner(bob).query.approve(kanaria.address, { u64: 1 }, true)
      ).gasRequired;
      await gem.withSigner(bob).tx.approve(kanaria.address, { u64: i }, true, {
        gasLimit: approveGas,
      });
      expect(
        (await gem.query.allowance(bob.address, kanaria.address, { u64: 1 }))
          .value.ok
      ).to.equal(true);
    }
    // bob adds 3 gem nfts to bob's 5 kanaria nfts (kanaria is now parent of gem tokens)
    for (let k = 1; k < 6; k++) {
      for (let g = 1; g < 4; g++) {
        const addgemGas = (
          await kanaria
            .withSigner(bob)
            .query.addChild({ u64: k }, [gem.address, { u64: g }])
        ).gasRequired;
        const res = await kanaria
          .withSigner(bob)
          .tx.addChild({ u64: k }, [gem.address, { u64: g }], {
            gasLimit: addgemGas,
          });
        const balance = (
          await kanaria.query.childrenBalance({ u64: k })
        )?.value.unwrap().ok.toString();
      }
      expect(
        (await kanaria.query.childrenBalance({ u64: k }))?.value.unwrap().ok.toString()
      ).to.be.equal("3,0");
    }
  });
});

// Helper function to add an asset to a token
const addAssetToToken = async (contract: Rmrk, signer: KeyringPair, token: number, asset: number): Promise<void> => {
  let { gasRequired: addAssetGas } = await contract
    .withSigner(signer)
    .query.addAssetToToken({ u64: token }, asset, null);

  const addAssetTokenResult = await contract
    .withSigner(signer)
    .tx.addAssetToToken({ u64: token }, asset, null, { gasLimit: addAssetGas });

  emit(addAssetTokenResult, "AssetAddedToToken", {
    token: { u64: token },
    asset,
    replaces: null,
  });
}


// Helper function to accept an asset to a token
const acceptAsset = async (contract: Rmrk, signer: KeyringPair, token: number, asset: number): Promise<void> => {
  const { gasRequired: assetAcceptGas } = await contract.withSigner(signer)
    .query.acceptAsset({ u64: token }, asset);
  const acceptAssetResult = await contract.withSigner(signer)
    .tx.acceptAsset({ u64: token }, asset, { gasLimit: assetAcceptGas });
  emit(acceptAssetResult, "AssetAccepted", { token: { u64: token }, asset });
}