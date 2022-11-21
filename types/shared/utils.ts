import fs from "fs";
import type {ContractPromise} from "@polkadot/api-contract";
import {handleEventReturn} from "@supercolony/typechain-types";

export function getTypeDescription(id: number | string, fileName: string): any {
	const types = JSON.parse(fs.readFileSync(__dirname + `/../data/${fileName}.json`, 'utf8'));
	return types[id];
}

export function getEventTypeDescription(name: string, fileName: string): any {
	const types = JSON.parse(fs.readFileSync(__dirname + `/../event-data/${fileName}.json`, 'utf8'));
	return types[name];
}

export function decodeEvents(events: any[], contract: ContractPromise, fileName: string): any[] {
	return events.filter((record: any) => {
		const { event } = record;

		const [address, data] = record.event.data;

		return event.method == 'ContractEmitted' && address.toString() === contract.address.toString();
	}).map((record: any) => {
		const [address, data] = record.event.data;

		const {args, event} = contract.abi.decodeEvent(data);

		let _event: Record < string, any > = {};

		for (let i = 0; i < args.length; i++) {
			_event[event.args[i]!.name] = args[i]!.toJSON();
		}

		handleEventReturn(_event, getEventTypeDescription(event.identifier.toString(), fileName));

		return {
			name: event.identifier.toString(),
			args: _event,
		};
	});
}