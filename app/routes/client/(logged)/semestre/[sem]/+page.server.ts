import { getToken } from '$lib/client/token.server';
import { ReleveClient } from '$lib/protos/client.client';
import { ReleveNote } from '$lib/protos/commons';
import { clientClient } from '$lib/server/protoclients';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, params }) => {
	const token = getToken(cookies);
	const releve_client = new ReleveClient(clientClient);
	const releve: ReleveNote = ReleveNote.toJson(
		(
			await releve_client.get(
				{
					semetre: params.sem
				},
				{
					meta: {
						authorization: token
					}
				}
			)
		).response.releves!
	);
	return {
		releve
	};
};
