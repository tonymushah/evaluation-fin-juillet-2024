<script lang="ts">
	import { Button, Heading, Input, Label, Modal, P } from 'flowbite-svelte';
	import type { ActionData, PageServerData } from './$types';
	import ConfigsTable from '$lib/admin/components/config/ConfigsTable.svelte';
	import { goto } from '$app/navigation';
	import { route } from '$lib/ROUTES';
	import type { ConfigNote } from '$lib/protos/admin';
	import { derived, get, writable, type Writable } from 'svelte/store';
	import { enhance } from '$app/forms';

	export let data: PageServerData;

	export let form: ActionData;
	let inner_form: HTMLFormElement | undefined = undefined;
	$: list_map = new Map(data.list.map((cfg) => [cfg.code, cfg]));
	const selected = writable<ConfigNote | undefined>(undefined);
	function select(code: string) {
		selected.set(list_map.get(code));
	}
	const open_read = derived(selected, ($s) => $s != undefined);
	const open: Writable<boolean> = {
		subscribe(run, invalidate) {
			return open_read.subscribe(run, invalidate);
		},
		set(value) {
			if (value == false) {
				selected.set(undefined);
			}
		},
		update(updater) {
			const value = updater(get(open_read));
			if (value == false) {
				selected.set(undefined);
			}
		}
	};
</script>

<svelte:head>
	<title>Configuration notes</title>
</svelte:head>

<Heading tag="h2">Configuration</Heading>

<P>Commancer a configurer les donnes des maintenant</P>

{#if form?.errors}
	<P color="text-red-900 dark:text-red-300">{form.errors}</P>
{/if}

<ConfigsTable data={data.list} on:cellClick={(e) => select(e.detail.code)} />

<Modal title="Metre a jour la configuration" bind:open={$open} autoclose outsideclose>
	{#if $selected}
		<form method="post" bind:this={inner_form} use:enhance class="grid gap-3">
			<input hidden name="code" value={$selected.code} />
			<div class="grid gap-1">
				<Label class="text-xl">Nom</Label>
				<Input name="nom" type="text" value={$selected.name} />
			</div>
			<div class="grid gap-1">
				<Label class="text-xl">Valeur</Label>
				<Input name="valeur" type="number" value={$selected.value} />
			</div>
		</form>
	{/if}
	<svelte:fragment slot="footer">
		<Button on:click={() => inner_form?.submit()}>Update</Button>
		<Button color="alternative">Cancel</Button>
	</svelte:fragment>
</Modal>
