<script lang="ts">
	import ClientNavBar from '$lib/client/components/ClientNavBar.svelte';
	import { setToken } from '$lib/client/token.client';
	import { initGRPCClientContext } from '$lib/contexts/rpc_client';
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
	initGRPCClientContext({
		baseUrl: data.backUrl
	});
	//setToken(data.token);
</script>

<ClientNavBar />

<slot />
