import {getWallet, setupContract} from './helper'
import { expect } from "chai";
import { encodeAddress } from "@polkadot/keyring"
import * as BN from "bn.js";

const MAX_SUPPLY = 888;

describe('Minting tests', () => {
    async function setup() {
        const royaltyAccount = await getWallet()
        const zero_address = "0x0000000000000000000000000000000000000000000000000000000000000000"
        let contract_factory = await setupContract(
            'rmrk_contract', 'new', 'nameRMRK', 'RMK', MAX_SUPPLY, 1000, 'ipfs://collectionmetadata', 'ipfs://tokenUriPrefix', encodeAddress(zero_address), 0)

        return {
            deployer: contract_factory.deployer,
            contract: contract_factory.contract,
            bob: contract_factory.bob
        }
    }

    it('create collection works', async () => {
        const { deployer, contract, bob } = await setup()
        console.log("testing in progress")

        await expect(contract.query["minting::maxSupply"]()).to.eventually.have.property('output').to.equal(MAX_SUPPLY)
        // await expect(contract.query["factory::feeToSetter"]()).to.eventually.have.property('output').to.equal(wallet.address)
        // await expect(contract.query["factory::allPairLength"]()).to.eventually.have.property('output').to.equal(0)
    })

    // it('create pair', async () => {
    //     const { contract, token_1, token_2 } = await setup_psp22()

    //     await expect(contract.query["factory::allPairLength"]()).to.eventually.have.property('output').to.equal(0)
    //     await expect(contract.tx["factory::createPair"](token_1.address, token_2.address)).to.eventually.be.fulfilled
    //     await expect(contract.query["factory::allPairLength"]()).to.eventually.have.property('output').to.equal(1)
    // })

    // it('set fee', async () => {
    //     const { contract, token_1, wallet } = await setup_psp22()

    //     const zero_address = "0x0000000000000000000000000000000000000000000000000000000000000000"
    //     await expect(contract.query["factory::feeTo"]()).to.eventually.have.property('output').to.equal(encodeAddress(zero_address))
    //     await expect(contract.tx["factory::setFeeTo"](token_1.address)).to.eventually.be.rejected
    //     await contract.connect(wallet).tx["factory::setFeeTo"](token_1.address)
    //     await expect(contract.query["factory::feeTo"]()).to.eventually.have.property('output').to.equal(token_1.address)
    // })

    // it('set fee setter', async () => {
    //     const { contract, token_1, wallet } = await setup_psp22()

    //     await expect(contract.query["factory::feeToSetter"]()).to.eventually.have.property('output').to.equal(wallet.address)
    //     await expect(contract.tx["factory::setFeeToSetter"](token_1.address)).to.eventually.be.rejected
    //     await contract.connect(wallet).tx["factory::setFeeToSetter"](token_1.address)
    //     await expect(contract.query["factory::feeToSetter"]()).to.eventually.have.property('output').to.equal(token_1.address)
    // })
})