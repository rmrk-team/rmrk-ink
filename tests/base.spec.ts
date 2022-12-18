import { expect, use } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { encodeAddress } from '@polkadot/keyring';
import BN from 'bn.js';
import Rmrk_factory from '../types/constructors/rmrk_contract';
import Rmrk from '../types/contracts/rmrk_contract';

import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { ReturnNumber } from '@supercolony/typechain-types';
import { PartType, Part } from 'types-arguments/rmrk_contract';

use(chaiAsPromised);

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const BASE_METADATA = "ipfs://baseMetadata";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;

// Create a new instance of contract
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });

describe('RMRK Base tests', () => {
  let kanariaFactory: Rmrk_factory;
  let gemFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let bob: KeyringPair;
  let dave: KeyringPair;
  let kanaria: Rmrk;
  let gem: Rmrk;

  const ZERO_ADDRESS = encodeAddress(
    '0x0000000000000000000000000000000000000000000000000000000000000000',
  );
  let gasRequired: bigint;

  async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
    dave = keyring.addFromUri('//Dave');
    kanariaFactory = new Rmrk_factory(api, deployer);
    kanaria = new Rmrk((await kanariaFactory.new(
      ["Kanaria"],
      ['KAN'],
      [BASE_URI],
      MAX_SUPPLY,
      PRICE_PER_MINT,
      [COLLECTION_METADATA],
      deployer.address,
      10,
    )).address, deployer, api);

    gemFactory = new Rmrk_factory(api, deployer);
    gem = new Rmrk((await gemFactory.new(
      ["Gem"],
      ['GM'],
      [BASE_URI],
      MAX_SUPPLY,
      PRICE_PER_MINT,
      [COLLECTION_METADATA],
      dave.address,
      100,
    )).address, deployer, api);
  }

  it('Setup Base', async () => {
    await setup();
    // This test follows MergesEquippable user story, but without Equipalble tests.
    // https://github.com/rmrk-team/evm-sample-contracts/tree/master/contracts/MergedEquippable
    // The scenarrio is different only when it comes to procedure of nesting child tokens,
    // but the end result is the same.

    // First Bob mints tokens from kanaria and gem contracts.
    // After Deployer (contract owner) adds new assets to gem and kanaria contracts, the same deployer will
    // add those assets to the tokens.
    // Bob accepts new assets on all of his tokens (both kanaria and gem tokens) 
    // Bob addds gem tokens (children) to kanaria tokens (parent)
    // Equipping TBA...

    const assetDefaultId = 1;
    const assetComposedId = 2;
    // bob mints 5 kanaria

    const { gasRequired } = await kanaria.withSigner(bob).query.mint(bob.address, 5);
    let kanariaMintResult = await kanaria.withSigner(bob).tx.mint(bob.address, 5, { value: PRICE_PER_MINT.muln(5), gasLimit: gasRequired * 2n });
    emit(kanariaMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: 1 }, });

    // bob mints 15 gem
    const gasRequiredGem = (await gem.withSigner(bob).query.mint(bob.address, 1)).gasRequired;
    for (let i = 1; i < 16; i++) {
      const gemMintResult = await gem.withSigner(bob).tx.mint(bob.address, 1, { value: PRICE_PER_MINT, gasLimit: gasRequiredGem * 2n });
      emit(gemMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: i }, });
    }
    expect((await gem.query.balanceOf(bob.address)).value).to.equal(15);

    // deployer adds two assets for kanaria
    const assetEntryGas = (await kanaria.withSigner(deployer).query.addAssetEntry(assetDefaultId, "1", "1", ["ipfs://default.png"], [1, 2, 3])).gasRequired;
    const addAssetResult = await kanaria.withSigner(deployer).tx.addAssetEntry(assetDefaultId, "1", "1", ["ipfs://default.png"], [], { gasLimit: assetEntryGas * 2n });
    emit(addAssetResult, 'AssetSet', { asset: 1 });
    await kanaria.withSigner(deployer).tx.addAssetEntry(assetComposedId, "1", "1", ["ipfs://meta1.json"], [1, 3, 5, 7, 9, 10, 11], { gasLimit: assetEntryGas * 2n });
    expect((await kanaria.withSigner(deployer).query.totalAssets())?.value.toString()).to.be.equal("2");

    // add both assets to token 1
    const assetAddGas = (await kanaria.withSigner(deployer).query.addAssetToToken({ u64: 1 }, assetDefaultId, { u64: 1 })).gasRequired;
    const addAssetTokenResult = await kanaria.withSigner(deployer).tx.addAssetToToken({ u64: 1 }, assetDefaultId, { u64: 1 }, { gasLimit: assetAddGas * 2n });
    emit(addAssetTokenResult, 'AssetAddedToToken', { token: { u64: 1 }, asset: 1, replaces: null });
    await kanaria.withSigner(deployer).tx.addAssetToToken({ u64: 1 }, assetComposedId, { u64: 1 }, { gasLimit: assetAddGas * 2n });
    expect((await kanaria.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,2");

    // bob accepts both assets
    const assetAcceptGas = (await kanaria.withSigner(bob).query.acceptAsset({ u64: 1 }, assetDefaultId)).gasRequired;
    await kanaria.withSigner(bob).tx.acceptAsset({ u64: 1 }, assetDefaultId, { gasLimit: assetAcceptGas });
    await kanaria.withSigner(bob).tx.acceptAsset({ u64: 1 }, assetComposedId, { gasLimit: assetAcceptGas * 2n });
    expect((await kanaria.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()).to.be.equal("2,0");

    // We'll add 4 assets for each gem, a full version and 3 versions matching each slot.
    // We will have only 2 types of gems -> 4x2: 8 assets.
    // This is not composed by others, so fixed and slot parts are never used.
    const equippableRefIdLeftGem = 1;
    const equippableRefIdMidGem = 2;
    const equippableRefIdRightGem = 3;
    const gemAssetAddGas = (await gem.withSigner(deployer).query.addAssetEntry(0, "1", "1", ["ipfs://gems/typeA/full.svg"], [])).gasRequired;
    await gem.withSigner(deployer).tx.addAssetEntry(1, 0, "1", ["ipfs://gems/typeA/full.svg"], [], { gasLimit: gemAssetAddGas });
    await gem.withSigner(deployer).tx.addAssetEntry(2, equippableRefIdLeftGem, "1", ["ipfs://gems/typeA/left.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(3, equippableRefIdMidGem, "1", ["ipfs://gems/typeA/mid.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(4, equippableRefIdRightGem, "1", ["ipfs://gems/typeA/right.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(5, 0, "1", ["ipfs://gems/typeB/full.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(6, equippableRefIdLeftGem, "1", ["ipfs://gems/typeB/left.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(7, equippableRefIdMidGem, "1", ["ipfs://gems/typeB/mid.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(8, equippableRefIdRightGem, "1", ["ipfs://gems/typeB/right.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    expect((await gem.withSigner(deployer).query.totalAssets())?.value.toString()).to.be.equal("8");

    // We add assets of type A to gem 1 and 2, and type Bto gem 3. Both are nested into the first kanaria
    // This means gems 1 and 2 will have the same asset, which is totally valid.
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 1 }, 1, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 1 }, 2, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 1 }, 3, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 1 }, 4, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 2 }, 1, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 2 }, 2, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 2 }, 3, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 2 }, 4, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 3 }, 5, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 3 }, 6, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 3 }, 7, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetToToken({ u64: 3 }, 8, { u64: 0 }, { gasLimit: assetAddGas * 2n });
    expect((await gem.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,4");
    expect((await gem.query.totalTokenAssets({ u64: 2 }))?.value.ok.toString()).to.be.equal("0,4");
    expect((await gem.query.totalTokenAssets({ u64: 3 }))?.value.ok.toString()).to.be.equal("0,4");

    // We accept each asset for all gems
    await gem.withSigner(bob).tx.acceptAsset({ u64: 1 }, 1, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 1 }, 2, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 1 }, 3, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 1 }, 4, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 2 }, 1, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 2 }, 2, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 2 }, 3, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 2 }, 4, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 3 }, 5, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 3 }, 6, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 3 }, 7, { gasLimit: assetAcceptGas * 2n });
    await gem.withSigner(bob).tx.acceptAsset({ u64: 3 }, 8, { gasLimit: assetAcceptGas * 2n });
    expect((await gem.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()).to.be.equal("4,0");
    expect((await gem.query.totalTokenAssets({ u64: 2 }))?.value.ok.toString()).to.be.equal("4,0");
    expect((await gem.query.totalTokenAssets({ u64: 3 }))?.value.ok.toString()).to.be.equal("4,0");

    // bob approves kanaria Contract on gem (for nesting gem on kanaria)
    const approveGas = (await gem.withSigner(bob).query.approve(kanaria.address, { u64: 1 }, true)).gasRequired;
    for (let i = 1; i < 16; i++) {
      await gem.withSigner(bob).tx.approve(kanaria.address, { u64: i }, true, { gasLimit: approveGas });
      expect((await gem.query.allowance(bob.address, kanaria.address, { u64: 1 })).value).to.equal(true);
    }
    // bob adds 3 gem nfts to bob's 5 kanaria nfts (kanaria is now parent of gem tokens)
    const addgemGas = (await kanaria.withSigner(bob).query.addChild({ u64: 1 }, [gem.address, { u64: 1 }])).gasRequired;
    for (let k = 1; k < 6; k++) {
      for (let g = 1; g < 4; g++) {
        const res = await kanaria.withSigner(bob).tx.addChild({ u64: k }, [gem.address, { u64: g }], { gasLimit: addgemGas * 2n });
        const balance = (await kanaria.query.childrenBalance({ u64: k }))?.value.ok.toString();
      }
      expect((await kanaria.query.childrenBalance({ u64: k }))?.value.ok.toString()).to.be.equal("3,0");
    }

    // set Base
    const setupBaseGas = (await gem.query.setupBase([BASE_METADATA])).gasRequired;
    await gem.withSigner(deployer).tx.setupBase([BASE_METADATA], { gasLimit: setupBaseGas * 2n });

    // const PART_LIST: Part[] = [
    //   {
    //     partType: PartType.slot,
    //     z: 0,
    //     equippable: [],
    //     metadataUri: ["ipfs://backgrounds/1.svg"],
    //     isEquippableByAll: true
    //   },
    //   {
    //     partType: PartType.fixed,
    //     z: 0,
    //     equippable: [],
    //     metadataUri: ["ipfs://backgrounds/2.svg"],
    //     isEquippableByAll: false
    //   },
    // ];
    // const addPartListGas = (await gem.withSigner(deployer).query.addPartList(PART_LIST)).gasRequired;
    // const tx = await gem.withSigner(deployer).tx.addPartList(PART_LIST, { gasLimit: addPartListGas * 2n });
    // console.log("result:  ", tx)
  })

})

// Helper function to parse Events
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function emit(result: { events?: any }, name: string, args: any): void {
  const event = result.events.find(
    (event: { name: string }) => event.name === name,
  );
  for (const key of Object.keys(event.args)) {
    if (event.args[key] instanceof ReturnNumber) {
      event.args[key] = event.args[key].toNumber();
    }
  }
  expect(event).eql({ name, args, });
}
