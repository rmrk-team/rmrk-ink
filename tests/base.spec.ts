import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import { encodeAddress } from "@polkadot/keyring";
import BN from "bn.js";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable_lazy";
import Rmrk from "../types/contracts/rmrk_example_equippable_lazy";
import { RmrkError } from "../types/types-returns/rmrk_example_equippable_lazy";

import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { ReturnNumber } from "@supercolony/typechain-types";
import {
  PartType,
  Part,
} from "../types/types-arguments/rmrk_example_equippable_lazy";

use(chaiAsPromised);

const MAX_SUPPLY = 888;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const BASE_METADATA = "ipfs://baseMetadata";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("RMRK Base tests", () => {
  let kanariaFactory: Rmrk_factory;
  let gemFactory: Rmrk_factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let bob: KeyringPair;
  let dave: KeyringPair;
  let kanaria: Rmrk;
  let gem: Rmrk;

  const ZERO_ADDRESS = encodeAddress(
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  );
  let gasRequired: bigint;

  async function setup(): Promise<void> {
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
  }

  it("Setup Base", async () => {
    await setup();

    // set Base metadata
    const setupBaseGas = (await gem.query.setupBase([BASE_METADATA]))
      .gasRequired;
    await gem
      .withSigner(deployer)
      .tx.setupBase([BASE_METADATA], { gasLimit: setupBaseGas * 2n });

    // define 2 test Parts
    const PART_LIST: Part[] = [
      {
        partType: PartType.slot,
        z: 0,
        equippable: [],
        metadataUri: ["ipfs://backgrounds/1.svg"],
        isEquippableByAll: true,
      },
      {
        partType: PartType.fixed,
        z: 0,
        equippable: [],
        metadataUri: ["ipfs://backgrounds/2.svg"],
        isEquippableByAll: false,
      },
    ];

    // add parts to base
    const addPartListGas = (
      await gem.withSigner(deployer).query.addPartList(PART_LIST)
    ).gasRequired;
    await gem
      .withSigner(deployer)
      .tx.addPartList(PART_LIST, { gasLimit: addPartListGas * 2n });
    expect((await gem.query.getPartsCount())?.value).to.be.equal(2);

    // add/remove equippable addresses
    const addEquipGas = (
      await gem
        .withSigner(deployer)
        .query.addEquippableAddresses(0, [kanaria.address])
    ).gasRequired;
    await gem
      .withSigner(deployer)
      .tx.addEquippableAddresses(0, [kanaria.address], {
        gasLimit: addEquipGas * 2n,
      });
    expect((await gem.query.ensureEquippable(0, kanaria.address))?.value).to.be
      .ok;
    expect((await gem.query.ensureEquippable(1, kanaria.address))?.value).to.be
      .ok;
    const removePartListGas = (
      await gem.withSigner(deployer).query.resetEquippableAddresses(0)
    ).gasRequired;
    await gem
      .withSigner(deployer)
      .tx.resetEquippableAddresses(0, { gasLimit: removePartListGas * 2n });

    // should fail in attempt to add equippable address to fixed part.
    const failAddEquip = await gem
      .withSigner(deployer)
      .query.addEquippableAddresses(1, [kanaria.address]);
    expect(failAddEquip.value.err.rmrk).to.be.equal(RmrkError.partIsNotSlot);
  });
});

// Helper function to parse Events
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function emit(result: { events?: any }, name: string, args: any): void {
  const event = result.events.find(
    (event: { name: string }) => event.name === name
  );
  for (const key of Object.keys(event.args)) {
    if (event.args[key] instanceof ReturnNumber) {
      event.args[key] = event.args[key].toNumber();
    }
  }
  expect(event).eql({ name, args });
}

// Helper function to convert error code to string
function hex2a(psp34CustomError: any): string {
  var hex = psp34CustomError.toString(); //force conversion
  var str = "";
  for (var i = 0; i < hex.length; i += 2)
    str += String.fromCharCode(parseInt(hex.substr(i, 2), 16));
  return str.substring(1);
}
