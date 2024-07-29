<script lang="ts">
	import ParEtudiant from '$lib/admin/components/insert-note/ParEtudiant.svelte';
	import ParMatiere from '$lib/admin/components/insert-note/ParMatiere.svelte';
	import { Heading, Tabs } from 'flowbite-svelte';
</script>

<div class="mx-10 grid gap-2">
	<Heading tag="h2">Insertion des notes</Heading>
	<Tabs tabStyle="underline">
		<ParEtudiant />
		<ParMatiere />
	</Tabs>
</div>
