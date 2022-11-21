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

describe('RMRK Nesting tests', () => {
  let parentFactory: Rmrk_factory;
  let childFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let bob: KeyringPair;
  let parent: Rmrk;
  let child: Rmrk;

  const gasLimit = 18750000000;
  const ZERO_ADDRESS = encodeAddress(
    '0x0000000000000000000000000000000000000000000000000000000000000000',
  );
  let gasRequired: bigint;

  async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
    parentFactory = new Rmrk_factory(api, deployer);
    parent = new Rmrk((await parentFactory.new(
      ["RmrkProject 1"],
      ['RMKPARENT'],
      [BASE_URI],
      MAX_SUPPLY,
      PRICE_PER_MINT,
      [COLLECTION_METADATA],
      ZERO_ADDRESS,
      0,
    )).address, deployer, api);

    childFactory = new Rmrk_factory(api, deployer);
    child = new Rmrk((await childFactory.new(
      ["RmrkProject 2"],
      ['RMKCHILD'],
      [BASE_URI],
      MAX_SUPPLY,
      PRICE_PER_MINT,
      [COLLECTION_METADATA],
      ZERO_ADDRESS,
      0,
    )).address, deployer, api);
  }

  it('Init two rmrk contracts works', async () => {
    await setup();
    expect((await parent.query.totalSupply()).value.rawNumber.toNumber()).to.equal(0);
    expect((await parent.query.owner()).value).to.equal(deployer.address);
    expect((await parent.query.maxSupply()).value).to.equal(MAX_SUPPLY);
    expect((await parent.query.price()).value.rawNumber.toString()).to.equal(PRICE_PER_MINT.toString());
    const parentCollectionId = (await parent.query.collectionId()).value;

    expect((await child.query.totalSupply()).value.rawNumber.toNumber()).to.equal(0);
    expect((await child.query.owner()).value).to.equal(deployer.address);
    expect((await child.query.maxSupply()).value).to.equal(MAX_SUPPLY);
    expect((await child.query.price()).value.rawNumber.toString()).to.equal(PRICE_PER_MINT.toString());
    const childCollectionId = (await child.query.collectionId()).value;
    expect(parentCollectionId).to.not.be.equal(childCollectionId);
  })

  it('Add child works', async () => {
    await setup();

    // bob mints parent
    const mintGas = (await parent.withSigner(bob).query.mintNext()).gasRequired;
    await parent.withSigner(bob).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    expect((await parent.query.totalSupply()).value.rawNumber.toNumber()).to.equal(1);

    // bob mints child
    await child.withSigner(bob).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    expect((await child.query.totalSupply()).value.rawNumber.toNumber()).to.equal(1);

    // bob approves parentContract on child
    const approveGas = (await child.withSigner(bob).query.approve(parent.address, { u64: 1 }, true)).gasRequired;
    await child.withSigner(bob).tx.approve(parent.address, { u64: 1 }, true, { gasLimit: approveGas });
    expect((await child.query.allowance(bob.address, parent.address, { u64: 1 })).value).to.equal(true);

    // bob adds child nft to parent
    const addChildGas = (await parent.withSigner(bob).query.addChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    await parent.withSigner(bob).tx.addChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: addChildGas });
    expect((await parent.query.childrenBalance())?.value.ok.toString()).to.be.equal("1,0");
  })

})