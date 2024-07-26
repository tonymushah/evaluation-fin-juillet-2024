import { ADMIN_BACK_URL } from '$env/static/private';
import { getToken } from '$lib/admin/token.server';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	return {
		//token: getToken(cookies),
		backUrl: ADMIN_BACK_URL
	};
};
