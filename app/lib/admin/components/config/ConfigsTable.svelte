<script lang="ts">
	import type { ConfigNote } from '$lib/protos/admin';
	import { Table, TableBody, TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	import { createEventDispatcher } from 'svelte';
	import ConfigsTableHeader from './table/ConfigsTableHeader.svelte';

	export let data: ConfigNote[];
	const dispatch = createEventDispatcher<{
		cellClick: MouseEvent & {
			code: string;
		};
	}>();
</script>

<Table>
	<ConfigsTableHeader />
	<TableBody>
		{#each data as config}
			<TableBodyRow
				class="hover:bg-slate-400 dark:hover:bg-slate-600"
				on:click={(e) => {
					dispatch('cellClick', {
						...e,
						code: config.code
					});
				}}
			>
				<TableBodyCell>{config.code}</TableBodyCell>
				<TableBodyCell>{config.name}</TableBodyCell>
				<TableBodyCell>{config.value}</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>
