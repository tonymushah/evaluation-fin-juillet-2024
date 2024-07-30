import { hasToken, setToken } from '$lib/client/token.server';
import { AuthClient } from '$lib/protos/client.client';
import { clientClient } from '$lib/server/protoclients';
import { redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { route } from '$lib/ROUTES';

export const ssr = false;

export const load: PageServerLoad = async ({ cookies }) => {
	if (hasToken(cookies)) {
		redirect(300, route('/client'));
	}
};

export const actions = {
	async default({ request, cookies }) {
		let error: string | undefined = undefined;
		const formData = await request.formData();
		const client = new AuthClient(clientClient);
		const numero_etudiant = formData.get('numero_etudiant');
		console.log(numero_etudiant);
		if (typeof numero_etudiant == 'string') {
			let should_redirect = false;
			try {
				const res = await client.login({
					numero: numero_etudiant
				});
				setToken(cookies, res.response.token);
				should_redirect = true;
			} catch (_error) {
				if (_error instanceof Error) {
					error = _error.message;
				} else {
					error = JSON.stringify(_error);
				}
			}
			if (should_redirect) {
				redirect(300, route('/client'));
			}
		} else {
			error = 'invalid etudiant input';
		}
		return {
			error
		};
	}
} satisfies Actions;
