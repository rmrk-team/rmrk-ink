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
	 * totalSupply
	 *
	*/
	"totalSupply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::totalSupply", [], __options);
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
	 * collectionId
	 *
	*/
	"collectionId" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::collectionId", [], __options);
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
	 * maxSupply
	 *
	*/
	"maxSupply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "utils::maxSupply", [], __options);
	}

	/**
	 * price
	 *
	*/
	"price" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "utils::price", [], __options);
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
		return buildSubmittableExtrinsic( this.__nativeContract, "utils::tokenUri", [tokenId], __options);
	}

	/**
	 * withdraw
	 *
	*/
	"withdraw" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "utils::withdraw", [], __options);
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
		return buildSubmittableExtrinsic( this.__nativeContract, "utils::setBaseUri", [uri], __options);
	}

	/**
	 * mint
	 *
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { (number | string | BN) } mintAmount,
	*/
	"mint" (
		to: ArgumentTypes.AccountId,
		mintAmount: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "minting::mint", [to, mintAmount], __options);
	}

	/**
	 * mintWithMetadata
	 *
	 * @param { string } metadata,
	 * @param { ArgumentTypes.AccountId } to,
	*/
	"mintWithMetadata" (
		metadata: string,
		to: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "minting::mintWithMetadata", [metadata, to], __options);
	}

	/**
	 * mintNext
	 *
	*/
	"mintNext" (
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "minting::mintNext", [], __options);
	}

	/**
	 * removeChild
	 *
	 * @param { ArgumentTypes.Id } parentTokenId,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"removeChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::removeChild", [parentTokenId, childNft], __options);
	}

	/**
	 * acceptChild
	 *
	 * @param { ArgumentTypes.Id } parentTokenId,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"acceptChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::acceptChild", [parentTokenId, childNft], __options);
	}

	/**
	 * childrenBalance
	 *
	 * @param { ArgumentTypes.Id } parentTokenId,
	*/
	"childrenBalance" (
		parentTokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::childrenBalance", [parentTokenId], __options);
	}

	/**
	 * addChild
	 *
	 * @param { ArgumentTypes.Id } parentTokenId,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"addChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::addChild", [parentTokenId, childNft], __options);
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
	 * rejectChild
	 *
	 * @param { ArgumentTypes.Id } parentTokenId,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	*/
	"rejectChild" (
		parentTokenId: ArgumentTypes.Id,
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "nesting::rejectChild", [parentTokenId, childNft], __options);
	}

	/**
	 * getAcceptedTokenAssets
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"getAcceptedTokenAssets" (
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::getAcceptedTokenAssets", [tokenId], __options);
	}

	/**
	 * addAssetEntry
	 *
	 * @param { (number | string | BN) } id,
	 * @param { (number | string | BN) } equippableGroupId,
	 * @param { Array<(number | string | BN)> } assetUri,
	*/
	"addAssetEntry" (
		id: (number | string | BN),
		equippableGroupId: (number | string | BN),
		assetUri: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::addAssetEntry", [id, equippableGroupId, assetUri], __options);
	}

	/**
	 * addAssetToToken
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } assetId,
	 * @param { ArgumentTypes.Id | null } replacesAssetWithId,
	*/
	"addAssetToToken" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		replacesAssetWithId: ArgumentTypes.Id | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::addAssetToToken", [tokenId, assetId, replacesAssetWithId], __options);
	}

	/**
	 * totalAssets
	 *
	*/
	"totalAssets" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::totalAssets", [], __options);
	}

	/**
	 * rejectAsset
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } assetId,
	*/
	"rejectAsset" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::rejectAsset", [tokenId, assetId], __options);
	}

	/**
	 * acceptAsset
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } assetId,
	*/
	"acceptAsset" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::acceptAsset", [tokenId, assetId], __options);
	}

	/**
	 * getAssetUri
	 *
	 * @param { (number | string | BN) } assetId,
	*/
	"getAssetUri" (
		assetId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::getAssetUri", [assetId], __options);
	}

	/**
	 * removeAsset
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } assetId,
	*/
	"removeAsset" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::removeAsset", [tokenId, assetId], __options);
	}

	/**
	 * setPriority
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { Array<(number | string | BN)> } priorities,
	*/
	"setPriority" (
		tokenId: ArgumentTypes.Id,
		priorities: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::setPriority", [tokenId, priorities], __options);
	}

	/**
	 * totalTokenAssets
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"totalTokenAssets" (
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "multiAsset::totalTokenAssets", [tokenId], __options);
	}

	/**
	 * setupBase
	 *
	 * @param { Array<(number | string | BN)> } baseMetadata,
	*/
	"setupBase" (
		baseMetadata: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::setupBase", [baseMetadata], __options);
	}

	/**
	 * getPart
	 *
	 * @param { (number | string | BN) } partId,
	*/
	"getPart" (
		partId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::getPart", [partId], __options);
	}

	/**
	 * setEquippableByAll
	 *
	 * @param { (number | string | BN) } partId,
	*/
	"setEquippableByAll" (
		partId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::setEquippableByAll", [partId], __options);
	}

	/**
	 * resetEquippableAddresses
	 *
	 * @param { (number | string | BN) } partId,
	*/
	"resetEquippableAddresses" (
		partId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::resetEquippableAddresses", [partId], __options);
	}

	/**
	 * isEquippable
	 *
	 * @param { (number | string | BN) } partId,
	 * @param { ArgumentTypes.AccountId } targetAddress,
	*/
	"isEquippable" (
		partId: (number | string | BN),
		targetAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::isEquippable", [partId, targetAddress], __options);
	}

	/**
	 * addPartList
	 *
	 * @param { Array<ArgumentTypes.Part> } parts,
	*/
	"addPartList" (
		parts: Array<ArgumentTypes.Part>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::addPartList", [parts], __options);
	}

	/**
	 * getPartsCount
	 *
	*/
	"getPartsCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::getPartsCount", [], __options);
	}

	/**
	 * addEquippableAddresses
	 *
	 * @param { (number | string | BN) } partId,
	 * @param { Array<ArgumentTypes.AccountId> } equippableAddress,
	*/
	"addEquippableAddresses" (
		partId: (number | string | BN),
		equippableAddress: Array<ArgumentTypes.AccountId>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::addEquippableAddresses", [partId, equippableAddress], __options);
	}

	/**
	 * isEquippableByAll
	 *
	 * @param { (number | string | BN) } partId,
	*/
	"isEquippableByAll" (
		partId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::isEquippableByAll", [partId], __options);
	}

	/**
	 * getBaseMetadata
	 *
	*/
	"getBaseMetadata" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "base::getBaseMetadata", [], __options);
	}

	/**
	 * setValidParentForEquippableGroup
	 *
	 * @param { (number | string | BN) } equippableGroupId,
	 * @param { ArgumentTypes.AccountId } parentAddress,
	 * @param { (number | string | BN) } partId,
	*/
	"setValidParentForEquippableGroup" (
		equippableGroupId: (number | string | BN),
		parentAddress: ArgumentTypes.AccountId,
		partId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "equippable::setValidParentForEquippableGroup", [equippableGroupId, parentAddress, partId], __options);
	}

	/**
	 * extendEquippableAsset
	 *
	 * @param { (number | string | BN) } assetId,
	 * @param { (number | string | BN) } groupId,
	 * @param { Array<(number | string | BN)> } portIds,
	*/
	"extendEquippableAsset" (
		assetId: (number | string | BN),
		groupId: (number | string | BN),
		portIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "equippable::extendEquippableAsset", [assetId, groupId, portIds], __options);
	}

	/**
	 * getEquipment
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } slotPartId,
	*/
	"getEquipment" (
		tokenId: ArgumentTypes.Id,
		slotPartId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "equippable::getEquipment", [tokenId, slotPartId], __options);
	}

	/**
	 * unequip
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } slotPartId,
	*/
	"unequip" (
		tokenId: ArgumentTypes.Id,
		slotPartId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "equippable::unequip", [tokenId, slotPartId], __options);
	}

	/**
	 * getAssetAndEquippableData
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } assetId,
	*/
	"getAssetAndEquippableData" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "equippable::getAssetAndEquippableData", [tokenId, assetId], __options);
	}

	/**
	 * equip
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } assetId,
	 * @param { (number | string | BN) } slotPartId,
	 * @param { [ArgumentTypes.AccountId, ArgumentTypes.Id] } childNft,
	 * @param { (number | string | BN) } childAssetId,
	*/
	"equip" (
		tokenId: ArgumentTypes.Id,
		assetId: (number | string | BN),
		slotPartId: (number | string | BN),
		childNft: [ArgumentTypes.AccountId, ArgumentTypes.Id],
		childAssetId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "equippable::equip", [tokenId, assetId, slotPartId, childNft, childAssetId], __options);
	}

}