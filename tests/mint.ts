import {getWallet, setupContract} from './helper'
import { expect } from "chai";
import { network } from 'redspot'
import { encodeAddress } from "@polkadot/keyring"
import * as BN from "bn.js";
import { u32 } from '@polkadot/types-codec';
import { buildTx } from '@redspot/patract/buildTx'
const { api } = network;

const MAX_SUPPLY = 888;
const PRICE_PER_MINT = 1000;
const TOKEN_URI = "ipfs://tokenUriPrefix/";
const TOKEN_URI_1 = "ipfs://tokenUriPrefix/1.json";
const ONE = new BN(10).pow(new BN(api.registry.chainDecimals[0]));


describe('Minting tests', () => {
    async function setup() {
        const royaltyAccount = await getWallet()
        const zero_address = "0x0000000000000000000000000000000000000000000000000000000000000000"
        let contract_factory = await setupContract(
            'rmrk_contract', 'new', 'nameRMRK', 'RMK', MAX_SUPPLY, PRICE_PER_MINT, 'ipfs://collectionmetadata', TOKEN_URI, encodeAddress(zero_address), 0)

        return {
            owner: contract_factory.deployer,
            contract: contract_factory.contract,
            bob: contract_factory.bob,
        }
    }

    it('create collection works', async () => {
        const { owner, contract, bob } = await setup()

        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);
        await expect(contract.query["ownable::owner"]()).to.eventually.have.property('output').to.equal(owner.address);

        // console.log("contract.query", contract.query)
        await expect(contract.query["rmrkMintable::maxSupply"]()).to.eventually.have.property('output').to.equal(MAX_SUPPLY);
        await expect(contract.query["rmrkMintable::pricePerMint"]()).to.eventually.have.property('output').to.equal(PRICE_PER_MINT);
        
        // check that the collection is created in the Uniques pallet
        const unique_response = await api.query.uniques.class<u32>(0);
        console.log("uniques class response", unique_response.toHuman());
        console.log("owner:", owner.address.toString());
        console.log("contract:", contract.address.toString());
        expect(unique_response.toHuman().owner).to.be.eq(contract.address);
        console.log("uniques class response", unique_response.toHuman().totalDeposit);
    })
    
    it('mint 1 token works', async () => {
        const { owner, contract, bob } = await setup()
        
        console.log("contract.query", contract.query)
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);
        const result = await contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 1, {value: PRICE_PER_MINT})
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(1);
        await expect(contract.query["rmrkMintable::tokenUri"](1)).to.eventually.have.property('output').to.equal(TOKEN_URI_1);
        await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(1);
        // await expect(contract.query["psp34::ownerOf"](1)).to.eventually.have.property('output').to.equal(bob.address);

        const output_uri = await contract.query["rmrkMintable::tokenUri"](1);
        console.log(output_uri.output?.toHuman());

    })

    it('mint 5 tokens works', async () => {
        const { owner, contract, bob } = await setup()
        
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);
        const result = await contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 5, {value: 5 * PRICE_PER_MINT})
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(5);
        await expect(contract.query["rmrkMintable::tokenUri"](1)).to.eventually.have.property('output').to.equal(TOKEN_URI_1);
    })

    it('token transfer works', async () => {
        const { owner, contract, bob } = await setup()
        
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);

        // let owner mint 1 token
        const result = await contract.connect(owner).tx["rmrkMintable::mint"](owner.address, 1, {value: PRICE_PER_MINT})
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(1);
        await expect(contract.query["psp34::balanceOf"](owner.address)).to.eventually.have.property('output').to.equal(1);
        await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(owner.address);
        
        // Owner transfers token to Bob
        await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(0);
        await expect(contract.tx["psp34::transfer"](bob.address, {u64:1}, [])).to.eventually.be.fulfilled
        await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(1);
        await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(bob.address);
    })
})