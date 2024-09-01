import type { InsertNotesRequest } from '$lib/protos/admin';
import { NotesClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';

export default async function noteInsert(
	toSend: InsertNotesRequest[]
): Promise<Map<string, string>> {
	const insertClient = new NotesClient(adminClient);
	const results = new Map<string, string>();
	const channel = insertClient.insert();
	const messageUnli = channel.responses.onMessage((mess) => {
		results.set(mess.reqId, mess.customMessage);
	});
	for (let index = 0; index < toSend.length; index++) {
		const element = toSend[index];
		await channel.requests.send(element);
	}
	channel.requests.complete();
	return new Promise((res, rej) => {
		const completeUnli = channel.responses.onComplete(() => {
			res(results);
		});
		const errorUnli = channel.responses.onError((e) => {
			rej(e);
		});
		return {
			[Symbol.dispose]: () => {
				completeUnli();
				messageUnli();
				errorUnli();
			}
		};
	});
}
