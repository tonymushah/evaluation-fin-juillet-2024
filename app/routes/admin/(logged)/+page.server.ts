import { EtudiantAdmisOuNonResponse } from '$lib/protos/admin';
import { GettersClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const gettersClient = new GettersClient(adminClient);
	const admisData: EtudiantAdmisOuNonResponse = EtudiantAdmisOuNonResponse.toJson(
		(await gettersClient.etudiantAdmisOuNon({})).response
	);
	return {
		admisData
	};
};
