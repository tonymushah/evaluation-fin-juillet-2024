import noteInsert from '$lib/admin/modules/noteInsert.server';
import type { InsertNotesRequest } from '$lib/protos/admin';
import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

type InsertValue = {
	matiere?: string;
	valeur?: number;
};
export const POST: RequestHandler = async ({ params, request }) => {
	const etudiant = params.etu;
	const to_send: InsertNotesRequest[] = [];
	try {
		const jsonBody: Record<string, InsertValue> = await request.json();

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
