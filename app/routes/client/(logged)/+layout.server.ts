import { CLIENT_BACK_URL } from '$env/static/private';
import { getToken } from '$lib/client/token.server';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	return {
		token: getToken(cookies),
		backUrl: CLIENT_BACK_URL
	};
};
