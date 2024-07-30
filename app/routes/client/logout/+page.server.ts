import { deleteToken } from '$lib/client/token.server';
import { route } from '$lib/ROUTES.js';
import { redirect } from '@sveltejs/kit';

export const load = async ({ cookies }) => {
	deleteToken(cookies);
	redirect(300, route('/client/login'));
};
