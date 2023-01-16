import type {ReturnNumber} from "@supercolony/typechain-types";
import type * as ReturnTypes from '../types-returns/rmrk_contract';

export interface Transfer {
	from: ReturnTypes.AccountId | null;
	to: ReturnTypes.AccountId | null;
	id: ReturnTypes.Id;
}

export interface Approval {
	from: ReturnTypes.AccountId;
	to: ReturnTypes.AccountId;
	id: ReturnTypes.Id | null;
	approved: boolean;
}

export interface ChildAdded {
	to: ReturnTypes.Id;
	collection: ReturnTypes.AccountId;
	child: ReturnTypes.Id;
}

export interface ChildAccepted {
	parent: ReturnTypes.Id;
	collection: ReturnTypes.AccountId;
	child: ReturnTypes.Id;
}

export interface ChildRemoved {
	parent: ReturnTypes.Id;
	childCollection: ReturnTypes.AccountId;
	childTokenId: ReturnTypes.Id;
}

export interface ChildRejected {
	parent: ReturnTypes.Id;
	childCollection: ReturnTypes.AccountId;
	childTokenId: ReturnTypes.Id;
}

export interface AssetSet {
	asset: number;
}

export interface AssetAddedToToken {
	token: ReturnTypes.Id;
	asset: number;
	replaces: number | null;
}

export interface AssetAccepted {
	token: ReturnTypes.Id;
	asset: number;
}

export interface AssetRejected {
	token: ReturnTypes.Id;
	asset: number;
}

export interface AssetRemoved {
	token: ReturnTypes.Id;
	asset: number;
}

export interface AssetPrioritySet {
	token: ReturnTypes.Id;
	priorities: Array<number>;
}

export interface AssetEquipped {
	token: ReturnTypes.Id;
	asset: number;
	child: ReturnTypes.Id;
	childAsset: number;
}

export interface AssetUnEquipped {
	token: ReturnTypes.Id;
	asset: number;
	slot: number;
}

export interface ParentEquippableGroupSet {
	group: number;
	slot: number;
	parent: ReturnTypes.AccountId;
}

