import { getToken } from '$lib/client/token.server';
import { ListeRatrapageResponse } from '$lib/protos/client';
import { ReleveClient } from '$lib/protos/client.client';
import { clientClient } from '$lib/server/protoclients';
import { reduce } from 'lodash-es';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async function ({ cookies }) {
	const token = getToken(cookies);
	const ratrapage_client = new ReleveClient(clientClient);
	const result = await ratrapage_client.listeRatrapage(
		{},
		{
			meta: {
				authorization: token
			}
		}
	);
	const total_montant = reduce(
		result.response.list,
		(total, i) => {
			return total + i.montant;
		},
		0
	);
	const res_data: ListeRatrapageResponse = ListeRatrapageResponse.toJson(result.response);
	return {
		ratrapages: res_data.list,
		total_montant
	};
};
