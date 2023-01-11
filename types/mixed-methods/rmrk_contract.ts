/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@supercolony/typechain-types';
import type { QueryReturnType } from '@supercolony/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@supercolony/typechain-types';
import { txSignAndSend } from '@supercolony/typechain-types';
import type * as ArgumentTypes from '../types-arguments/rmrk_contract';
import type * as ReturnTypes from '../types-returns/rmrk_contract';
import type BN from 'bn.js';
import {ReturnNumber} from '@supercolony/typechain-types';
import {getTypeDescription} from './../shared/utils';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";


export default class Methods {
	private __nativeContract : ContractPromise;
	private __keyringPair : KeyringPair;
	private __callerAddress : string;
	private __apiPromise: ApiPromise;

	constructor(
		apiPromise : ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
		this.__callerAddress = keyringPair.address;
	}

	/**
	* approve
	*
	* @param { ArgumentTypes.AccountId } operator,
	* @param { ArgumentTypes.Id | null } id,
	* @param { boolean } approved,
	* @returns { void }
	*/
	"approve" (
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		approved: boolean,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::approve", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [operator, id, approved], __options);
	}

	/**
	* totalSupply
	*
	* @returns { ReturnNumber }
	*/
	"totalSupply" (
		__options: GasLimit,
	): Promise< QueryReturnType< ReturnNumber > >{
		return queryJSON< ReturnNumber >( this.__nativeContract, this.__callerAddress, "psp34::totalSupply", [], __options, (result) => { return new ReturnNumber(result as (number | string)); });
	}

	/**
	* collectionId
	*
	* @returns { ReturnTypes.Id }
	*/
	"collectionId" (
		__options: GasLimit,
	): Promise< QueryReturnType< ReturnTypes.Id > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::collectionId", [], __options, (result) => { return handleReturnType(result, getTypeDescription(1, 'rmrk_contract')); });
	}

	/**
	* balanceOf
	*
	* @param { ArgumentTypes.AccountId } owner,
	* @returns { number }
	*/
	"balanceOf" (
		owner: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< number > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::balanceOf", [owner], __options);
	}

	/**
	* ownerOf
	*
	* @param { ArgumentTypes.Id } id,
	* @returns { ReturnTypes.AccountId | null }
	*/
	"ownerOf" (
		id: ArgumentTypes.Id,
		__options: GasLimit,
	): Promise< QueryReturnType< ReturnTypes.AccountId | null > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::ownerOf", [id], __options, (result) => { return handleReturnType(result, getTypeDescription(19, 'rmrk_contract')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< boolean > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::allowance", [owner, operator, id], __options);
	}

	/**
	* transfer
	*
	* @param { ArgumentTypes.AccountId } to,
	* @param { ArgumentTypes.Id } id,
	* @param { Array<(number | string | BN)> } data,
	* @returns { void }
	*/
	"transfer" (
		to: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		data: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::transfer", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [to, id, data], __options);
	}

	/**
	* owner
	*
	* @returns { ReturnTypes.AccountId }
	*/
	"owner" (
		__options: GasLimit,
	): Promise< QueryReturnType< ReturnTypes.AccountId > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'rmrk_contract')); });
	}

	/**
	* renounceOwnership
	*
	* @returns { void }
	*/
	"renounceOwnership" (
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::renounceOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [], __options);
	}

	/**
	* transferOwnership
	*
	* @param { ArgumentTypes.AccountId } newOwner,
	* @returns { void }
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::transferOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [newOwner], __options);
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
		__options: GasLimit,
	): Promise< QueryReturnType< Array<number> | null > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Metadata::getAttribute", [id, key], __options, (result) => { return handleReturnType(result, getTypeDescription(49, 'rmrk_contract')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Id, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options, (result) => { return handleReturnType(result, getTypeDescription(50, 'rmrk_contract')); });
	}

	/**
	* tokenByIndex
	*
	* @param { (string | number | BN) } index,
	* @returns { Result<ReturnTypes.Id, ReturnTypes.PSP34Error> }
	*/
	"tokenByIndex" (
		index: (string | number | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Id, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Enumerable::tokenByIndex", [index], __options, (result) => { return handleReturnType(result, getTypeDescription(50, 'rmrk_contract')); });
	}

	/**
	* tokenUri
	*
	* @param { (number | string | BN) } tokenId,
	* @returns { Result<string, ReturnTypes.PSP34Error> }
	*/
	"tokenUri" (
		tokenId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<string, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "utils::tokenUri", [tokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(51, 'rmrk_contract')); });
	}

	/**
	* withdraw
	*
	* @returns { void }
	*/
	"withdraw" (
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "utils::withdraw", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [], __options);
	}

	/**
	* setBaseUri
	*
	* @param { string } uri,
	* @returns { void }
	*/
	"setBaseUri" (
		uri: string,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "utils::setBaseUri", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [uri], __options);
	}

	/**
	* price
	*
	* @returns { ReturnNumber }
	*/
	"price" (
		__options: GasLimit,
	): Promise< QueryReturnType< ReturnNumber > >{
		return queryJSON< ReturnNumber >( this.__nativeContract, this.__callerAddress, "utils::price", [], __options, (result) => { return new ReturnNumber(result as (number | string)); });
	}

	/**
	* maxSupply
	*
	* @returns { number }
	*/
	"maxSupply" (
		__options: GasLimit,
	): Promise< QueryReturnType< number > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "utils::maxSupply", [], __options);
	}

	/**
	* mint
	*
	* @param { ArgumentTypes.AccountId } to,
	* @param { (number | string | BN) } mintAmount,
	* @returns { void }
	*/
	"mint" (
		to: ArgumentTypes.AccountId,
		mintAmount: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "minting::mint", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [to, mintAmount], __options);
	}

	/**
	* mintWithMetadata
	*
	* @param { string } metadata,
	* @param { ArgumentTypes.AccountId } to,
	* @returns { void }
	*/
	"mintWithMetadata" (
		metadata: string,
		to: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "minting::mintWithMetadata", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [metadata, to], __options);
	}

	/**
	* mintNext
	*
	* @returns { void }
	*/
	"mintNext" (
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "minting::mintNext", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [], __options);
	}

	/**
	* addChild
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { void }
	*/
	"addChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "nesting::addChild", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [parentTokenId, childNft], __options);
	}

	/**
	* removeChild
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { void }
	*/
	"removeChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "nesting::removeChild", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [parentTokenId, childNft], __options);
	}

	/**
	* acceptChild
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { void }
	*/
	"acceptChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "nesting::acceptChild", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [parentTokenId, childNft], __options);
	}

	/**
	* rejectChild
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { void }
	*/
	"rejectChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "nesting::rejectChild", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [parentTokenId, childNft], __options);
	}

	/**
	* transferChild
	*
	* @param { ArgumentTypes.Id } from,
	* @param { ArgumentTypes.Id } to,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @returns { void }
	*/
	"transferChild" (
		from: ArgumentTypes.Id,
		to: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "nesting::transferChild", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [from, to, childNft], __options);
	}

	/**
	* childrenBalance
	*
	* @param { ArgumentTypes.Id } parentTokenId,
	* @returns { Result<[number, number], ReturnTypes.PSP34Error> }
	*/
	"childrenBalance" (
		parentTokenId: ArgumentTypes.Id,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<[number, number], ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "nesting::childrenBalance", [parentTokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(53, 'rmrk_contract')); });
	}

	/**
	* getAcceptedTokenAssets
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<Array<number> | null, ReturnTypes.PSP34Error> }
	*/
	"getAcceptedTokenAssets" (
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<number> | null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "multiAsset::getAcceptedTokenAssets", [tokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(55, 'rmrk_contract')); });
	}

	/**
	* addAssetEntry
	*
	* @param { (number | string | BN) } id,
	* @param { (number | string | BN) } equippableGroupId,
	* @param { Array<(number | string | BN)> } assetUri,
	* @param { Array<(number | string | BN)> } partIds,
	* @returns { void }
	*/
	"addAssetEntry" (
		id: (number | string | BN),
		equippableGroupId: (number | string | BN),
		assetUri: Array<(number | string | BN)>,
		partIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "multiAsset::addAssetEntry", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [id, equippableGroupId, assetUri, partIds], __options);
	}

	/**
	* removeAsset
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } assetId,
	* @returns { void }
	*/
	"removeAsset" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "multiAsset::removeAsset", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [tokenId, assetId], __options);
	}

	/**
	* getAssetUri
	*
	* @param { (number | string | BN) } assetId,
	* @returns { Array<number> | null }
	*/
	"getAssetUri" (
		assetId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Array<number> | null > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "multiAsset::getAssetUri", [assetId], __options, (result) => { return handleReturnType(result, getTypeDescription(49, 'rmrk_contract')); });
	}

	/**
	* setPriority
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { Array<(number | string | BN)> } priorities,
	* @returns { void }
	*/
	"setPriority" (
		tokenId: ArgumentTypes.Id,
		priorities: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "multiAsset::setPriority", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [tokenId, priorities], __options);
	}

	/**
	* totalTokenAssets
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<[number, number], ReturnTypes.PSP34Error> }
	*/
	"totalTokenAssets" (
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<[number, number], ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "multiAsset::totalTokenAssets", [tokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(53, 'rmrk_contract')); });
	}

	/**
	* rejectAsset
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } assetId,
	* @returns { void }
	*/
	"rejectAsset" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "multiAsset::rejectAsset", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [tokenId, assetId], __options);
	}

	/**
	* addAssetToToken
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } assetId,
	* @param { (number | string | BN) | null } replacesAssetWithId,
	* @returns { void }
	*/
	"addAssetToToken" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		replacesAssetWithId: (number | string | BN) | null,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "multiAsset::addAssetToToken", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [tokenId, assetId, replacesAssetWithId], __options);
	}

	/**
	* acceptAsset
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } assetId,
	* @returns { void }
	*/
	"acceptAsset" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "multiAsset::acceptAsset", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [tokenId, assetId], __options);
	}

	/**
	* totalAssets
	*
	* @returns { number }
	*/
	"totalAssets" (
		__options: GasLimit,
	): Promise< QueryReturnType< number > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "multiAsset::totalAssets", [], __options);
	}

	/**
	* isEquippableByAll
	*
	* @param { (number | string | BN) } partId,
	* @returns { boolean }
	*/
	"isEquippableByAll" (
		partId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< boolean > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "base::isEquippableByAll", [partId], __options);
	}

	/**
	* resetEquippableAddresses
	*
	* @param { (number | string | BN) } partId,
	* @returns { void }
	*/
	"resetEquippableAddresses" (
		partId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "base::resetEquippableAddresses", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [partId], __options);
	}

	/**
	* setupBase
	*
	* @param { Array<(number | string | BN)> } baseMetadata,
	* @returns { void }
	*/
	"setupBase" (
		baseMetadata: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "base::setupBase", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [baseMetadata], __options);
	}

	/**
	* addEquippableAddresses
	*
	* @param { (number | string | BN) } partId,
	* @param { Array<ArgumentTypes.AccountId> } equippableAddress,
	* @returns { void }
	*/
	"addEquippableAddresses" (
		partId: (number | string | BN),
		equippableAddress: Array<ArgumentTypes.AccountId>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "base::addEquippableAddresses", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [partId, equippableAddress], __options);
	}

	/**
	* ensureEquippable
	*
	* @param { (number | string | BN) } partId,
	* @param { ArgumentTypes.AccountId } targetAddress,
	* @returns { Result<null, ReturnTypes.PSP34Error> }
	*/
	"ensureEquippable" (
		partId: (number | string | BN),
		targetAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "base::ensureEquippable", [partId, targetAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(45, 'rmrk_contract')); });
	}

	/**
	* getPartsCount
	*
	* @returns { number }
	*/
	"getPartsCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< number > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "base::getPartsCount", [], __options);
	}

	/**
	* addPartList
	*
	* @param { Array<ArgumentTypes.Part> } parts,
	* @returns { void }
	*/
	"addPartList" (
		parts: Array<ArgumentTypes.Part>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "base::addPartList", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [parts], __options);
	}

	/**
	* setEquippableByAll
	*
	* @param { (number | string | BN) } partId,
	* @returns { void }
	*/
	"setEquippableByAll" (
		partId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "base::setEquippableByAll", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [partId], __options);
	}

	/**
	* getPart
	*
	* @param { (number | string | BN) } partId,
	* @returns { ReturnTypes.Part | null }
	*/
	"getPart" (
		partId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< ReturnTypes.Part | null > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "base::getPart", [partId], __options, (result) => { return handleReturnType(result, getTypeDescription(59, 'rmrk_contract')); });
	}

	/**
	* getBaseMetadata
	*
	* @returns { string }
	*/
	"getBaseMetadata" (
		__options: GasLimit,
	): Promise< QueryReturnType< string > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "base::getBaseMetadata", [], __options);
	}

	/**
	* getAssetAndEquippableData
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } assetId,
	* @returns { Result<ReturnTypes.Asset, ReturnTypes.PSP34Error> }
	*/
	"getAssetAndEquippableData" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Asset, ReturnTypes.PSP34Error> > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "equippable::getAssetAndEquippableData", [tokenId, assetId], __options, (result) => { return handleReturnType(result, getTypeDescription(60, 'rmrk_contract')); });
	}

	/**
	* getEquipment
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } slotPartId,
	* @returns { ReturnTypes.Equipment | null }
	*/
	"getEquipment" (
		tokenId: ArgumentTypes.Id,
		slotPartId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< ReturnTypes.Equipment | null > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "equippable::getEquipment", [tokenId, slotPartId], __options, (result) => { return handleReturnType(result, getTypeDescription(61, 'rmrk_contract')); });
	}

	/**
	* unequip
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } slotPartId,
	* @returns { void }
	*/
	"unequip" (
		tokenId: ArgumentTypes.Id,
		slotPartId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "equippable::unequip", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [tokenId, slotPartId], __options);
	}

	/**
	* equip
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } assetId,
	* @param { (number | string | BN) } slotPartId,
	* @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	* @param { (number | string | BN) } childAssetId,
	* @returns { void }
	*/
	"equip" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		slotPartId: (number | string | BN),
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		childAssetId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "equippable::equip", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [tokenId, assetId, slotPartId, childNft, childAssetId], __options);
	}

	/**
	* setValidParentForEquippableGroup
	*
	* @param { (number | string | BN) } equippableGroupId,
	* @param { ArgumentTypes.AccountId } parentAddress,
	* @param { (number | string | BN) } partId,
	* @returns { void }
	*/
	"setValidParentForEquippableGroup" (
		equippableGroupId: (number | string | BN),
		parentAddress: ArgumentTypes.AccountId,
		partId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "equippable::setValidParentForEquippableGroup", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rmrk_contract");
		}, [equippableGroupId, parentAddress, partId], __options);
	}

}