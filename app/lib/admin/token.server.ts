import { route } from '$lib/ROUTES';
import generateTokenHandler from '$lib/generators/token/generateTokenHandler.server';
import { type Cookies } from '@sveltejs/kit';
import { ADMIN_TOKEN_KEY } from './token.key';

const { hasToken, getTokenOrRedirect, setToken, deleteToken } =
	generateTokenHandler(ADMIN_TOKEN_KEY);

export function getToken(cookies: Cookies): string {
	return getTokenOrRedirect(cookies, route('/admin/login'));
}

export { deleteToken, hasToken, setToken };
