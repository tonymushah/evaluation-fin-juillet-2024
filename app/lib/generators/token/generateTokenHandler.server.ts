import { redirect, type Cookies } from '@sveltejs/kit';

// place files you want to import through the `$lib` alias in this folder.
export default function generateTokenHandler(KEY: string, path?: string) {
	function hasToken(cookies: Cookies): boolean {
		return cookies.get(KEY) != undefined;
	}
	function getToken(cookies: Cookies): string {
		const token = cookies.get(KEY);
		if (token == undefined) {
			throw new Error('Token not found');
		} else {
			return token;
		}
	}
	function getTokenWithCallback(cookies: Cookies, callback: () => never): string {
		try {
			return getToken(cookies);
		} catch {
			callback();
		}
	}
	function getTokenOrRedirect(cookies: Cookies, path: string): string {
		return getTokenWithCallback(cookies, () => {
			redirect(301, path);
		});
	}
	function setToken(cookies: Cookies, value: string) {
		cookies.set(KEY, value, {
			path: path ?? '/'
		});
	}
	function deleteToken(cookies: Cookies) {
		cookies.delete(KEY, {
			path: path ?? '/'
		});
	}
	return {
		hasToken,
		getToken,
		getTokenWithCallback,
		getTokenOrRedirect,
		setToken,
		deleteToken
	};
}
