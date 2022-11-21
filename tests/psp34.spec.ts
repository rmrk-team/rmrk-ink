import { expect, use } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { encodeAddress } from '@polkadot/keyring';
import BN from 'bn.js';
import Rmrk_factory from '../types/constructors/rmrk_contract';
import Rmrk from '../types/contracts/rmrk_contract';

import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
// import { AccountId } from '../types/types-arguments/rmrk_contract';
// import { ReturnNumber } from '@supercolony/typechain-types';

use(chaiAsPromised);

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const TOKEN_URI_1 = "ipfs://tokenUriPrefix/1.json";
const TOKEN_URI_5 = "ipfs://tokenUriPrefix/5.json";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;

// Create a new instance of contract
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });

describe('Minting rmrk as psp34 tests', () => {
  let rmrkFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;
  let contract: Rmrk;

  const gasLimit = 18750000000;
  const ZERO_ADDRESS = encodeAddress(
    '0x0000000000000000000000000000000000000000000000000000000000000000',
  );
  let gasRequired: bigint;

  async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri('//Alice');
    wallet = keyring.addFromUri('//Bob');
    rmrkFactory = new Rmrk_factory(api, deployer);
    contract = new Rmrk((await rmrkFactory.new(
      ["RmrkProject"],
      ['RMK'],
      [BASE_URI],
      MAX_SUPPLY,
      PRICE_PER_MINT,
      [COLLECTION_METADATA],
      ZERO_ADDRESS,
      0,
    )).address, deployer, api);
  }

  it('create collection works', async () => {
    await setup();
    const queryList = await contract.query;
    // console.log("Query list for rmrk:", queryList);
    expect((await contract.query.totalSupply()).value.rawNumber.toNumber()).to.equal(0);
    expect((await contract.query.owner()).value).to.equal(deployer.address);
    expect((await contract.query.maxSupply()).value).to.equal(MAX_SUPPLY);
    expect((await contract.query.price()).value.rawNumber.toString()).to.equal(PRICE_PER_MINT.toString());
  })

  it('mintNext works', async () => {
    await setup();
    const tokenId = 1;

    expect((await contract.query.totalSupply()).value.rawNumber.toNumber()).to.equal(0);

    // mint
    const { gasRequired } = await contract.withSigner(wallet).query.mintNext();
    console.log("gasRequired", gasRequired);
    await contract.withSigner(wallet).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: gasRequired * 2n});

    // verify minting results. The totalSupply value is BN
    expect((await contract.query.totalSupply()).value.rawNumber.toNumber()).to.equal(1);

    expect((await contract.query.balanceOf(wallet.address)).value).to.equal(1);
    expect((await contract.query.ownerOf({u64: tokenId})).value).to.equal(wallet.address);

    // verify tokenUri call
    // expect((await contract.query.tokenUri(1))).to.equal(TOKEN_URI_1);
  })

  // it('mint 5 tokens works', async () => {
  //     const { contract, bob, uniquesCollectionId} = await setup();

  //     await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);
  //     await expect(contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 5, {value: PRICE_PER_MINT.muln(5)})).to.be.eventually.fulfilled;
  //     await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(new BN("5"));
  //     await expect(contract.query["rmrkMintable::tokenUri"](5)).to.eventually.have.property('output').to.equal(TOKEN_URI_5);
  //     await expect(contract.query["psp34::ownerOf"]({u64: 5})).to.eventually.have.property('output').to.equal(bob.address);

  //     // verify that the item is created in the Uniques pallet
  //     await expect((await api.query.uniques.asset(uniquesCollectionId, 5)).toHuman()?.owner).to.be.equal(bob.address);

  // })

  // it('token transfer works', async () => {
  //     const { owner, contract, bob, uniquesCollectionId } = await setup();

  //     // Bob mints 1 token
  //     await expect(contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 1, {value: PRICE_PER_MINT})).to.be.eventually.fulfilled;
  //     await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(new BN("1"));

  //     // verify that the owner in the Uniques pallet is Bob
  //     await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.owner).to.be.equal(bob.address);

  //     // Bob transfers token to Owner
  //     await expect(contract.connect(bob).tx["psp34::transfer"](owner.address, {u64:1}, [])).to.be.eventually.fulfilled;
  //     await expect(contract.query["psp34::balanceOf"](owner.address)).to.eventually.have.property('output').to.equal(new BN("1"));
  //     await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(0);
  //     await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(owner.address);

  //     // verify that the owner in the Uniques pallet is "owner"
  //     await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.owner).to.be.equal(owner.address);
  // })

  // it('token aprove works', async () => {
  //     const { owner, contract, bob, uniquesCollectionId } = await setup();

  //     // Bob mints 1 token
  //     await expect(contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 1, {value: PRICE_PER_MINT})).to.be.eventually.fulfilled;
  //     await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(new BN("1"));

  //     // verify that the owner in the Uniques pallet is Bob
  //     await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.owner).to.be.equal(bob.address);
  //     await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(bob.address);

  //     // Bob approves owner to be operator of the token
  //     await expect(contract.connect(bob).tx["psp34::approve"](owner.address, {u64:1}, true)).to.be.eventually.fulfilled;
  //     await expect(contract.query["psp34::allowance"](bob.address, owner.address, {u64:1})).to.eventually.have.property('output').to.equal(true);

  //     // verify that Bob is still the owner
  //     await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.owner).to.be.equal(bob.address);
  //     await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(bob.address);

  //     // verify allowance in Uniques pallet
  //     await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.approved).to.be.equal(owner.address);
  // })
})