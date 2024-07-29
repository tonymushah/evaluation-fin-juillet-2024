import { Genre, ReleveNoteStatus, type Etudiant } from '$lib/protos/commons';
import { faker } from '@faker-js/faker';
import type { PageServerLoad } from './$types';
import { dateToCommonDate } from '$lib';
import dayjs from 'dayjs';
import type { EtudiantSemestre } from '$lib/admin/components/etudiant-page/semetres/SemestreTable.svelte';

export const load: PageServerLoad = async () => {
	const birthday = faker.date.birthdate();
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
	const etudiant: Etudiant = {
		nom: faker.person.lastName(),
		prenom: faker.person.firstName(),
		dateNaissance: dateToCommonDate(birthday),
		numero: 'Numero etu',
		age: dayjs(new Date()).diff(birthday, 'years'),
		promotion: 'P15',
		genre: Genre.G_AUTRE
	};
	return {
		etudiant,
		semestres
	};
};
