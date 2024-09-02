import { EtudiantRangsResponse } from '$lib/protos/admin';
import { SemetresServClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async function ({ params }) {
	const semestre = params.sem;
	const semServClient = new SemetresServClient(adminClient);
	const res = await semServClient.etudiantRangs({
		semestre
	});
	const semetres: EtudiantRangsResponse = EtudiantRangsResponse.toJson(res.response);
	return {
		semestre,
		etudiants: semetres.entries
	};
};
