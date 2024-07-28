import type { EtudiantSemestre } from '$lib/admin/components/etudiant-page/semetres/SemestreTable.svelte';
import { ReleveNoteStatus } from '$lib/protos/commons';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async function () {
	const semestres: EtudiantSemestre[] = [
		{
			semestre: 'S1',
			status: ReleveNoteStatus.S_VALID
		},
		{
			semestre: 'S2',
			status: ReleveNoteStatus.S_AJOURNEE
		},
		{
			semestre: 'S3',
			status: ReleveNoteStatus.S_AJOURNEE
		},
		{
			semestre: 'S4',
			status: ReleveNoteStatus.S_AJOURNEE
		},
		{
			semestre: 'S5',
			status: ReleveNoteStatus.S_VALID
		},
		{
			semestre: 'S6',
			status: ReleveNoteStatus.S_VALID
		}
	];
	return {
		semestres
	};
};
