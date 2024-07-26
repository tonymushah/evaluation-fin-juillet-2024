import { goto } from '$app/navigation';
import { getContext, setContext } from 'svelte';

export default function generateTokenHandler(KEY: string) {
	function getToken(): string {
		const context = getContext<string | undefined>(KEY);
		if (context != undefined) {
			return context;
		} else {
			throw new Error(`${KEY} context is undefined`);
		}
	}
	return {
		setToken(value: string): string {
			return setContext(KEY, value);
		},
		getToken,
		getTokenOrRedirect(path: string): string {
			try {
				return getToken();
			} catch (error) {
				goto(path);
				throw error;
			}
		}
	};
}
