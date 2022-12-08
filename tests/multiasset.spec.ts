import { expect, use } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { encodeAddress } from '@polkadot/keyring';
import BN from 'bn.js';
import Rmrk_factory from '../types/constructors/rmrk_contract';
import Rmrk from '../types/contracts/rmrk_contract';

import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
// import { AccountId } from '../types/types-arguments/rmrk_contract';
import { ReturnNumber } from '@supercolony/typechain-types';

use(chaiAsPromised);

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const TOKEN_URI_1 = "ipfs://tokenUriPrefix/1.json";
const TOKEN_URI_5 = "ipfs://tokenUriPrefix/5.json";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;
// const ASSET1 = {{id: "1"},
//   {equippableGroupId: "1"},
//   baseId: "1",
//   assetUri: "ipfs://assetUri/",
//   partIds: [1, 2, 3]};

// Create a new instance of contract
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });

describe('RMRK Multi Asset tests', () => {
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

  it('Init two rmrk contracts works', async () => {
    await setup();
    expect((await kanaria.query.totalSupply()).value.rawNumber.toNumber()).to.equal(0);
    expect((await kanaria.query.owner()).value).to.equal(deployer.address);
    expect((await kanaria.query.maxSupply()).value).to.equal(MAX_SUPPLY);
    expect((await kanaria.query.price()).value.rawNumber.toString()).to.equal(PRICE_PER_MINT.toString());
    const kanariaCollectionId = (await kanaria.query.collectionId()).value;

    expect((await gem.query.totalSupply()).value.rawNumber.toNumber()).to.equal(0);
    expect((await gem.query.owner()).value).to.equal(deployer.address);
    expect((await gem.query.maxSupply()).value).to.equal(MAX_SUPPLY);
    expect((await gem.query.price()).value.rawNumber.toString()).to.equal(PRICE_PER_MINT.toString());
    const gemCollectionId = (await gem.query.collectionId()).value;
    expect(kanariaCollectionId).to.not.be.equal(gemCollectionId);
  })

  it('Add assets to token and approve them works', async () => {
    await setup();

    // The mintToken function should accept two arguments (Kanaria and Gem). We will prepare a batch of transactions
    // to mint the tokens and send them. Once the tokens are minted, we will output the total number of tokens minted.
    // While the Kanaria tokens will be minted to the tokenOwner address, the Gem tokens will be minted 
    // using the nestMint method in order to be minted directly to the Kanaria tokens. 
    // We will mint three Gem tokens to each Kanaria. 
    // Since all of the nested tokens need to be approved, 
    // we will also build a batch of transaction to accept a single nest-minted Gem for each Kanaria:

    const assetDefaultId = 1;
    const assetComposedId = 2;
    // bob mints 5 kanaria
    const { gasRequired } = await kanaria.withSigner(bob).query.mint(bob.address, 5);
    let kanariaMintResult = await kanaria.withSigner(bob).tx.mint(bob.address, 5, { value: PRICE_PER_MINT.muln(5), gasLimit: gasRequired * 2n });
    emit(kanariaMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: 1 }, });

    // bob mints 15 gem
    const gasRequiredGem = (await gem.withSigner(bob).query.mint(bob.address, 5)).gasRequired;
    const res = await gem.withSigner(bob).tx.mint(bob.address, 5, { value: PRICE_PER_MINT.muln(5), gasLimit: gasRequiredGem * 2n });
    const res2 = await gem.withSigner(bob).tx.mint(bob.address, 5, { value: PRICE_PER_MINT.muln(5), gasLimit: gasRequiredGem * 2n });
    const gemMintResult = await gem.withSigner(bob).tx.mint(bob.address, 5, { value: PRICE_PER_MINT.muln(5), gasLimit: gasRequiredGem * 2n });
    // emit(gemMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: 15 }, }); // TODO 11 is emitted
    expect((await gem.query.balanceOf(bob.address)).value).to.equal(15);

    // bob approves kanariaContract on gem
    const approveGas = (await gem.withSigner(bob).query.approve(kanaria.address, { u64: 1 }, true)).gasRequired;
    for (let i = 1; i < 16; i++) {
      await gem.withSigner(bob).tx.approve(kanaria.address, { u64: i }, true, { gasLimit: approveGas });
      expect((await gem.query.allowance(bob.address, kanaria.address, { u64: 1 })).value).to.equal(true);
    }
    // bob adds 3 gem nfts to bob's 5 kanaria nfts
    const addgemGas = (await kanaria.withSigner(bob).query.addChild({ u64: 1 }, [gem.address, { u64: 1 }])).gasRequired;
    for (let k = 1; k < 6; k++) {
      for (let g = 1; g < 4; g++) {
        const res = await kanaria.withSigner(bob).tx.addChild({ u64: k }, [gem.address, { u64: g }], { gasLimit: addgemGas * 2n });
        const balance = (await kanaria.query.childrenBalance({ u64: k }))?.value.ok.toString();
      }
      expect((await kanaria.query.childrenBalance({ u64: k }))?.value.ok.toString()).to.be.equal("3,0");
    }

    // deployer adds two resource for kanaria
    const assetEntryGas = (await kanaria.withSigner(deployer).query.addAssetEntry(assetDefaultId, "1", "1", ["ipfs://default.png"], [1, 2, 3])).gasRequired;
    const ass = await kanaria.withSigner(deployer).tx.addAssetEntry(assetDefaultId, "1", "1", ["ipfs://default.png"], [], { gasLimit: assetEntryGas * 2n });
    await kanaria.withSigner(deployer).tx.addAssetEntry(2, "1", "1", ["ipfs://meta1.json"], [1, 3, 5, 7, 9, 10, 11], { gasLimit: assetEntryGas * 2n });
    expect((await kanaria.withSigner(deployer).query.totalAssets())?.value.toString()).to.be.equal("2");

    // add both assets to token 1
    const assetAddGas = (await kanaria.withSigner(deployer).query.addAssetToToken({ u64: 1 }, assetDefaultId, { u64: 1 })).gasRequired;
    await kanaria.withSigner(deployer).tx.addAssetToToken({ u64: 1 }, assetDefaultId, { u64: 1 }, { gasLimit: assetAddGas * 2n });
    await kanaria.withSigner(deployer).tx.addAssetToToken({ u64: 1 }, assetComposedId, { u64: 1 }, { gasLimit: assetAddGas * 2n });
    expect((await kanaria.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,2");
    
    // bob accepts both assets
    const assetAcceptGas = (await kanaria.withSigner(bob).query.acceptAsset({ u64: 1 }, assetDefaultId)).gasRequired;
    await kanaria.withSigner(bob).tx.acceptAsset({ u64: 1 }, assetDefaultId, { gasLimit: assetAcceptGas });
    await kanaria.withSigner(bob).tx.acceptAsset({ u64: 1 }, assetComposedId, { gasLimit: assetAcceptGas * 2n });
    // expect ((await kanaria.withSigner(deployer).query.getAcceptedTokenAssets({u64: 1}))?.value).to.be.equal({ok: [1, 2]});
    expect((await kanaria.query.totalTokenAssets({ u64: 1 }))?.value.ok.toString()).to.be.equal("2,0");
    
    // We'll add 4 assets for each gem, a full version and 3 versions matching each slot.
    // We will have only 2 types of gems -> 4x2: 8 assets.
    // This is not composed by others, so fixed and slot parts are never used.
    const equippableRefIdLeftGem = 1;
    const equippableRefIdMidGem = 2;
    const equippableRefIdRightGem = 3;
    const gemAssetAddGas = (await gem.withSigner(deployer).query.addAssetEntry(0, "1", "1", ["ipfs://gems/typeA/full.svg"], [])).gasRequired;
    await gem.withSigner(deployer).tx.addAssetEntry(1, 0, "1", ["ipfs://gems/typeA/full.svg"], [], { gasLimit: gemAssetAddGas });
    await gem.withSigner(deployer).tx.addAssetEntry(2, equippableRefIdLeftGem, "1", ["ipfs://gems/typeA/left.svg"], [], { gasLimit: gemAssetAddGas *2n });
    await gem.withSigner(deployer).tx.addAssetEntry(3, equippableRefIdMidGem, "1", ["ipfs://gems/typeA/mid.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(4, equippableRefIdRightGem, "1", ["ipfs://gems/typeA/right.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(5, 0, "1", ["ipfs://gems/typeB/full.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(6, equippableRefIdLeftGem, "1", ["ipfs://gems/typeB/left.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(7, equippableRefIdMidGem, "1", ["ipfs://gems/typeB/mid.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    await gem.withSigner(deployer).tx.addAssetEntry(8, equippableRefIdRightGem, "1", ["ipfs://gems/typeB/right.svg"], [], { gasLimit: gemAssetAddGas * 2n });
    expect((await gem.withSigner(deployer).query.totalAssets())?.value.toString()).to.be.equal("8");


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

// Helper function to convert error code to string
function hex2a(psp34CustomError: any): string {
  var hex = psp34CustomError.toString(); //force conversion
  var str = '';
  for (var i = 0; i < hex.length; i += 2)
    str += String.fromCharCode(parseInt(hex.substr(i, 2), 16));
  return str.substring(1);
}