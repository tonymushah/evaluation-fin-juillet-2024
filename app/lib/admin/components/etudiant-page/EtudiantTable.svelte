<script lang="ts">
	import { commonDateToDate } from '$lib';
	import { Etudiant, EtudiantStatus } from '$lib/protos/commons';
	import { route } from '$lib/ROUTES';
	import { Table, TableBody, TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	import EtudiantTableHead from './EtudiantTableHead.svelte';

	export let etudiants: Etudiant[];
</script>

<Table
	inputClass="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-2/3 p-2.5 ps-10 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
	hoverable
>
	<EtudiantTableHead />
	<TableBody tableBodyClass="divide-y">
		{#each etudiants as { nom, numero, prenom, dateNaissance, age, promotion, moyenne, status }}
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
					<TableBodyCell>
						{moyenne ?? 0}
					</TableBodyCell>
					<TableBodyCell>
						{#if status == EtudiantStatus[EtudiantStatus.E_AJOURNEE]}
							Ajournee
						{:else}
							Admis
						{/if}
					</TableBodyCell>
				</TableBodyRow>
			</a>
		{/each}
	</TableBody>
</Table>
