<script lang="ts">
	import { goto } from '$app/navigation';
	import { route } from '$lib/ROUTES';

	import { Button, Heading, Hr, P } from 'flowbite-svelte';
	let admis = NaN;
	let ajournee = NaN;
</script>

<svelte:head>
	<title>Admin</title>
</svelte:head>

<div class="mx-10">
	<section>
		<Heading tag="h2">Statistique etudiant</Heading>
		<P
			>Admis en licence: {#if isNaN(admis)}
				NaN
			{:else}
				{admis}
			{/if}
		</P>
		<P
			>Ajourees: {#if isNaN(ajournee)}
				NaN
			{:else}
				{ajournee}
			{/if}</P
		>
	</section>
	<Hr />
	<section>
		<Heading class="mb-3" tag="h2">Insertions</Heading>
		<div>
			<Button on:click={() => goto(route('/admin/note-insert'))}>Insertion des notes</Button>
		</div>
	</section>
</div>
