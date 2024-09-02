import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import type { InsertNotesRequest } from '$lib/protos/admin';
import noteInsert from '$lib/admin/modules/noteInsert.server';

type InsertValue = {
	matiere?: string;
	valeur?: number;
};
export const POST: RequestHandler = async ({ params, request }) => {
	const etudiant = params.etu;
	const jsonBody: Record<string, InsertValue> = await request.json();
	const to_send: InsertNotesRequest[] = [];
	Object.entries(jsonBody).forEach(([id, value]) => {
		if (value.matiere && value.valeur) {
			to_send.push({
				etudiant,
				matiere: value.matiere,
				note: value.valeur,
				reqId: id
			});
		}
	});
	console.log(to_send);
	try {
		const result = await noteInsert(to_send);
		return json(Object.fromEntries(result));
	} catch (e) {
		return json(
			{
				message: e.message
			},
			{
				status: 500
			}
		);
	}
};
