import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import Catalog_Factory from "../types/constructors/catalog_example";
import Contract from "../types/contracts/catalog_example";
import { RmrkError } from "../types/types-returns/catalog_example";

import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import {
  PartType,
  Part,
} from "../types/types-arguments/catalog_example";

use(chaiAsPromised);

const CATALOG_METADATA = "ipfs://catalogMetadata/data.json";

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("RMRK Catalog tests", () => {
  let catalogFactory: Catalog_Factory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  let catalog: Contract;

  beforeEach(async function (): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true });
    deployer = keyring.addFromUri("//Alice");
    catalogFactory = new Catalog_Factory(api, deployer);
    catalog = new Contract(
      (
        await catalogFactory.new([CATALOG_METADATA])
      ).address,
      deployer,
      api
    );
  });

  it("Add/Remove Catalog parts works", async () => {
    // define 2 test Parts
    const PART_LIST: Part[] = [
      {
        partType: PartType.slot,
        z: 0,
        equippable: [],
        partUri: ["ipfs://backgrounds/1.svg"],
        isEquippableByAll: true,
      },
      {
        partType: PartType.fixed,
        z: 0,
        equippable: [],
        partUri: ["ipfs://backgrounds/2.svg"],
        isEquippableByAll: false,
      },
    ];

    // add parts to catalog
    await catalog
      .withSigner(deployer)
      .tx.addPartList(PART_LIST);
    expect((await catalog.query.getPartsCount())?.value.unwrap()).to.be.equal(2);

    // should fail since no addresses are added to equippable list for part 0
    const failEnsure = await catalog
      .withSigner(deployer)
      .query.ensureEquippable(0, catalog.address)
    expect(failEnsure.value.unwrap().err.rmrk).to.be.equal(RmrkError.addressNotEquippable);

    // add equippable addresses for part 0
    await catalog
      .withSigner(deployer)
      .tx.addEquippableAddresses(0, [catalog.address]);
    expect((await catalog.query.ensureEquippable(0, catalog.address))?.value.unwrap()).to.be
      .ok;

    // should fail since address is not added to equippable list for part 1
    const failEnsure2 = await catalog
      .withSigner(deployer)
      .query.ensureEquippable(1, catalog.address)
    expect(failEnsure2.value.unwrap().err.rmrk).to.be.equal(RmrkError.addressNotEquippable);

    // remove all equippable addresses for part 0
    expect((await catalog.query.getPart(0))?.value.unwrap().equippable.toString().length).to.be.greaterThan(1);
    await catalog
      .withSigner(deployer)
      .tx.resetEquippableAddresses(0);
    expect((await catalog.query.getPart(0))?.value.unwrap().equippable.toString()).to.be.equal("");

    // should fail in attempt to add equippable address to fixed part.
    const failAddEquip = await catalog
      .withSigner(deployer)
      .query.addEquippableAddresses(1, [catalog.address]);
    expect(failAddEquip.value.unwrap().err.rmrk).to.be.equal(RmrkError.partIsNotSlot);
  });
});

