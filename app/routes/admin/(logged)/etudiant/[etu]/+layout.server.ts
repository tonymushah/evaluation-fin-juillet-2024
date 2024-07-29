import { dateToCommonDate } from '$lib';
import { Genre, type Etudiant } from '$lib/protos/commons';
import dayjs from 'dayjs';
import type { LayoutServerLoad } from './$types';
import { faker } from '@faker-js/faker';

export const ssr = true;

export const load: LayoutServerLoad = async function () {
	const birthday = faker.date.birthdate();
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
		etudiant
	};
};
