import type { Etudiant } from '$lib/protos/commons';
import type { LayoutServerLoad } from './$types';
import { faker } from '@faker-js/faker';

export const ssr = true;

export const load: LayoutServerLoad = async function () {
	const etudiant: Etudiant = {
		nom: faker.person.lastName(),
		prenom: faker.person.firstName(),
		dateNaissance: {
			annee: 2000,
			jour: 12,
			mois: 1
		},
		numero: 'Numero etu',
		age: 24,
		promotion: 'P15'
	};
	return {
		etudiant
	};
};
