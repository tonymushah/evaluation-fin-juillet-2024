<script lang="ts" context="module">
	import { ReleveNoteStatus } from '$lib/protos/commons';
</script>

<script lang="ts">
	import type { EtudiantSemestre } from '$lib/admin/components/etudiant-page/semetres/SemestreTable.svelte';
	import SemestreTableHead from '$lib/admin/components/etudiant-page/semetres/SemestreTableHead.svelte';
	import { route } from '$lib/ROUTES';
	import { P, Table, TableBody, TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	export let data: EtudiantSemestre[];
	$: isAjournee = (semestre: string) => {
		return data.find((sem) => sem.semestre == semestre)?.status == ReleveNoteStatus.S_AJOURNEE;
	};
	$: data_ = data.sort((a, b) => a.moyenne - b.moyenne);
</script>

<Table hoverable shadow color="blue">
	<SemestreTableHead />
	<TableBody tableBodyClass="divide-y">
		{#each data as { semestre, status, moyenne }, rang}
			<a
				class="contents"
				href={route('/client/semestre/[sem]', {
					sem: semestre
				})}
			>
				<TableBodyRow color={isAjournee(semestre) ? 'red' : 'default'}>
					<TableBodyCell>
						<P>
							{rang + 1}
						</P>
					</TableBodyCell>
					<TableBodyCell>
						<P>
							{semestre}
						</P>
					</TableBodyCell>
					<TableBodyCell>
						{moyenne}
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
