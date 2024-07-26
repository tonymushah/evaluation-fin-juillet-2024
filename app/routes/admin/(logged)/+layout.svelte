<script lang="ts">
	import { setToken } from '$lib/admin/token.client';
	import { initGRPCClientContext } from '$lib/rpc_client.context';
	import type { LayoutServerData } from './$types';

	export let data: LayoutServerData;
	initGRPCClientContext({
		baseUrl: data.backUrl
	});
	// setToken(data.token);
</script>

<slot />
