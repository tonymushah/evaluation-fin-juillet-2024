import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { route } from '$lib/ROUTES';

export const ssr = false;

export const load: PageServerLoad = async () => {
	redirect(300, route('/client'));
};
