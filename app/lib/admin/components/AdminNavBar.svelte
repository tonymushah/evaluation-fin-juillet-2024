<script lang="ts">
	import { getGRPCClientContext } from '$lib/contexts/rpc_client';
	import { route } from '$lib/ROUTES';
	import { Navbar, NavBrand, NavHamburger, NavLi, NavUl, Toast, Button } from 'flowbite-svelte';
	import { derived, writable } from 'svelte/store';
	import resetDB from './navbar/reset';
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
				})
				.catch((error) => {
					errors.set(new Error(error));
				})
				.finally(() => {
					isFetching.set(false);
					isDone.set(true);
				});
		}
	}
	const hasError = derived(errors, (e) => e != undefined);
</script>

<Navbar let:NavContainer color="primary">
	<NavContainer class="rounded-lg border bg-white px-5 py-2 dark:bg-gray-600">
		<NavBrand href={route('/admin')}>
			<span class="self-center whitespace-nowrap text-xl font-semibold">Admin</span>
		</NavBrand>
		<NavHamburger />
		<NavUl>
			<NavLi href={route('/admin/imports')}>Imports</NavLi>

			<NavLi on:click={() => reset()}>Reset database</NavLi>
		</NavUl>
	</NavContainer>
</Navbar>

{#if !$isFetching && $isDone}
	<Toast
		color={$hasError ? 'red' : 'primary'}
		on:close={() => {
			isDone.set(false);
		}}
	>
		{#if $hasError}
			<p class="flex flex-col">
				Oh no! we caught some error:
				<span class=" text-red-700">{$errors?.message}</span>
			</p>
		{:else if $message != undefined}
			<p class="flex text-green-500">{$message}</p>
		{:else}
			Somewhat done??
		{/if}
	</Toast>
{/if}
