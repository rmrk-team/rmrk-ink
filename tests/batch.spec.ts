import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import Rmrk_factory from "../types/constructors/rmrk_example_equippable";
import Rmrk from "../types/contracts/rmrk_example_equippable";
import type * as ArgumentTypes from '../types/types-arguments/rmrk_example_equippable';
import Catalog_Factory from "../types/constructors/catalog_example";
import Contract from "../types/contracts/catalog_example";
import {
    PartType,
    Part,
} from "../types/types-arguments/catalog_example";
import { SignAndSendSuccessResponse } from "@727-ventures/typechain-types";
import { RmrkError } from "types-returns/rmrk_example_equippable";

use(chaiAsPromised);

const MAX_SUPPLY = 10000;
const BASE_URI = "ipfs://tokenUriPrefix/";
const COLLECTION_METADATA = "ipfs://collectionMetadata/data.json";
const CATALOG_METADATA = "ipfs://catalogMetadata/data.json";

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("RMRK Batch calls test", () => {
    let parentFactory: Rmrk_factory;
    let childFactory: Rmrk_factory;
    let catalogFactory: Catalog_Factory;
    let api: ApiPromise;
    let deployer: KeyringPair;
    let parent: Rmrk;
    let child: Rmrk;
    let catalog: Contract;


    beforeEach(async function (): Promise<void> {
        api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true });
        deployer = keyring.addFromUri("//Alice");

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

        catalogFactory = new Catalog_Factory(api, deployer);
        catalog = new Contract(
            (
                await catalogFactory.new([CATALOG_METADATA])
            ).address,
            deployer,
            api
        );

    });

    it("deployer mints and transfers many works", async () => {
        const PARENT_TOKENS = 50;
        const CHILD_TOKENS = 50;
        const ASSET_ID1 = 1;

        // add part for catalog
        const PART_LIST: Part[] = [
            {
                partType: PartType.slot,
                z: 0,
                equippable: [],
                partUri: ["ipfs://backgrounds/1.svg"],
                isEquippableByAll: true,
            },
        ];

        // add parts to catalog
        await catalog
            .withSigner(deployer)
            .tx.addPartList(PART_LIST);
        expect((await catalog.query.getPartsCount())?.value.unwrap()).to.be.equal(1);

        // deployer mints many parent tokens
        await mintMany(parent, deployer, PARENT_TOKENS);
        expect(
            (await parent.query.totalSupply()).value.unwrap().toNumber()
        ).to.equal(PARENT_TOKENS);
        expect((await parent.query.ownerOf({ u64: 1 })).value.unwrap()).to.equal(
            deployer.address
        );

        // deployer mints many child tokens
        await mintMany(child, deployer, CHILD_TOKENS);
        expect(
            (await child.query.totalSupply()).value.unwrap().toNumber()
        ).to.equal(CHILD_TOKENS);
        expect((await child.query.ownerOf({ u64: 1 })).value.unwrap()).to.equal(
            deployer.address
        );

        // create and add Asset to many tokens
        await child
            .withSigner(deployer)
            .tx.addAssetEntry(catalog.address, ASSET_ID1, 0, ["ipfs://gems/typeA/full.svg"], []);
        var tokenList = new Array();
        for (let i = 1; i <= CHILD_TOKENS; i++) {
            tokenList.push({ u64: i });
        }

        expect((await child
            .withSigner(deployer)
            .tx.addAssetToManyTokens(tokenList, ASSET_ID1))?.result).to.be.ok;

        for (let i = 1; i <= CHILD_TOKENS; i++) {
            expect(
                (await child.query.totalTokenAssets({ u64: i }))?.value.unwrap().ok.toString()
            ).to.be.equal("1,0");
        }

        // deployer approves parent's Contract on child
        await approve(child, parent, deployer);

        // deployer adds each child nft to its parent
        var parentChildPair = new Array();
        for (let i = 1; i <= PARENT_TOKENS; i++) {
            parentChildPair.push([{ u64: i }, { u64: i }]);
        }
        await addManyChildren(parent, deployer, child, parentChildPair);
        // check that all parent tokens have 1 child
        for (let i = 1; i <= PARENT_TOKENS; i++) {
            expect(
                (await parent.query.childrenBalance({ u64: i }))?.value.unwrap().ok.toString()
            ).to.be.equal("1,0");
        }

        // deployer transfers all tokens to users
        const manyUsers = createBatchUsers(PARENT_TOKENS);
        let tokenDestinationPair = new Array<[ArgumentTypes.Id, ArgumentTypes.AccountId]>();
        for (let i = 1; i <= PARENT_TOKENS; i++) {
            tokenDestinationPair.push([{ u64: i }, manyUsers[i - 1].address]);
        }

        await parent
            .withSigner(deployer)
            .tx.transferMany(
                tokenDestinationPair,
            );
        for (let i = 1; i <= PARENT_TOKENS; i++) {
            expect((await parent.query.ownerOf({ u64: i })).value.unwrap()).to.equal(
                manyUsers[i - 1].address
            );
        }
    });

    it("Set of batch calls works", async () => {
        const PARENT_TOKENS = 100;
        const CHILD_TOKENS = 100;
        const ASSET_ID1 = 1;
        const sets = 10;

        // add part for catalog
        const PART_LIST: Part[] = [
            {
                partType: PartType.slot,
                z: 0,
                equippable: [],
                partUri: ["ipfs://backgrounds/1.svg"],
                isEquippableByAll: true,
            },
        ];

        // add parts to catalog
        await catalog
            .withSigner(deployer)
            .tx.addPartList(PART_LIST);
        expect((await catalog.query.getPartsCount())?.value.unwrap()).to.be.equal(1);

        // deployer mints many parent tokens
        for (let i = 0; i < sets; i++) {
            await mintMany(parent, deployer, PARENT_TOKENS);
        }
        expect(
            (await parent.query.totalSupply()).value.unwrap().toNumber()
        ).to.equal(PARENT_TOKENS * sets);
        expect((await parent.query.ownerOf({ u64: 1 })).value.unwrap()).to.equal(
            deployer.address
        );

        // deployer mints many child tokens
        for (let i = 0; i < sets; i++) {
            await mintMany(child, deployer, CHILD_TOKENS);
        }
        expect(
            (await child.query.totalSupply()).value.unwrap().toNumber()
        ).to.equal(CHILD_TOKENS * sets);
        expect((await child.query.ownerOf({ u64: 1 })).value.unwrap()).to.equal(
            deployer.address
        );

        // create and add Asset to many tokens
        await child
            .withSigner(deployer)
            .tx.addAssetEntry(catalog.address, ASSET_ID1, 0, ["ipfs://gems/typeA/full.svg"], []);

        for (let i = 0; i < sets; i++) {
            var tokenList = new Array();
            // console.log("addAssetToManyTokens, set=", i)
            for (let j = CHILD_TOKENS * i + 1; j <= CHILD_TOKENS * (i + 1); j++) {
                tokenList.push({ u64: j });
            }
            expect((await child
                .withSigner(deployer)
                .tx.addAssetToManyTokens(tokenList, ASSET_ID1))?.result).to.be.ok;
        }
        for (let i = 1; i <= CHILD_TOKENS * sets; i++) {
            expect(
                (await child.query.totalTokenAssets({ u64: i }))?.value.unwrap().ok.toString()
            ).to.be.equal("1,0");
        }

        // deployer approves parent's Contract on child
        await approve(child, parent, deployer);

        // deployer adds each child nft to its parent
        for (let i = 0; i < sets; i++) {
            var parentChildPair = new Array();
            // console.log("addManyChildren, set=", i)
            for (let j = PARENT_TOKENS * i + 1; j <= PARENT_TOKENS * (i + 1); j++) {
                parentChildPair.push([{ u64: j }, { u64: j }]);
            }
            await addManyChildren(parent, deployer, child, parentChildPair);
        }
        // check that all parent tokens have 1 child
        for (let i = 1; i <= PARENT_TOKENS * sets; i++) {
            expect(
                (await parent.query.childrenBalance({ u64: i }))?.value.unwrap().ok.toString()
            ).to.be.equal("1,0");
        }

        // deployer transfers all tokens to users
        const manyUsers = createBatchUsers(PARENT_TOKENS * sets);

        for (let i = 0; i < sets; i++) {
            let tokenDestinationPair = new Array<[ArgumentTypes.Id, ArgumentTypes.AccountId]>();
            // console.log("transferMany, set=", i)
            for (let j = PARENT_TOKENS * i + 1; j <= PARENT_TOKENS * (i + 1); j++) {
                tokenDestinationPair.push([{ u64: j }, manyUsers[j - 1].address]);
            }
            await parent
                .withSigner(deployer)
                .tx.transferMany(
                    tokenDestinationPair,
                );
        }
        for (let i = 1; i <= PARENT_TOKENS*sets; i++) {
            expect((await parent.query.ownerOf({ u64: i })).value.unwrap()).to.equal(
                manyUsers[i - 1].address
            );
        }
    });
});

// helper function to mint many tokens
const mintMany = async (contract: Rmrk, signer: KeyringPair, mintAmount: number): Promise<void> => {
    let mintResult = await contract
        .withSigner(signer)
        .tx.mintMany(
            signer.address,
            mintAmount,
        );
}

// helper function to add many children to many parents
const addManyChildren = async (contract: Rmrk, signer: KeyringPair, child: Rmrk, parentChildPair: any): Promise<void> => {
    let addResult = await contract
        .withSigner(signer)
        .tx.addManyChildren(
            child.address,
            parentChildPair,
        );
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

// helper function to create a batch of users
const createBatchUsers = (amount: number): KeyringPair[] => {
    const users = new Array();
    for (let i = 0; i < amount; i++) {
        users.push(keyring.addFromUri(`//user${i}`));
    }
    return users;
}
