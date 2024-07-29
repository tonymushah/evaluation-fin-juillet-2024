<script lang="ts">
	import { Input, TableBodyCell } from 'flowbite-svelte';
	import type { Writable } from 'svelte/store';
	export let valeur: Writable<number | undefined>;
	export let matiere: Writable<string | undefined>;
</script>

<TableBodyCell>
	<Input bind:value={$matiere} />
</TableBodyCell>

<TableBodyCell>
	<Input bind:value={$valeur} />
</TableBodyCell>
