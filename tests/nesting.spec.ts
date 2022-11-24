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
  let dave: KeyringPair;
  let parent: Rmrk;
  let child: Rmrk;

  const ZERO_ADDRESS = encodeAddress(
    '0x0000000000000000000000000000000000000000000000000000000000000000',
  );
  let gasRequired: bigint;

  async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
    dave = keyring.addFromUri('//Dave');
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

  it('Add child (different user), approval works', async () => {
    await setup();

    // bob mints parent
    const mintGas = (await parent.withSigner(bob).query.mintNext()).gasRequired;
    let parentMintResult = await parent.withSigner(bob).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    emit(parentMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: 1 }, });

    // dave mints child
    let childMintResult = await child.withSigner(dave).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    expect((await child.query.totalSupply()).value.rawNumber.toNumber()).to.equal(1);
    emit(childMintResult, 'Transfer', { from: null, to: dave.address, id: { u64: 1 }, });

    // dave approves parentContract on child
    const approveGas = (await child.withSigner(dave).query.approve(parent.address, { u64: 1 }, true)).gasRequired;
    let approveResult = await child.withSigner(dave).tx.approve(parent.address, { u64: 1 }, true, { gasLimit: approveGas });
    expect((await child.query.allowance(dave.address, parent.address, { u64: 1 })).value).to.equal(true);
    emit(approveResult, 'Approval', { from: dave.address, to: parent.address, id: { u64: 1 }, approved: true, });

    // dave adds child nft to bob's parent nft
    const addChildGas = (await parent.withSigner(dave).query.addChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const addChildResult = await parent.withSigner(dave).tx.addChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: addChildGas });
    emit(addChildResult, 'ChildAdded', { to: { u64: 1 }, collection: child.address, child: { u64: 1 }, });
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,1");

    // since bob is owner of parent, dave can't accept child
    const failResult = await parent.withSigner(dave).query.acceptChild({ u64: 1 }, [child.address, { u64: 1 }]);
    expect(hex2a(failResult.value.err.custom)).to.be.equal('NotAuthorised');

    // bob accepts child
    const acceptChildGas = (await parent.withSigner(bob).query.acceptChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const acceptChildResult = await parent.withSigner(bob).tx.acceptChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: acceptChildGas });
    emit(acceptChildResult, 'ChildAccepted', { parent: { u64: 1 }, collection: child.address, child: { u64: 1 } });
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("1,0");

    // bob fails to accept already accepted child
    const failAcceptResult = await parent.withSigner(bob).query.acceptChild({ u64: 1 }, [child.address, { u64: 1 }]);
    expect(hex2a(failAcceptResult.value.err.custom)).to.be.equal('AlreadyAddedChild');

    // dave fails to remove child (not owner)
    const failRemoveChild = await parent.withSigner(dave).query.removeChild({ u64: 1 }, [child.address, { u64: 1 }]);
    expect(hex2a(failRemoveChild.value.err.custom)).to.be.equal('NotAuthorised');

    // bob removes child
    const removeChildGas = (await parent.withSigner(bob).query.removeChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const removeChildResult = await parent.withSigner(bob).tx.removeChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: removeChildGas });
    emit(removeChildResult, 'ChildRemoved', { parent: { u64: 1 }, childCollection: child.address, childTokenId: { u64: 1 }, });
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,0");
  })

  it('Add child (different user), reject works', async () => {
    await setup();

    // bob mints parent
    const mintGas = (await parent.withSigner(bob).query.mintNext()).gasRequired;
    let parentMintResult = await parent.withSigner(bob).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    emit(parentMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: 1 }, });

    // dave mints child
    let childMintResult = await child.withSigner(dave).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    expect((await child.query.totalSupply()).value.rawNumber.toNumber()).to.equal(1);
    emit(childMintResult, 'Transfer', { from: null, to: dave.address, id: { u64: 1 }, });

    // dave approves parentContract on child
    const approveGas = (await child.withSigner(dave).query.approve(parent.address, { u64: 1 }, true)).gasRequired;
    let approveResult = await child.withSigner(dave).tx.approve(parent.address, { u64: 1 }, true, { gasLimit: approveGas });
    expect((await child.query.allowance(dave.address, parent.address, { u64: 1 })).value).to.equal(true);
    emit(approveResult, 'Approval', { from: dave.address, to: parent.address, id: { u64: 1 }, approved: true, });

    // dave adds child nft to bob's parent nft
    const addChildGas = (await parent.withSigner(dave).query.addChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const addChildResult = await parent.withSigner(dave).tx.addChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: addChildGas });
    emit(addChildResult, 'ChildAdded', { to: { u64: 1 }, collection: child.address, child: { u64: 1 }, });
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,1");

    // since bob is owner of parent, dave can't accept child
    const failAcceptResult = await parent.withSigner(dave).query.acceptChild({ u64: 1 }, [child.address, { u64: 1 }]);
    expect(hex2a(failAcceptResult.value.err.custom)).to.be.equal('NotAuthorised');

    // since bob is owner of parent, dave fails to reject child
    const failRejectResult = await parent.withSigner(dave).query.rejectChild({ u64: 1 }, [child.address, { u64: 1 }]);
    expect(hex2a(failRejectResult.value.err.custom)).to.be.equal('NotAuthorised');

    // bob rejects child
    const rejectChildGas = (await parent.withSigner(bob).query.rejectChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const acceptChildResult = await parent.withSigner(bob).tx.rejectChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: rejectChildGas });
    emit(acceptChildResult, 'ChildRejected', { parent: { u64: 1 }, childCollection: child.address, childTokenId: { u64: 1 } });
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,0");
  })

  it('Add child (same user) works', async () => {
    await setup();

    // bob mints parent
    const mintGas = (await parent.withSigner(bob).query.mintNext()).gasRequired;
    let parentMintResult = await parent.withSigner(bob).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    emit(parentMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: 1 }, });

    // bob mints child
    let childMintResult = await child.withSigner(bob).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    expect((await child.query.totalSupply()).value.rawNumber.toNumber()).to.equal(1);
    emit(childMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: 1 }, });

    // bob approves parentContract on child
    const approveGas = (await child.withSigner(bob).query.approve(parent.address, { u64: 1 }, true)).gasRequired;
    let approveResult = await child.withSigner(bob).tx.approve(parent.address, { u64: 1 }, true, { gasLimit: approveGas });
    expect((await child.query.allowance(bob.address, parent.address, { u64: 1 })).value).to.equal(true);
    emit(approveResult, 'Approval', { from: bob.address, to: parent.address, id: { u64: 1 }, approved: true, });

    // bob adds child nft to parent
    const addChildGas = (await parent.withSigner(bob).query.addChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const result = await parent.withSigner(bob).query.addChild({ u64: 1 }, [child.address, { u64: 1 }]);
    const addChildResult = await parent.withSigner(bob).tx.addChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: addChildGas });
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("1,0");
    emit(addChildResult, 'ChildAdded', { to: { u64: 1 }, collection: child.address, child: { u64: 1 }, });

    // since bob is owner of both parent and child it is automatically approved
    emit(addChildResult, 'ChildAccepted', { parent: { u64: 1 }, collection: child.address, child: { u64: 1 }, });
  })

  it('Add two parents, move/transfer child works', async () => {
    await setup();

    // bob mints parent-1
    const mintGas = (await parent.withSigner(bob).query.mintNext()).gasRequired;
    let parentMintResult = await parent.withSigner(bob).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    emit(parentMintResult, 'Transfer', { from: null, to: bob.address, id: { u64: 1 }, });

    // dave mints parent-2
    let parentMintResult2 = await parent.withSigner(dave).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    expect((await parent.query.totalSupply()).value.rawNumber.toNumber()).to.equal(2);
    emit(parentMintResult2, 'Transfer', { from: null, to: dave.address, id: { u64: 2 }, });

    // dave mints a child and approves parentContract on child
    let childMintResult = await child.withSigner(dave).tx.mintNext({ value: PRICE_PER_MINT, gasLimit: mintGas * 2n });
    expect((await child.query.totalSupply()).value.rawNumber.toNumber()).to.equal(1);
    emit(childMintResult, 'Transfer', { from: null, to: dave.address, id: { u64: 1 }, });
    const approveGas = (await child.withSigner(dave).query.approve(parent.address, { u64: 1 }, true)).gasRequired;
    let approveResult = await child.withSigner(dave).tx.approve(parent.address, { u64: 1 }, true, { gasLimit: approveGas });
    expect((await child.query.allowance(dave.address, parent.address, { u64: 1 })).value).to.equal(true);
    emit(approveResult, 'Approval', { from: dave.address, to: parent.address, id: { u64: 1 }, approved: true, });

    // dave adds child nft to his parent-2 nft
    const addChildGas = (await parent.withSigner(dave).query.addChild({ u64: 2 }, [child.address, { u64: 1 }])).gasRequired;
    const addChildResult = await parent.withSigner(dave).tx.addChild({ u64: 2 }, [child.address, { u64: 1 }], { gasLimit: addChildGas });
    emit(addChildResult, 'ChildAdded', { to: { u64: 2 }, collection: child.address, child: { u64: 1 }, });
    expect((await parent.query.childrenBalance({ u64: 2 }))?.value.ok.toString()).to.be.equal("1,0");


    // dave transfers his child-1 from parent-2 to bob's parent-1, bob accepts the child
    const transferChildGas = (await parent.withSigner(dave).query.transferChild({ u64: 2 }, { u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const transferChildResult = await parent.withSigner(dave).tx.transferChild({ u64: 2 }, { u64: 1 }, [child.address, { u64: 1 }], { gasLimit: transferChildGas });
    emit(transferChildResult, 'ChildRemoved', { parent: { u64: 2 }, childCollection: child.address, childTokenId: { u64: 1 }, });
    expect((await parent.query.childrenBalance({ u64: 2 }))?.value.ok.toString()).to.be.equal("0,0");
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,1");

    // bob accepts new child
    const acceptChildGas = (await parent.withSigner(bob).query.acceptChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const acceptChildResult = await parent.withSigner(bob).tx.acceptChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: acceptChildGas });
    emit(acceptChildResult, 'ChildAccepted', { parent: { u64: 1 }, collection: child.address, child: { u64: 1 } });
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("1,0");

    // parent contract owns child token (in child contract)
    expect((await child.query.ownerOf({ u64: 1 })).value).to.equal(parent.address);

    // bob removes child
    const removeChildGas = (await parent.withSigner(bob).query.removeChild({ u64: 1 }, [child.address, { u64: 1 }])).gasRequired;
    const removeChildResult = await parent.withSigner(bob).tx.removeChild({ u64: 1 }, [child.address, { u64: 1 }], { gasLimit: removeChildGas });
    emit(removeChildResult, 'ChildRemoved', { parent: { u64: 1 }, childCollection: child.address, childTokenId: { u64: 1 }, });
    expect((await parent.query.childrenBalance({ u64: 1 }))?.value.ok.toString()).to.be.equal("0,0");

    // bob owns child token (in child contract)
    expect((await child.query.ownerOf({ u64: 1 })).value).to.equal(bob.address);
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