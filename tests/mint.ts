import {getWallet, setupContract} from './helper'
import { expect } from "chai";
import { network } from 'redspot'
import { encodeAddress } from "@polkadot/keyring"
import * as BN from "bn.js";
import { u32 } from '@polkadot/types-codec';
import { buildTx } from '@redspot/patract/buildTx'
const { api } = network;

const MAX_SUPPLY = 888;
const TOKEN_URI = "ipfs://tokenUriPrefix/";
const TOKEN_URI_1 = "ipfs://tokenUriPrefix/1.json";
const ONE = new BN(10).pow(new BN(18));
const PRICE_PER_MINT = ONE;

describe('Minting tests', () => {
    async function setup() {
        const royaltyAccount = await getWallet()
        const zero_address = "0x0000000000000000000000000000000000000000000000000000000000000000"
        const uniquesCollectionId = await api.query.uniques.nextCollectionId();
        // console.log("uniquesCollectionId:", uniquesCollectionId.toString());
        let contract_factory = await setupContract(
            'rmrk_contract', 'new', 'nameRMRK', 'RMK', MAX_SUPPLY, PRICE_PER_MINT, 'ipfs://collectionmetadata', TOKEN_URI, encodeAddress(zero_address), 0, 
            uniquesCollectionId
            );

        // workaround to send some funds to contract during instantiation
        const result = await contract_factory.contract.connect(contract_factory.deployer).tx["rmrkMintable::createCollection"]({value: ONE.muln(1)})
        // console.log("createCollection:", result);

        return {
            owner: contract_factory.deployer,
            contract: contract_factory.contract,
            bob: contract_factory.bob,
            uniquesCollectionId,
        }
    }
    
    it('create collection works', async () => {
        const { owner, contract, uniquesCollectionId } = await setup()

        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);
        await expect(contract.query["ownable::owner"]()).to.eventually.have.property('output').to.equal(owner.address);

        // console.log("api.query.uniques", api.query.uniques)
        await expect(contract.query["rmrkMintable::maxSupply"]()).to.eventually.have.property('output').to.equal(MAX_SUPPLY);
        await expect(contract.query["rmrkMintable::pricePerMint"]()).to.eventually.have.property('output').to.equal(PRICE_PER_MINT);
        
        // check that the collection is created in the Uniques pallet
        const unique_response = await api.query.uniques.class<u32>(uniquesCollectionId);
        // console.log("uniques class response", unique_response.toHuman());
        // console.log("owner:", owner.address.toString());
        // console.log("contract:", contract.address.toString());
        expect(unique_response.toHuman().owner).to.be.eq(contract.address.toString());
    })
    
    it('mint 1 token works', async () => {
        const { uniquesCollectionId, owner, contract, bob } = await setup()
        const tokenId = 1;

        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);

        // mint TODO: how to check result is ok?
        await contract.connect(bob).tx["rmrkMintable::mint"](bob.address, tokenId, {value: PRICE_PER_MINT})

        // verify minting results
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(new BN("1"));
        await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(new BN("1"));
        await expect(contract.query["psp34::ownerOf"]({u64: tokenId})).to.eventually.have.property('output').to.equal(bob.address);

        // verify tokenUri call
        await expect(contract.query["rmrkMintable::tokenUri"](1)).to.eventually.have.property('output').to.equal(TOKEN_URI_1);
        
        // verify that the item is created in the Uniques pallet
        await expect((await api.query.uniques.asset(uniquesCollectionId, tokenId)).toHuman().owner).to.be.equal(bob.address);

    })

    // it('mint 5 tokens works', async () => {
    //     const { owner, contract, bob } = await setup()
        
    //     await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);
    //     const result = await contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 5, {value: 5 * PRICE_PER_MINT})
    //     await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(5);
    //     await expect(contract.query["rmrkMintable::tokenUri"](1)).to.eventually.have.property('output').to.equal(TOKEN_URI_1);
    // })

    // it('token transfer works', async () => {
    //     const { owner, contract, bob } = await setup()
        
    //     await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);

    //     // let owner mint 1 token
    //     const result = await contract.connect(owner).tx["rmrkMintable::mint"](owner.address, 1, {value: PRICE_PER_MINT})
    //     await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(1);
    //     await expect(contract.query["psp34::balanceOf"](owner.address)).to.eventually.have.property('output').to.equal(1);
    //     await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(owner.address);
        
    //     // Owner transfers token to Bob
    //     await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(0);
    //     await expect(contract.tx["psp34::transfer"](bob.address, {u64:1}, [])).to.eventually.be.fulfilled
    //     await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(1);
    //     await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(bob.address);
    // })
})