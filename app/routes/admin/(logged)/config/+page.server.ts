import { ConfigNote, GetConfigsMessageResponse } from '$lib/protos/admin';
import { CfgNoteServiceClient } from '$lib/protos/admin.client';
import { Empty } from '$lib/protos/commons';
import { adminClient } from '$lib/server/protoclients';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const cfg_client = new CfgNoteServiceClient(adminClient);
	const res: GetConfigsMessageResponse = GetConfigsMessageResponse.toJson(
		(await cfg_client.getConfigs(Empty)).response
	);
	return {
		list: res.configs
	};
};

export const actions = {
	async default({ request }) {
		const formData = await request.formData();
		let code;

		const form_code = formData.get('code');
		if (typeof form_code == 'string') {
			code = form_code;
		} else {
			return {
				errors: 'Invalid code input'
			};
		}
		let nom;
		const form_nom = formData.get('nom');
		if (typeof form_nom == 'string') {
			nom = form_nom;
		} else {
			return {
				code,
				errors: 'Invalid nom input'
			};
		}
		let valeur;
		const form_valeur = formData.get('valeur');
		if (typeof form_valeur == 'string') {
			valeur = Number(form_valeur);
			if (isNaN(valeur)) {
				return {
					code,
					errors: 'Invalid valeur input'
				};
			}
		} else {
			return {
				code,
				errors: 'Invalid valeur input'
			};
		}
		const toSend: ConfigNote = {
			code,
			name: nom,
			value: valeur
		};
		console.log(toSend);
		const cfg_client = new CfgNoteServiceClient(adminClient);
		await cfg_client.updateConfig(ConfigNote.fromJson(toSend));
	}
} satisfies Actions;
