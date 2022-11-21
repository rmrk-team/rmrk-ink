import {CodePromise} from "@polkadot/api-contract";
import type {KeyringPair} from "@polkadot/keyring/types";
import Files from "fs";
import type {ApiPromise} from "@polkadot/api";
import {_signAndSend, SignAndSendSuccessResponse} from "@supercolony/typechain-types";
import type {ConstructorOptions} from "@supercolony/typechain-types";
import type * as ArgumentTypes from '../types-arguments/rmrk_contract';
import type BN from 'bn.js';

export default class Constructors {
	readonly nativeAPI: ApiPromise;
	readonly signer: KeyringPair;

	constructor(
		nativeAPI: ApiPromise,
		signer: KeyringPair,
	) {
		this.nativeAPI = nativeAPI;
		this.signer = signer;
	}

    /**
    * new
    *
	* @param { Array<(number | string | BN)> } name,
	* @param { Array<(number | string | BN)> } symbol,
	* @param { Array<(number | string | BN)> } baseUri,
	* @param { (number | string | BN) } maxSupply,
	* @param { (string | number | BN) } pricePerMint,
	* @param { Array<(number | string | BN)> } collectionMetadata,
	* @param { ArgumentTypes.AccountId } royaltyReceiver,
	* @param { (number | string | BN) } royalty,
	*/
   	async "new" (
   		name: Array<(number | string | BN)>,
   		symbol: Array<(number | string | BN)>,
   		baseUri: Array<(number | string | BN)>,
   		maxSupply: (number | string | BN),
   		pricePerMint: (string | number | BN),
   		collectionMetadata: Array<(number | string | BN)>,
   		royaltyReceiver: ArgumentTypes.AccountId,
   		royalty: (number | string | BN),
   		__options ? : ConstructorOptions,
   	) {
   		const __contract = JSON.parse(Files.readFileSync("./artifacts/rmrk_contract.contract").toString());
		const code = new CodePromise(this.nativeAPI, __contract, __contract.source.wasm);
		const gasLimit = 100000 * 1000000 || __options?.gasLimit;

		const storageDepositLimit = __options?.storageDepositLimit;
        const tx = code.tx["new"]!({ gasLimit, storageDepositLimit, value: __options?.value }, name, symbol, baseUri, maxSupply, pricePerMint, collectionMetadata, royaltyReceiver, royalty);
		let response;

		try {
			response = await _signAndSend(this.nativeAPI.registry, tx, this.signer, (event: any) => event);
		}
		catch (error) {
			console.log(error);
		}

		return {
			result: response as SignAndSendSuccessResponse,
			// @ts-ignore
			address: (response as SignAndSendSuccessResponse)!.result!.contract.address.toString(),
		}
   	}
}