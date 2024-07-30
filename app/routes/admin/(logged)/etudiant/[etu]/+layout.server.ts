import { EtudiantInfoResponse } from '$lib/protos/admin';
import { EtudiantsClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';
import type { LayoutServerLoad } from './$types';

export const ssr = true;

export const load: LayoutServerLoad = async function ({ params }) {
	const etudiant_client = new EtudiantsClient(adminClient);
	const res: EtudiantInfoResponse = EtudiantInfoResponse.toJson(
		(
			await etudiant_client.info({
				numero: params.etu
			})
		).response
	);
	return {
		etudiant: res.current!
	};
};
