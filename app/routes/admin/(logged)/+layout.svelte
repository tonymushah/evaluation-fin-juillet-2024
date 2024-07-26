<script lang="ts">
	import AdminNavBar from '$lib/admin/components/AdminNavBar.svelte';
	import { setToken } from '$lib/admin/token.client';
	import { initGRPCClientContext } from '$lib/contexts/rpc_client';
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
	initGRPCClientContext({
		baseUrl: data.backUrl
	});
	// setToken(data.token);
</script>

<AdminNavBar />

<slot />
