<script lang="ts">
	import { getGRPCClientContext } from '$lib/contexts/rpc_client';
	import { route } from '$lib/ROUTES';
	import {
		Navbar,
		NavBrand,
		NavHamburger,
		NavLi,
		NavUl,
		Toast,
		Button,
		DarkMode
	} from 'flowbite-svelte';
	import { derived, writable } from 'svelte/store';
	import resetDB from './navbar/reset';
	import { fade, fly } from 'svelte/transition';
	const transport = getGRPCClientContext();
	const isFetching = writable(false);
	const isDone = writable(false);
	const message = writable<string | undefined>(undefined);
	const errors = writable<Error | undefined>();
	async function reset() {
		if (!$isFetching) {
			$isFetching = true;
			await resetDB(transport)
				.then((result) => {
					message.set('Database reseted');
					errors.set(undefined);
				})
				.catch((error) => {
					message.set(undefined);
					errors.set(error);
				})
				.finally(() => {
					isFetching.set(false);
					isDone.set(true);
				});
		}
	}
	const hasError = derived([errors, message], ([e, m]) => e != undefined && m == undefined);
</script>

<Navbar let:NavContainer color="primary">
	<NavContainer class="rounded-lg border bg-white px-5 py-2 dark:bg-gray-600">
		<NavBrand href={route('/admin')}>
			<span class="self-center whitespace-nowrap text-xl font-semibold">Admin</span>
		</NavBrand>
		<NavHamburger />
		<NavUl>
			<NavLi href={route('/admin/etudiant')}>Etudiants</NavLi>
			<NavLi href={route('/admin/imports')}>Imports</NavLi>
			<NavLi href={route('/admin/config')}>Configurations</NavLi>
			<NavLi href={route("/admin/semestre")}>Semestres</NavLi>
			<NavLi on:click={() => reset()}>Reset database</NavLi>
		</NavUl>
		<div>
			<DarkMode size="md" />
		</div>
	</NavContainer>
</Navbar>

{#if !$isFetching && $isDone}
	<div transition:fade>
		<Toast
			position="bottom-right"
			color={$hasError ? 'red' : 'primary'}
			divClass="w-full max-w-xs p-4 text-gray-500 bg-slate-300 rounded shadow dark:text-gray-400 dark:bg-slate-700 gap-3 z-10"
			on:close={() => {
				isDone.set(false);
			}}
			transition={fly}
			params={{
				x: 300
			}}
		>
			{#if $message != undefined}
				<p class="flex text-green-500">{$message}</p>
			{:else if $hasError}
				<p class="flex flex-col">
					Oh no! we caught some error:
					<span class=" text-red-700 dark:text-red-300">{$errors?.message}</span>
				</p>
			{:else}
				Somewhat done??
			{/if}
		</Toast>
	</div>
{/if}
