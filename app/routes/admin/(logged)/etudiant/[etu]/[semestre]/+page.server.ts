import { EtudiantsClient } from '$lib/protos/admin.client';
import { ReleveNote, ReleveNoteStatus, ReleveNoteUnitStatus } from '$lib/protos/commons';
import { adminClient } from '$lib/server/protoclients';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const etudiant_client = new EtudiantsClient(adminClient);
	const releve: ReleveNote = ReleveNote.toJson(
		(
			await etudiant_client.releveNote({
				semestre: [params.semestre],
				etudiant: params.etu
			})
		).response.releves[0]
	);
	console.log(releve);
	return {
		releve
	};
};
