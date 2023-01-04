import type BN from 'bn.js';

export interface Id {
	u8 ? : (number | string | BN),
	u16 ? : (number | string | BN),
	u32 ? : (number | string | BN),
	u64 ? : (number | string | BN),
	u128 ? : (string | number | BN),
	bytes ? : Array<(number | string | BN)>
}

export class IdBuilder {
	static U8(value: (number | string | BN)): Id {
		return {
			u8: value,
		};
	}
	static U16(value: (number | string | BN)): Id {
		return {
			u16: value,
		};
	}
	static U32(value: (number | string | BN)): Id {
		return {
			u32: value,
		};
	}
	static U64(value: (number | string | BN)): Id {
		return {
			u64: value,
		};
	}
	static U128(value: (string | number | BN)): Id {
		return {
			u128: value,
		};
	}
	static Bytes(value: Array<(number | string | BN)>): Id {
		return {
			bytes: value,
		};
	}
}

export type AccountId = string | number[]

export type Key = string | number[]

export type Asset = {
	assetId: (number | string | BN),
	equippableGroupId: (number | string | BN),
	assetUri: Array<(number | string | BN)>
}

export type Part = {
	partType: PartType,
	z: (number | string | BN),
	equippable: Array<AccountId>,
	metadataUri: Array<(number | string | BN)>,
	isEquippableByAll: boolean
}

export enum PartType {
	none = 'None',
	slot = 'Slot',
	fixed = 'Fixed'
}

export type Equipment = {
	assetId: (number | string | BN),
	childAssetId: (number | string | BN),
	childNft: [AccountId, Id]
}

export type EquippableAsset = {
	groupId: (number | string | BN),
	portIds: Array<(number | string | BN)>
}

export interface PSP34Error {
	custom ? : Array<(number | string | BN)>,
	selfApprove ? : null,
	notApproved ? : null,
	tokenExists ? : null,
	tokenNotExists ? : null,
	safeTransferCheckFailed ? : Array<(number | string | BN)>
}

export class PSP34ErrorBuilder {
	static Custom(value: Array<(number | string | BN)>): PSP34Error {
		return {
			custom: value,
		};
	}
	static SelfApprove(): PSP34Error {
		return {
			selfApprove: null,
		};
	}
	static NotApproved(): PSP34Error {
		return {
			notApproved: null,
		};
	}
	static TokenExists(): PSP34Error {
		return {
			tokenExists: null,
		};
	}
	static TokenNotExists(): PSP34Error {
		return {
			tokenNotExists: null,
		};
	}
	static SafeTransferCheckFailed(value: Array<(number | string | BN)>): PSP34Error {
		return {
			safeTransferCheckFailed: value,
		};
	}
}

export enum OwnableError {
	callerIsNotOwner = 'CallerIsNotOwner',
	newOwnerIsZero = 'NewOwnerIsZero'
}

