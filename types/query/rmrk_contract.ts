/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@supercolony/typechain-types';
import type { QueryReturnType } from '@supercolony/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@supercolony/typechain-types';
import type * as ArgumentTypes from '../types-arguments/rmrk_contract';
import type * as ReturnTypes from '../types-returns/rmrk_contract';
import type BN from 'bn.js';
import {ReturnNumber} from '@supercolony/typechain-types';
import {getTypeDescription} from './../shared/utils';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
	}

	/**
	* collectionId
	*
	* @returns { ReturnTypes.Id }
	*/
	"collectionId" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< ReturnTypes.Id > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::collectionId", [], __options , (result) => { return handleReturnType(result, getTypeDescription(1, 'rmrk_contract')); });
	}

	/**
	* totalSupply
	*
	* @returns { ReturnNumber }
	*/
	"totalSupply" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< ReturnNumber > >{
		return queryJSON< ReturnNumber >( this.__nativeContract, this.__callerAddress, "psp34::totalSupply", [], __options , (result) => { return new ReturnNumber(result as (number | string)); });
	}

	/**
	* transfer
	*
	* @param { ArgumentTypes.AccountId } to,
	* @param { ArgumentTypes.Id } id,
	* @param { Array<(number | string | BN)> } data,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"transfer" (
		to: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		data: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34::transfer", [to, id, data], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* allowance
	*
	* @param { ArgumentTypes.AccountId } owner,
	* @param { ArgumentTypes.AccountId } operator,
	* @param { ArgumentTypes.Id | null } id,
	* @returns { boolean }
	*/
	"allowance" (
		owner: ArgumentTypes.AccountId,
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		__options ? : GasLimit,
	): Promise< QueryReturnType< boolean > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::allowance", [owner, operator, id], __options );
	}

	/**
	* approve
	*
	* @param { ArgumentTypes.AccountId } operator,
	* @param { ArgumentTypes.Id | null } id,
	* @param { boolean } approved,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"approve" (
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		approved: boolean,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34::approve", [operator, id, approved], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* ownerOf
	*
	* @param { ArgumentTypes.Id } id,
	* @returns { ReturnTypes.AccountId | null }
	*/
	"ownerOf" (
		id: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< ReturnTypes.AccountId | null > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::ownerOf", [id], __options , (result) => { return handleReturnType(result, getTypeDescription(19, 'rmrk_contract')); });
	}

	/**
	* balanceOf
	*
	* @param { ArgumentTypes.AccountId } owner,
	* @returns { number }
	*/
	"balanceOf" (
		owner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< number > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::balanceOf", [owner], __options );
	}

	/**
	* renounceOwnership
	*
	* @returns { Result<null, ReturnTypes.OwnableError> }
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.OwnableError> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "ownable::renounceOwnership", [], __options , (result) => { return handleReturnType(result, getTypeDescription(35, 'rmrk_contract')); });
	}

	/**
	* transferOwnership
	*
	* @param { ArgumentTypes.AccountId } newOwner,
	* @returns { Result<null, ReturnTypes.OwnableError> }
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.OwnableError> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "ownable::transferOwnership", [newOwner], __options , (result) => { return handleReturnType(result, getTypeDescription(35, 'rmrk_contract')); });
	}

	/**
	* owner
	*
	* @returns { ReturnTypes.AccountId }
	*/
	"owner" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< ReturnTypes.AccountId > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'rmrk_contract')); });
	}

	/**
	* getAttribute
	*
	* @param { ArgumentTypes.Id } id,
	* @param { Array<(number | string | BN)> } key,
	* @returns { Array<number> | null }
	*/
	"getAttribute" (
		id: ArgumentTypes.Id,
		key: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Array<number> | null > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Metadata::getAttribute", [id, key], __options , (result) => { return handleReturnType(result, getTypeDescription(37, 'rmrk_contract')); });
	}

	/**
	* tokenByIndex
	*
	* @param { (string | number | BN) } index,
	* @returns { Result<ReturnTypes.Id, ReturnTypes.PSP34Error> }
	*/
	"tokenByIndex" (
		index: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Id, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Enumerable::tokenByIndex", [index], __options , (result) => { return handleReturnType(result, getTypeDescription(38, 'rmrk_contract')); });
	}

	/**
	* ownersTokenByIndex
	*
	* @param { ArgumentTypes.AccountId } owner,
	* @param { (string | number | BN) } index,
	* @returns { Result<ReturnTypes.Id, ReturnTypes.PSP34Error> }
	*/
	"ownersTokenByIndex" (
		owner: ArgumentTypes.AccountId,
		index: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Id, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options , (result) => { return handleReturnType(result, getTypeDescription(38, 'rmrk_contract')); });
	}

	/**
	* withdraw
	*
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"withdraw" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Custom::withdraw", [], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* setBaseUri
	*
	* @param { string } uri,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"setBaseUri" (
		uri: string,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Custom::setBaseUri", [uri], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* mintFor
	*
	* @param { ArgumentTypes.AccountId } to,
	* @param { (number | string | BN) } mintAmount,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"mintFor" (
		to: ArgumentTypes.AccountId,
		mintAmount: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Custom::mintFor", [to, mintAmount], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* price
	*
	* @returns { ReturnNumber }
	*/
	"price" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< ReturnNumber > >{
		return queryJSON< ReturnNumber >( this.__nativeContract, this.__callerAddress, "psp34Custom::price", [], __options , (result) => { return new ReturnNumber(result as (number | string)); });
	}

	/**
	* maxSupply
	*
	* @returns { number }
	*/
	"maxSupply" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< number > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Custom::maxSupply", [], __options );
	}

	/**
	* tokenUri
	*
	* @param { (number | string | BN) } tokenId,
	* @returns { Result<string, ReturnTypes.PSP34Error> }
	*/
	"tokenUri" (
		tokenId: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<string, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Custom::tokenUri", [tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(40, 'rmrk_contract')); });
	}

	/**
	* mintNext
	*
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"mintNext" (
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Custom::mintNext", [], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* transferChild
	*
	* @param { ArgumentTypes.Id } from,
	* @param { ArgumentTypes.Id } to,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"transferChild" (
		from: ArgumentTypes.Id,
		to: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "nesting::transferChild", [from, to, childNft], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* removeChild
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"removeChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "nesting::removeChild", [parentTokenId, childNft], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* childrenBalance
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @returns { Result<[number, number], ReturnTypes.PSP34Error> }
	*/
	"childrenBalance" (
		parentTokenId: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<[number, number], ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "nesting::childrenBalance", [parentTokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(41, 'rmrk_contract')); });
	}

	/**
	* acceptChild
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"acceptChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "nesting::acceptChild", [parentTokenId, childNft], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* addChild
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"addChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "nesting::addChild", [parentTokenId, childNft], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

	/**
	* rejectChild
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"rejectChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "nesting::rejectChild", [parentTokenId, childNft], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'rmrk_contract')); });
	}

}