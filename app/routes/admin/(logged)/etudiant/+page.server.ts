import { EtudiantsClient } from '$lib/protos/admin.client.js';
import { EtudiantsListResponse } from '$lib/protos/admin.js';
import { adminClient } from '$lib/server/protoclients.js';

export const load = async ({ url }) => {
	const etudiant_client = new EtudiantsClient(adminClient);
	const res: EtudiantsListResponse = EtudiantsListResponse.toJson(
		(
			await etudiant_client.list({
				limit: 100,
				promotion: []
			})
		).response
	);
	return {
		res
	};
};
