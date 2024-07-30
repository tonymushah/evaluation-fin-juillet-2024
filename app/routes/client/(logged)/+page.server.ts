import type { EtudiantSemestre } from '$lib/admin/components/etudiant-page/semetres/SemestreTable.svelte';
import { getToken } from '$lib/client/token.server';
import { CurrentClient, SemestresClient } from '$lib/protos/client.client';
import { Etudiant } from '$lib/protos/commons';
import { clientClient } from '$lib/server/protoclients';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies }) => {
	const token = getToken(cookies);
	const cur_client = new CurrentClient(clientClient);
	const etudiant: Etudiant = Etudiant.toJson(
		(
			await cur_client.get(
				{},
				{
					meta: {
						authorization: token
					}
				}
			)
		).response
	)!;
	const semestres_cli = new SemestresClient(clientClient);
	const semestres: EtudiantSemestre[] = (
		await semestres_cli.list(
			{
				semestre: []
			},
			{
				meta: {
					authorization: token
				}
			}
		)
	).response.semetres.map((sm) => ({
		moyenne: sm.moyenne,
		semestre: sm.semetre,
		status: sm.status
	}));
	console.log(semestres);
	return {
		etudiant,
		semestres
	};
};
