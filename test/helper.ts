import {artifacts, network, patract} from 'redspot'
import * as BN from 'bn.js';
import {createSigner} from "redspot/provider";
import { Keyring } from '@polkadot/keyring'
const {api} = network
import { buildTx } from '@redspot/patract/buildTx'

const {getContractFactory, getRandomSigner} = patract
const {getSigners} = network

const ONE = new BN(10).pow(new BN(api.registry.chainDecimals[0]))

export const setupContract = async (name, constructor, ...args) => {
    await api.isReady
    const signers = await getSigners()
    const signer = await getRandomSigner(signers[0], ONE.muln(100000))
    // @ts-ignore
    const alice = createSigner(signer, new Keyring({ type: 'sr25519'}).addFromUri('//Alice'));
    await buildTx(api.registry, api.tx.balances.transfer(alice.address, ONE.muln(1000)), signer.address)
    const bob = await getRandomSigner(signers[1], ONE.muln(100000))
    const contractFactory = await getContractFactory(name, alice)
    const contract = await contractFactory.deploy(constructor, ...args)
    const abi = artifacts.readArtifact(name)

    return {
        deployer: alice,
        bob,
        contract,
        abi,
        query: contract.query,
        tx: contract.tx
    }
}

export const attachContract = async (name, address) => {
    await api.isReady
    const signers = await getSigners()
    const deployer = await getRandomSigner(signers[0], ONE.muln(100000))

    const contractFactory = await getContractFactory(name, deployer)
    const contract = await contractFactory.attach(address)
    const abi = artifacts.readArtifact(name)

    return {
        deployer,
        contract,
        abi,
        query: contract.query,
        tx: contract.tx
    }
}

export const getWallet = async () => {
    await api.isReady
    const signers = await getSigners()
    return await getRandomSigner(signers[1], ONE.muln(1000))
}