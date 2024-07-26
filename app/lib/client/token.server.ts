import { route } from '$lib/ROUTES';
import generateTokenHandler from '$lib/generators/token/generateTokenHandler.server';
import { type Cookies } from '@sveltejs/kit';
import { CLIENT_TOKEN_KEY as TOKEN_KEY } from './token.key';

const { hasToken, getTokenOrRedirect, setToken, deleteToken } = generateTokenHandler(TOKEN_KEY);

export function getToken(cookies: Cookies): string {
	return getTokenOrRedirect(cookies, route('/client/login'));
}

export { deleteToken, hasToken, setToken };
