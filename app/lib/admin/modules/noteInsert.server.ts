import type { InsertNotesRequest } from '$lib/protos/admin';
import { NotesClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';

export default async function noteInsert(
	_toSend: InsertNotesRequest[]
): Promise<Map<string, string>> {
	const toSend = _toSend.map<InsertNotesRequest>((v) => ({
		...v,
		note: Number(v.note)
	}));
	const insertClient = new NotesClient(adminClient);
	const results = new Map<string, string>();
	const channel = insertClient.insert();
	console.log('on mess');
	channel.responses.onMessage((mess) => {
		console.log(mess);
		results.set(mess.reqId, mess.customMessage);
	});
	channel.responses.onError((err) => {
		throw err;
	});
	console.log('sedding');
	for (let index = 0; index < toSend.length; index++) {
		const element = toSend[index];
		await channel.requests.send(element);
		console.log(element);
	}
	await channel.requests.complete();
	return results;
}
