import {getWallet, setupContract} from './helper'
import { expect } from "chai";
import { encodeAddress } from "@polkadot/keyring"
import * as BN from "bn.js";

describe('FACTORY', () => {
    async function setup() {
        const wallet = await getWallet()
        let pair = await setupContract('rmrk_contract', 'new')
        let pair_code_hash = (await pair.abi).source.hash
        let factory_contract =  await setupContract('factory_contract', 'new', wallet.address, pair_code_hash)
        return {
            wallet,
            deployer: factory_contract.deployer,
            alice: factory_contract.alice,
            contract: factory_contract.contract,
        }
    }

    // async function setup_psp22() {
    //     const wallet = await getWallet()
    //     let token_1 = await setupContract('psp22_token', 'new', new BN(10000000))
    //     let token_2 = await setupContract('psp22_token', 'new', new BN(10000000))
    //     let pair = await setupContract('pair_contract', 'new')
    //     let pair_code_hash = (await pair.abi).source.hash
    //     let factory_contract =  await setupContract('factory_contract', 'new', wallet.address, pair_code_hash)
    //     return {
    //         wallet,
    //         deployer: factory_contract.deployer,
    //         alice: factory_contract.alice,
    //         contract: factory_contract.contract,
    //         token_1: token_1.contract,
    //         token_2: token_2.contract
    //     }
    // }

    it('feeTo, feeToSetter, allPairsLength', async () => {
        const { contract, wallet } = await setup()
        console.log("testing in progress")

        // const zero_address = "0x0000000000000000000000000000000000000000000000000000000000000000"
        // await expect(contract.query["factory::feeTo"]()).to.eventually.have.property('output').to.equal(encodeAddress(zero_address))
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