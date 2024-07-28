<script lang="ts">
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		Checkbox,
		TableSearch
	} from 'flowbite-svelte';
	import EtudiantTableHead from './EtudiantTableHead.svelte';
	import type { Etudiant } from '$lib/protos/commons';
	import { route } from '$lib/ROUTES';
	import { goto } from '$app/navigation';

	export let etudiants: Etudiant[];
	export let input: string = '';
</script>

<TableSearch
	bind:inputValue={input}
	inputClass="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-2/3 p-2.5 ps-10 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
	hoverable
	placeholder="Recherche par nom et utiliser (Promotion:p1,p2) pour filtrer"
>
	<EtudiantTableHead />
	<TableBody>
		{#each etudiants as { nom, numero, prenom, dateNaissance, age }}
			<a
				class="contents"
				href={route('/admin/etudiant/[etu]', {
					etu: numero
				})}
			>
				<TableBodyRow>
					<TableBodyCell>
						{numero}
					</TableBodyCell>
					<TableBodyCell>
						<span>{nom}</span>
						&nbsp;
						<span>
							{prenom}
						</span>
					</TableBodyCell>
					<TableBodyCell>
						{#if dateNaissance}
							{new Date(dateNaissance.annee, dateNaissance.mois, dateNaissance.jour).toDateString()}
						{:else}
							NaN
						{/if}
					</TableBodyCell>
					<TableBodyCell>
						{age}
						{#if age > 1}
							ans
						{:else}
							an
						{/if}
					</TableBodyCell>
				</TableBodyRow>
			</a>
		{/each}
	</TableBody>
</TableSearch>
