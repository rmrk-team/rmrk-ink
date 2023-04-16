import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import { encodeAddress } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import BN from "bn.js";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable_lazy";
import Rmrk from "../types/contracts/rmrk_example_equippable_lazy";
import { RmrkError } from "../types/types-returns/rmrk_example_equippable_lazy";
import { SignAndSendSuccessResponse } from "@727-ventures/typechain-types";

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

describe("RMRK Nesting tests", () => {
  let parentFactory: Rmrk_factory;
  let childFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let bob: KeyringPair;
  let dave: KeyringPair;
  let parent: Rmrk;
  let child: Rmrk;

  const ZERO_ADDRESS = encodeAddress(
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  );

  beforeEach(async function (): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true });
    deployer = keyring.addFromUri("//Alice");
    bob = keyring.addFromUri("//Bob");
    dave = keyring.addFromUri("//Dave");
    parentFactory = new Rmrk_factory(api, deployer);
    parent = new Rmrk(
      (
        await parentFactory.new(
          ["RmrkProject 1"],
          ["RMKPARENT"],
          [BASE_URI],
          MAX_SUPPLY,
          PRICE_PER_MINT,
          [COLLECTION_METADATA],
          ZERO_ADDRESS,
          0
        )
      ).address,
      deployer,
      api
    );

    childFactory = new Rmrk_factory(api, deployer);
    child = new Rmrk(
      (
        await childFactory.new(
          ["RmrkProject 2"],
          ["RMKCHILD"],
          [BASE_URI],
          MAX_SUPPLY,
          PRICE_PER_MINT,
          [COLLECTION_METADATA],
          ZERO_ADDRESS,
          0
        )
      ).address,
      deployer,
      api
    );
  });

  it("Init two rmrk contracts works", async () => {
    expect(
      (await parent.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(0);
    expect(
      (await parent.query.hasRole(ADMIN_ROLE, deployer.address)).value.ok
    ).to.equal(true);
    expect((await parent.query.maxSupply()).value.unwrap()).to.equal(MAX_SUPPLY);
    expect((await parent.query.price()).value.unwrap().toString()).to.equal(
      PRICE_PER_MINT.toString()
    );
    const parentCollectionId = (await parent.query.collectionId()).value.ok.bytes.toString();

    expect(
      (await child.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(0);
    expect(
      (await child.query.hasRole(ADMIN_ROLE, deployer.address)).value.ok
    ).to.equal(true);
    expect((await child.query.maxSupply()).value.unwrap()).to.equal(MAX_SUPPLY);
    expect((await child.query.price()).value.unwrap().toString()).to.equal(
      PRICE_PER_MINT.toString()
    );
    const childCollectionId = (await child.query.collectionId()).value.ok.bytes.toString();
    expect(parentCollectionId).to.not.be.equal(childCollectionId);
  });

  it("Add child (different user), approval works", async () => {
    // bob mints parent
    const parentMintResult = await mintOne(parent, bob);

    // dave mints child
    const childMintResult = await mintOne(child, dave);

    // dave approves parent's Contract on child
    await approve(child, parent, dave);

    // dave adds child nft to bob's parent nft
    await addChild(child, parent, dave);
    expect(
      (await parent.query.childrenBalance({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("0,1");

    // since bob is owner of parent, dave can't accept child
    const failResult = await parent
      .withSigner(dave)
      .query.acceptChild({ u64: 1 }, [child.address, { u64: 1 }]);
    expect(failResult.value.unwrap().err.rmrk).to.be.equal(RmrkError.notTokenOwner);

    // bob accepts child
    await acceptChild(child, parent, bob);

    // bob fails to accept already accepted child
    const failAcceptResult = await parent
      .withSigner(bob)
      .query.acceptChild({ u64: 1 }, [child.address, { u64: 1 }]);
    expect(failAcceptResult.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.alreadyAddedChild
    );

    // dave fails to remove child (not owner)
    const failRemoveChild = await parent
      .withSigner(dave)
      .query.removeChild({ u64: 1 }, [child.address, { u64: 1 }]);
    expect(failRemoveChild.value.unwrap().err.rmrk).to.be.equal(RmrkError.notTokenOwner);

    // bob removes child
    await removeChild(child, parent, bob);
  });

  it("Add child (different user), reject works", async () => {

    // bob mints parent
    await mintOne(parent, bob);

    // dave mints child
    await mintOne(child, dave);

    await approve(child, parent, dave);

    // dave adds child nft to bob's parent nft
    await addChild(child, parent, dave);
    expect(
      (await parent.query.childrenBalance({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("0,1");

    // since bob is owner of parent, dave can't accept child
    const failAcceptResult = await parent
      .withSigner(dave)
      .query.acceptChild({ u64: 1 }, [child.address, { u64: 1 }]);

    expect(failAcceptResult.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.notTokenOwner
    );

    // since bob is owner of parent, dave fails to reject child
    const failRejectResult = await parent
      .withSigner(dave)
      .query.rejectChild({ u64: 1 }, [child.address, { u64: 1 }]);

    expect(failRejectResult.value.unwrap().err.rmrk).to.be.equal(
      RmrkError.notTokenOwner
    );

    // bob rejects child
    const acceptChildResult = await parent
      .withSigner(bob)
      .tx.rejectChild({ u64: 1 }, [child.address, { u64: 1 }]);
    emit(acceptChildResult, "ChildRejected", {
      parent: { u64: 1 },
      childCollection: child.address,
      childTokenId: { u64: 1 },
    });
    expect(
      (await parent.query.childrenBalance({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("0,0");
  });

  it("Add child (same user) works", async () => {

    // bob mints parent
    await mintOne(parent, bob);

    // bob mints child
    await mintOne(child, bob);


    // bob approves parentContract on child
    await approve(child, parent, bob);

    // bob adds child nft to parent
    const addChildResult = await addChild(child, parent, bob);
    expect(
      (await parent.query.childrenBalance({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("1,0");

    // since bob is owner of both parent and child it is automatically approved
    emit(addChildResult, "ChildAccepted", {
      parent: { u64: 1 },
      collection: child.address,
      child: { u64: 1 },
    });
  });

  it("Add two parents, move/transfer child works", async () => {
    // bob mints token1 parent
    await mintOne(parent, bob);

    // dave mints token2 on parent
    await mintOne(parent, dave, 2);

    // dave mints a child and approves parent's contract for child nft
    await mintOne(child, dave);
    await approve(child, parent, dave);

    // dave adds child nft to his parent token2
    await addChild(child, parent, dave, 2);

    // dave transfers his child-1 from parent-2 to bob's parent-1, bob accepts the child
    const transferChildResult = await parent
      .withSigner(dave)
      .tx.transferChild({ u64: 2 }, { u64: 1 }, [child.address, { u64: 1 }]);
    emit(transferChildResult, "ChildRemoved", {
      parent: { u64: 2 },
      childCollection: child.address,
      childTokenId: { u64: 1 },
    });
    expect(
      (await parent.query.childrenBalance({ u64: 2 }))?.value.unwrap().ok.toString()
    ).to.be.equal("0,0");
    expect(
      (await parent.query.childrenBalance({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("0,1");

    // bob accepts new child
    await acceptChild(child, parent, bob)

    // parent contract owns child token (in child contract)
    expect((await child.query.ownerOf({ u64: 1 })).value.unwrap()).to.equal(
      parent.address
    );

    // bob removes child from parent token2
    await removeChild(child, parent, bob);

    // bob now owns child token (in child contract). Remember that Dave originally minted it.
    expect((await child.query.ownerOf({ u64: 1 })).value.unwrap()).to.equal(bob.address);
  });
});


// helper function to mint a token
const mintOne = async (contract: Rmrk, signer: KeyringPair, token?: number): Promise<SignAndSendSuccessResponse> => {
  // call mint function
  let mintResult = await contract
    .withSigner(signer)
    .tx.mint({
      value: PRICE_PER_MINT,
    }
    );
  emit(mintResult, "Transfer", {
    from: null,
    to: signer.address,
    id: { u64: token ? token : 1 },
  });
  return mintResult;
}

// helper function to approve a token
const approve = async (child: Rmrk, parent: Rmrk, signer: KeyringPair): Promise<SignAndSendSuccessResponse> => {

  let approveResult = await child
    .withSigner(signer)
    .tx.approve(parent.address, { u64: 1 }, true);
  expect(
    (await child.query.allowance(signer.address, parent.address, { u64: 1 }))
      .value.ok
  ).to.equal(true);
  emit(approveResult, "Approval", {
    from: signer.address,
    to: parent.address,
    id: { u64: 1 },
    approved: true,
  });
  return approveResult;
}

// helper function to add a child to parent contract
const addChild = async (child: Rmrk, parent: Rmrk, signer: KeyringPair, parentToken?: number): Promise<SignAndSendSuccessResponse> => {
  const addChildResult = await parent
    .withSigner(signer)
    .tx.addChild({ u64: parentToken ? parentToken : 1 }, [child.address, { u64: 1 }]);
  emit(addChildResult, "ChildAdded", {
    to: { u64: parentToken ? parentToken : 1 },
    collection: child.address,
    child: { u64: 1 },
  });

  return addChildResult;
}

// helper function to accept a child on parent contract
const acceptChild = async (child: Rmrk, parent: Rmrk, signer: KeyringPair): Promise<SignAndSendSuccessResponse> => {
  const acceptChildResult = await parent
    .withSigner(signer)
    .tx.acceptChild({ u64: 1 }, [child.address, { u64: 1 }]);
  emit(acceptChildResult, "ChildAccepted", {
    parent: { u64: 1 },
    collection: child.address,
    child: { u64: 1 },
  });
  expect(
    (await parent.query.childrenBalance({ u64: 1 }))?.value.unwrap().ok.toString()
  ).to.be.equal("1,0");
  expect((await child.query.getParentOfChild([child.address, { u64: 1 }])).value.unwrap().u64)
    .to.be.equal(1);

  return acceptChildResult;
}

// helper function to remove a child on parent contract
const removeChild = async (child: Rmrk, parent: Rmrk, signer: KeyringPair): Promise<SignAndSendSuccessResponse> => {
  const removeChildResult = await parent
    .withSigner(signer)
    .tx.removeChild({ u64: 1 }, [child.address, { u64: 1 }]);
  emit(removeChildResult, "ChildRemoved", {
    parent: { u64: 1 },
    childCollection: child.address,
    childTokenId: { u64: 1 },
  });
  expect(
    (await parent.query.childrenBalance({ u64: 1 }))?.value.unwrap().ok.toString()
  ).to.be.equal("0,0");
  expect((await child.query.getParentOfChild([child.address, { u64: 1 }])).value.ok)
    .to.be.equal(null);

  return removeChildResult;
}