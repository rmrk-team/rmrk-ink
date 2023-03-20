import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import { encodeAddress } from "@polkadot/keyring";
import type { WeightV2 } from '@polkadot/types/interfaces'
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import BN from "bn.js";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable_lazy";
import Rmrk from "../types/contracts/rmrk_example_equippable_lazy";
import { RmrkError } from "../types/types-returns/rmrk_example_equippable_lazy";
import { emit } from "./helper";

import { SignAndSendSuccessResponse } from "@727-ventures/typechain-types";

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

describe("Minting rmrk as psp34, using MintingLazy trait from rmrk_example_equippable_lazy", () => {
  let rmrkFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let bob: KeyringPair;
  let contract: Rmrk;

  const ZERO_ADDRESS = encodeAddress(
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  );

  beforeEach(async function () {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri("//Alice");
    bob = keyring.addFromUri("//Bob");
    rmrkFactory = new Rmrk_factory(api, deployer);
    contract = new Rmrk(
      (
        await rmrkFactory.new(
          ["RmrkProject"],
          ["RMK"],
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
    // console.log("contract address", contract.address);
  });

  it("create collection works", async () => {
    expect(
      (await contract.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(0);
    expect(
      (await contract.query.hasRole(ADMIN_ROLE, deployer.address)).value.unwrap()
    ).to.equal(true);
    expect((await contract.query.maxSupply()).value.unwrap()).to.equal(MAX_SUPPLY);
    expect((await contract.query.price()).value.unwrap().toString()).to.equal(
      PRICE_PER_MINT.toString()
    );

    const collectionId = (await contract.query.collectionId()).value.unwrap().toString();

    // expect((await contract.query.getAttribute(collectionId, ["baseUri"])).value).to.equal(BASE_URI);
    // expect((await contract.query.getAttribute(collectionId, ["baseUri"])).value).to.equal(BASE_URI);
  });

  it("mint works", async () => {
    const tokenId = 1;

    expect(
      (await contract.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(0);

    const mintResult = await mintOne(contract, bob);

    // verify minting result
    expect(
      (await contract.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(1);
    expect((await contract.query.balanceOf(bob.address)).value.unwrap()).to.equal(1);
    expect((await contract.query.ownerOf({ u64: tokenId })).value.unwrap()).to.equal(
      bob.address
    );
    emit(mintResult, "Transfer", {
      from: null,
      to: bob.address,
      id: { u64: tokenId },
    });
  });

  it("mint 5 tokens works", async () => {
    const toMint = 5;
    expect(
      (await contract.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(0);


    await contract.withSigner(bob).tx.mintMany(toMint,
      {
        value: PRICE_PER_MINT.muln(toMint),
      }
    );

    // Verify minting result
    expect(
      (await contract.query.totalSupply()).value.unwrap().toNumber()
    ).to.equal(toMint);
    expect((await contract.query.ownerOf({ u64: toMint })).value.unwrap()).to.equal(
      bob.address
    );
  });

  it("token transfer works", async () => {
    // Bob mints
    const mintResult = await mintOne(contract, bob);

    // Bob transfers token to Deployer
    const transferGas = (
      await contract
        .withSigner(bob)
        .query.transfer(deployer.address, { u64: 1 }, [])
    ).gasRequired;
    let transferResult = await contract
      .withSigner(bob)
      .tx.transfer(deployer.address, { u64: 1 }, [], { gasLimit: transferGas });

    // Verify transfer
    expect((await contract.query.ownerOf({ u64: 1 })).value.unwrap()).to.equal(
      deployer.address
    );
    expect((await contract.query.balanceOf(bob.address)).value.ok).to.equal(0);
    emit(transferResult, "Transfer", {
      from: bob.address,
      to: deployer.address,
      id: { u64: 1 },
    });
  });

  it("token approval works", async () => {
    // Bob mints
    const mintResult = await mintOne(contract, bob);

    // Bob approves Deployer to be operator of the token
    const approveGas = (
      await contract
        .withSigner(bob)
        .query.approve(deployer.address, { u64: 1 }, true)
    ).gasRequired;
    let approveResult = await contract
      .withSigner(bob)
      .tx.approve(deployer.address, { u64: 1 }, true, { gasLimit: approveGas });

    // Verify that Bob is still the owner and allowance is set
    expect((await contract.query.ownerOf({ u64: 1 })).value.unwrap()).to.equal(
      bob.address
    );
    expect(
      (
        await contract.query.allowance(bob.address, deployer.address, {
          u64: 1,
        })
      ).value.ok
    ).to.equal(true);
    emit(approveResult, "Approval", {
      from: bob.address,
      to: deployer.address,
      id: { u64: 1 },
      approved: true,
    });
  });

  it("Minting token without funds should fail", async () => {

    // Bob tries to mint without funding
    const mintResult = await contract.withSigner(bob).query.mint(
      {
        value: 0
      },
    );
    expect(mintResult.value.unwrap().err.rmrk).to.be.equal(RmrkError.badMintValue);
  });
});

// helper function to mint a token
const mintOne = async (contract: Rmrk, signer: KeyringPair): Promise<SignAndSendSuccessResponse> => {
  // call mint function
  let mintResult = await contract
    .withSigner(signer)
    .tx.mint({
      value: PRICE_PER_MINT,
    }
    );
  return mintResult;
}

