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

export interface AddedChild {
	from: ReturnTypes.AccountId;
	to: ReturnTypes.Id;
	childCollection: ReturnTypes.AccountId;
	childTokenId: ReturnTypes.Id;
}

export interface ChildAccepted {
	by: ReturnTypes.AccountId;
	to: ReturnTypes.Id;
	childCollection: ReturnTypes.AccountId;
	childTokenId: ReturnTypes.Id;
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

