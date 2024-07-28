<script lang="ts">
	import ReleveNoteTable from '$lib/common/component/ReleveNoteTable.svelte';
	import type { PageServerData } from './$types';
	export let data: PageServerData;
</script>

<ReleveNoteTable data={data.releve} />
