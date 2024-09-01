import type { EtudiantSemestre } from '$lib/admin/components/etudiant-page/semetres/SemestreTable.svelte';
import { EtudiantsClient, GettersClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';
import type { PageServerLoad } from './$types';

async function getSemestres(): Promise<string[]> {
	const client = new GettersClient(adminClient);
	return (await client.semetres({})).response.semestre.map((sem) => sem.numero);
}

export const load: PageServerLoad = async function ({ params }) {
	const sems = await getSemestres();
	const etudiant_client = new EtudiantsClient(adminClient);

	const semestres: EtudiantSemestre[] = (
		await etudiant_client.releveNote({
			etudiant: params.etu,
			semestre: sems
		})
	).response.releves.map((rel) => ({
		semestre: rel.semestre,
		status: rel.status,
		moyenne: rel.moyenne
	}));
	return {
		semestres
	};
};
