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
	import { commonDateToDate } from '$lib';

	export let etudiants: Etudiant[];
</script>

<Table
	inputClass="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-2/3 p-2.5 ps-10 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
	hoverable
>
	<EtudiantTableHead />
	<TableBody tableBodyClass="divide-y">
		{#each etudiants as { nom, numero, prenom, dateNaissance, age, promotion }}
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
							{commonDateToDate(dateNaissance).toDateString()}
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
					<TableBodyCell>
						{promotion}
					</TableBodyCell>
				</TableBodyRow>
			</a>
		{/each}
	</TableBody>
</Table>
