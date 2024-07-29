import { setToken } from '$lib/client/token.server';
import { AuthClient } from '$lib/protos/client.client';
import { clientClient } from '$lib/server/protoclients';
import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { route } from '$lib/ROUTES';

export const ssr = true;

export const actions = {
	async default({ request, cookies }) {
		let error: string | undefined = undefined;
		const formData = await request.formData();
		const client = new AuthClient(clientClient);
		const numero_etudiant = formData.get('numero_etudiant');
		console.log(numero_etudiant);
		if (typeof numero_etudiant == 'string') {
			try {
				const res = await client.login({
					numero: numero_etudiant
				});
				setToken(cookies, res.response.token);
				redirect(300, route('/client'));
			} catch (_error) {
				if (_error instanceof Error) {
					error = _error.message;
				} else {
					error = JSON.stringify(_error);
				}
			}
		} else {
			error = 'invalid etudiant input';
		}
		return {
			error
		};
	}
} satisfies Actions;
