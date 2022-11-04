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
const TOKEN_URI_5 = "ipfs://tokenUriPrefix/5.json";
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

        return {
            owner: contract_factory.deployer,
            contract: contract_factory.contract,
            bob: contract_factory.bob,
            uniquesCollectionId,
        }
    }
    
    it('create collection works', async () => {
        const { owner, contract, uniquesCollectionId } = await setup();

        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);
        await expect(contract.query["ownable::owner"]()).to.eventually.have.property('output').to.equal(owner.address);

        // console.log("api.query.uniques", api.query.uniques)
        await expect(contract.query["rmrkMintable::maxSupply"]()).to.eventually.have.property('output').to.equal(MAX_SUPPLY);
        await expect(contract.query["rmrkMintable::pricePerMint"]()).to.eventually.have.property('output').to.equal(PRICE_PER_MINT);
        
        // check that the collection is created in the Uniques pallet
        const unique_response = await api.query.uniques.class<u32>(uniquesCollectionId);

        expect(unique_response.toHuman().owner).to.be.eq(contract.address.toString());
    })
    
    it('mint 1 token works', async () => {
        const { uniquesCollectionId, contract, bob } = await setup();
        const tokenId = 1;

        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);

        // mint
        await expect(contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 1, {value: PRICE_PER_MINT})).to.be.eventually.fulfilled;

        // verify minting results
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(new BN("1"));
        await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(new BN("1"));
        await expect(contract.query["psp34::ownerOf"]({u64: tokenId})).to.eventually.have.property('output').to.equal(bob.address);

        // verify tokenUri call
        await expect(contract.query["rmrkMintable::tokenUri"](1)).to.eventually.have.property('output').to.equal(TOKEN_URI_1);
        
        // verify that the item is created in the Uniques pallet
        await expect((await api.query.uniques.asset(uniquesCollectionId, tokenId)).toHuman()?.owner).to.be.equal(bob.address);

    })

    it('mint 5 tokens works', async () => {
        const { contract, bob, uniquesCollectionId} = await setup();
        
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(0);
        await expect(contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 5, {value: PRICE_PER_MINT.muln(5)})).to.be.eventually.fulfilled;
        await expect(contract.query["psp34::totalSupply"]()).to.eventually.have.property('output').to.equal(new BN("5"));
        await expect(contract.query["rmrkMintable::tokenUri"](5)).to.eventually.have.property('output').to.equal(TOKEN_URI_5);
        await expect(contract.query["psp34::ownerOf"]({u64: 5})).to.eventually.have.property('output').to.equal(bob.address);

        // verify that the item is created in the Uniques pallet
        await expect((await api.query.uniques.asset(uniquesCollectionId, 5)).toHuman()?.owner).to.be.equal(bob.address);

    })

    it('token transfer works', async () => {
        const { owner, contract, bob, uniquesCollectionId } = await setup();
        
        // Bob mints 1 token
        await expect(contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 1, {value: PRICE_PER_MINT})).to.be.eventually.fulfilled;
        await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(new BN("1"));

        // verify that the owner in the Uniques pallet is Bob
        await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.owner).to.be.equal(bob.address);

        // Bob transfers token to Owner
        await expect(contract.connect(bob).tx["psp34::transfer"](owner.address, {u64:1}, [])).to.be.eventually.fulfilled;
        await expect(contract.query["psp34::balanceOf"](owner.address)).to.eventually.have.property('output').to.equal(new BN("1"));
        await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(0);
        await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(owner.address);

        // verify that the owner in the Uniques pallet is "owner"
        await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.owner).to.be.equal(owner.address);
    })

    it('token aprove works', async () => {
        const { owner, contract, bob, uniquesCollectionId } = await setup();
        
        // Bob mints 1 token
        await expect(contract.connect(bob).tx["rmrkMintable::mint"](bob.address, 1, {value: PRICE_PER_MINT})).to.be.eventually.fulfilled;
        await expect(contract.query["psp34::balanceOf"](bob.address)).to.eventually.have.property('output').to.equal(new BN("1"));

        // verify that the owner in the Uniques pallet is Bob
        await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.owner).to.be.equal(bob.address);
        await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(bob.address);
        
        // Bob approves owner to be operator of the token
        await expect(contract.connect(bob).tx["psp34::approve"](owner.address, {u64:1}, true)).to.be.eventually.fulfilled;
        await expect(contract.query["psp34::allowance"](bob.address, owner.address, {u64:1})).to.eventually.have.property('output').to.equal(true);
        
        // verify that Bob is still the owner
        await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.owner).to.be.equal(bob.address);
        await expect(contract.query["psp34::ownerOf"]({u64:1})).to.eventually.have.property('output').to.equal(bob.address);

        // verify allowance in Uniques pallet
        await expect((await api.query.uniques.asset(uniquesCollectionId, 1)).toHuman()?.approved).to.be.equal(owner.address);
    })
})