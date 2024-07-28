// place files you want to import through the `$lib` alias in this folder.

import { Date as CommonDate } from './protos/commons';

export function commonDateToDate(date: CommonDate) {
	return new Date(date.annee, date.mois, date.jour);
}

export function dateToCommonDate(date: Date): CommonDate {
	return {
		jour: date.getDay(),
		mois: date.getMonth(),
		annee: date.getFullYear()
	};
}
