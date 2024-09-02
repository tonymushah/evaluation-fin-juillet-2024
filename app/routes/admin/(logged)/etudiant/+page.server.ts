import { EtudiantsClient } from '$lib/protos/admin.client.js';
import { EtudiantsListResponse } from '$lib/protos/admin.js';
import { adminClient } from '$lib/server/protoclients.js';

export const load = async ({ url }) => {
	const nom = url.searchParams.get('nom') ?? '';
	const promotion = url.searchParams
		.get('promotions')
		?.split(',')
		.map((p) => p.trim())
		.filter((p) => p.length != 0);
	const etudiant_client = new EtudiantsClient(adminClient);
	console.log(nom, promotion);
	const res: EtudiantsListResponse = EtudiantsListResponse.toJson(
		(
			await etudiant_client.list({
				limit: 100,
				promotion: promotion ?? [],
				nom: (() => {
					if (nom.length == 0) {
						return undefined;
					} else {
						return nom;
					}
				})()
			})
		).response
	);
	return {
		res
	};
};
