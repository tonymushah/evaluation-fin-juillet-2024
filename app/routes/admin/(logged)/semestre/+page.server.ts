import { GetSemetresResponse } from '$lib/protos/admin';
import { GettersClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async function () {
	const gettersClient = new GettersClient(adminClient);
	const res = await gettersClient.semetres({});
	const semetres: GetSemetresResponse = GetSemetresResponse.toJson(res.response);
	return {
		semetres: semetres.semestre
	};
};
