<script lang="ts" context="module">
	import { ReleveNoteStatus } from '$lib/protos/commons';
	export type EtudiantSemestre = {
		semestre: string;
		status: ReleveNoteStatus;
	};
</script>

<script lang="ts">
	import { P, Table, TableBody, TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	import SemestreTableHead from './SemestreTableHead.svelte';
	import { route } from '$lib/ROUTES';
	import { page } from '$app/stores';
	export let data: EtudiantSemestre[];
	$: isAjournee = (semestre: string) => {
		return data.find((sem) => sem.semestre == semestre)?.status == ReleveNoteStatus.S_AJOURNEE;
	};
</script>

<Table hoverable shadow color="blue">
	<SemestreTableHead />
	<TableBody tableBodyClass="divide-y">
		{#each data as { semestre, status }}
			<a
				class="contents"
				href={route('/admin/etudiant/[etu]/[semestre]', {
					etu: $page.params.etu,
					semestre
				})}
			>
				<TableBodyRow color={isAjournee(semestre) ? 'red' : 'default'}>
					<TableBodyCell>
						<P>
							{semestre}
						</P>
					</TableBodyCell>
					<TableBodyCell>
						<P>
							{#if status == ReleveNoteStatus.S_VALID}
								Valide
							{:else if status == ReleveNoteStatus.S_AJOURNEE}
								Ajournee
							{/if}
						</P>
					</TableBodyCell>
				</TableBodyRow>
			</a>
		{/each}
	</TableBody>
</Table>
