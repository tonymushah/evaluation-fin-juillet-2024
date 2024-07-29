import { ReleveNoteStatus, ReleveNoteUnitStatus, type ReleveNote } from '$lib/protos/commons';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const releve: ReleveNote = {
		credits: 0 as unknown as bigint,
		semestre: 'S1',
		status: ReleveNoteStatus.S_AJOURNEE,
		notes: [
			{
				matiere: {
					nom: 'some matiere',
					numero: 'some num',
					credits: 12
				},
				status: ReleveNoteUnitStatus.M_AJOURNEE,
				valeur: 5
			}
		]
	};
	return {
		releve
	};
};
