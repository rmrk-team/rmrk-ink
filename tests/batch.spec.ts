import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import { encodeAddress } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import BN from "bn.js";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable";
import Rmrk from "../types/contracts/rmrk_example_equippable";
import { RmrkError } from "../types/types-returns/rmrk_example_equippable";
import { SignAndSendSuccessResponse } from "@727-ventures/typechain-types";

import { emit } from "./helper";


use(chaiAsPromised);

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
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
  let users: KeyringPair[];
  let parent: Rmrk;
  let child: Rmrk;

  const ZERO_ADDRESS = encodeAddress(
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  );

  beforeEach(async function (): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
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
          [COLLECTION_METADATA],
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
          [COLLECTION_METADATA],
        )
      ).address,
      deployer,
      api
    );
  });

  it("deployer mints and transfers many works", async () => {
    const PARENT_TOKENS = 50;
    const CHILD_TOKENS = 50;
    const ASSET_ID1 = 1;

    // deployer mints many parent tokens
    await mintMany(parent, deployer, PARENT_TOKENS);
    expect(
      (await parent.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(PARENT_TOKENS);

    // deployer mints many child tokens
    await mintMany(child, deployer, CHILD_TOKENS);
    expect(
      (await child.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(CHILD_TOKENS);

    // create and add Asset to many tokens
    await child
      .withSigner(deployer)
      .tx.addAssetEntry(1, 0, ["ipfs://gems/typeA/full.svg"], []);
    var tokenList = new Array();
    for (let i = 1; i <= CHILD_TOKENS; i++) {
      tokenList.push({ u64: i });
    }
    await child
      .withSigner(deployer)
      .tx.addAssetToManyTokens(tokenList, ASSET_ID1);
    expect(
      (await child.query.totalTokenAssets({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("1,0");

    // deployer approves parent's Contract on child
    await approve(child, parent, deployer);

    // deployer adds each child nft to its parent
    var parentChildPair = new Array();
    for (let i = 1; i <= PARENT_TOKENS; i++) {
      parentChildPair.push([{ u64: i }, { u64: i }]);
    }
    await addManyChildren(parent, deployer, child, parentChildPair);
    // check that first token has 1 child
    expect(
      (await parent.query.childrenBalance({ u64: 1 }))?.value.unwrap().ok.toString()
    ).to.be.equal("1,0");
    // check that last token has 1 child
    expect(
      (await parent.query.childrenBalance({ u64: PARENT_TOKENS }))?.value.unwrap().ok.toString()
    ).to.be.equal("1,0");

  });
});


// helper function to mint a token
const mintMany = async (contract: Rmrk, signer: KeyringPair, mintAmount: number): Promise<void> => {
  // call mint function
  let mintResult = await contract
    .withSigner(signer)
    .tx.mintMany(
      signer.address,
      mintAmount,
    );
}

// helper function to add many children to many parents
const addManyChildren = async (contract: Rmrk, signer: KeyringPair, child: Rmrk, parentChildPair: any): Promise<void> => {
  console.log("parentChildPair", parentChildPair);

  let addResult = await contract
    .withSigner(signer)
    .tx.addManyChildren(
      child.address,
      parentChildPair,
    );

  // console.log("addResult", addResult.value.unwrap().ok);
}

// helper function to approve a token
const approve = async (child: Rmrk, parent: Rmrk, signer: KeyringPair): Promise<SignAndSendSuccessResponse> => {

  let approveResult = await child
    .withSigner(signer)
    .tx.approve(parent.address, null, true);
  expect(
    (await child.query.allowance(signer.address, parent.address, { u64: 1 }))
      .value.ok
  ).to.equal(true);
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
  return acceptChildResult;
}

// helper function to accept a child on parent contract
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
  return removeChildResult;
}

// helper function to create a batch of users
const createBatchUsers = async (api: ApiPromise, keyring: Keyring, amount: number): Promise<KeyringPair[]> => {
  const users = new Array();
  for (let i = 0; i < amount; i++) {
    users.push(keyring.addFromUri(`//user${i}`));
  }
  return users;
}