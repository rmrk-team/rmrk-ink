/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@supercolony/typechain-types';
import { buildSubmittableExtrinsic } from '@supercolony/typechain-types';
import type * as ArgumentTypes from '../types-arguments/rmrk_contract';
import type BN from 'bn.js';



export default class Methods {
	private __nativeContract : ContractPromise;

	constructor(
		nativeContract : ContractPromise,
	) {
		this.__nativeContract = nativeContract;
	}
	/**
	 * collectionId
	 *
	*/
	"collectionId" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::collectionId", [], __options);
	}

	/**
	 * ownerOf
	 *
	 * @param { ArgumentTypes.Id } id,
	*/
	"ownerOf" (
		id: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::ownerOf", [id], __options);
	}

	/**
	 * approve
	 *
	 * @param { ArgumentTypes.AccountId } operator,
	 * @param { ArgumentTypes.Id | null } id,
	 * @param { boolean } approved,
	*/
	"approve" (
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		approved: boolean,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::approve", [operator, id, approved], __options);
	}

	/**
	 * transfer
	 *
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { ArgumentTypes.Id } id,
	 * @param { Array<(number | string | BN)> } data,
	*/
	"transfer" (
		to: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		data: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::transfer", [to, id, data], __options);
	}

	/**
	 * balanceOf
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	*/
	"balanceOf" (
		owner: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::balanceOf", [owner], __options);
	}

	/**
	 * allowance
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	 * @param { ArgumentTypes.AccountId } operator,
	 * @param { ArgumentTypes.Id | null } id,
	*/
	"allowance" (
		owner: ArgumentTypes.AccountId,
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::allowance", [owner, operator, id], __options);
	}

	/**
	 * totalSupply
	 *
	*/
	"totalSupply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::totalSupply", [], __options);
	}

	/**
	 * renounceOwnership
	 *
	*/
	"renounceOwnership" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::renounceOwnership", [], __options);
	}

	/**
	 * owner
	 *
	*/
	"owner" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::owner", [], __options);
	}

	/**
	 * transferOwnership
	 *
	 * @param { ArgumentTypes.AccountId } newOwner,
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::transferOwnership", [newOwner], __options);
	}

	/**
	 * getAttribute
	 *
	 * @param { ArgumentTypes.Id } id,
	 * @param { Array<(number | string | BN)> } key,
	*/
	"getAttribute" (
		id: ArgumentTypes.Id,
		key: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Metadata::getAttribute", [id, key], __options);
	}

	/**
	 * ownersTokenByIndex
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	 * @param { (string | number | BN) } index,
	*/
	"ownersTokenByIndex" (
		owner: ArgumentTypes.AccountId,
		index: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options);
	}

	/**
	 * tokenByIndex
	 *
	 * @param { (string | number | BN) } index,
	*/
	"tokenByIndex" (
		index: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Enumerable::tokenByIndex", [index], __options);
	}

	/**
	 * price
	 *
	*/
	"price" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Custom::price", [], __options);
	}

	/**
	 * mintFor
	 *
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { (number | string | BN) } mintAmount,
	*/
	"mintFor" (
		to: ArgumentTypes.AccountId,
		mintAmount: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Custom::mintFor", [to, mintAmount], __options);
	}

	/**
	 * setBaseUri
	 *
	 * @param { string } uri,
	*/
	"setBaseUri" (
		uri: string,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Custom::setBaseUri", [uri], __options);
	}

	/**
	 * mintNext
	 *
	*/
	"mintNext" (
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Custom::mintNext", [], __options);
	}

	/**
	 * maxSupply
	 *
	*/
	"maxSupply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Custom::maxSupply", [], __options);
	}

	/**
	 * tokenUri
	 *
	 * @param { (number | string | BN) } tokenId,
	*/
	"tokenUri" (
		tokenId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Custom::tokenUri", [tokenId], __options);
	}

	/**
	 * withdraw
	 *
	*/
	"withdraw" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Custom::withdraw", [], __options);
	}

	/**
	 * addChild
	 *
	 * @param { ArgumentTypes.Id } parentTokenIs,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"addChild" (
		parentTokenIs: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::addChild", [parentTokenIs, childNft], __options);
	}

	/**
	 * transferChild
	 *
	 * @param { ArgumentTypes.Id } from,
	 * @param { ArgumentTypes.Id } to,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"transferChild" (
		from: ArgumentTypes.Id,
		to: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::transferChild", [from, to, childNft], __options);
	}

	/**
	 * acceptChild
	 *
	 * @param { ArgumentTypes.Id } parentTokenIs,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"acceptChild" (
		parentTokenIs: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::acceptChild", [parentTokenIs, childNft], __options);
	}

	/**
	 * removeChild
	 *
	 * @param { ArgumentTypes.Id } parentTokenIs,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"removeChild" (
		parentTokenIs: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::removeChild", [parentTokenIs, childNft], __options);
	}

	/**
	 * rejectChild
	 *
	 * @param { ArgumentTypes.Id } parentTokenIs,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"rejectChild" (
		parentTokenIs: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::rejectChild", [parentTokenIs, childNft], __options);
	}

}