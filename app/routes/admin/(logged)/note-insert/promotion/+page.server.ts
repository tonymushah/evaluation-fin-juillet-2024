import { GettersClient, NotesClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';
import type { Actions, PageServerLoad } from './$types';

async function getMatieres(client: GettersClient): Promise<string[]> {
	return (
		await client.matieres({
			limit: 100,
			semstres: []
		})
	).response.matieres.map((e) => e.numero);
}

async function getPromotions(client: GettersClient): Promise<string[]> {
	return (await client.promotions({})).response.promotions;
}

export const load: PageServerLoad = async () => {
	const client = new GettersClient(adminClient);
	return {
		matieres: await getMatieres(client),
		promotions: await getPromotions(client)
	};
};

export const actions = {
	async default({ request }) {
		const formData = await request.formData();
		let errors: string | undefined = undefined;
		const matiere = formData.get('matiere');
		const promotion = formData.get('promotion');
		const note = Number(formData.get('note'));
		if (typeof matiere == 'string' && typeof promotion == 'string' && !isNaN(note)) {
			const client = new NotesClient(adminClient);
			try {
				await client.insertParPromotion({
					matiere,
					promotion,
					note
				});
			} catch (error) {
				errors = error.message;
			}
		} else {
			errors = 'Invalid input';
		}
		return {
			errors
		};
	}
} satisfies Actions;
