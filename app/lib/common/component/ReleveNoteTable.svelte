<script lang="ts">
	import { ReleveNoteStatus, ReleveNoteUnitStatus, type ReleveNote } from '$lib/protos/commons';
	import {
		Heading,
		P,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';

	export let data: ReleveNote;
	$: isAjournee = (matiere: string) => {
		return (
			data.notes.find((mat) => mat.matiere?.numero == matiere)?.status ==
			ReleveNoteUnitStatus.M_AJOURNEE
		);
	};
</script>

<div class="flex w-screen items-center justify-center">
	<div class="w-11/12">
		<Heading tag="h2" class="text-xl">Semestre: {data.semestre}</Heading>
		<P>
			Status:
			{#if data.status == ReleveNoteStatus.S_VALID}
				Valide
			{:else if data.status == ReleveNoteStatus.S_AJOURNEE}
				Ajournee
			{/if}
		</P>
		<Table>
			<TableHead>
				<TableHeadCell>Matiere</TableHeadCell>
				<TableHeadCell>Note</TableHeadCell>
				<TableHeadCell>Credit</TableHeadCell>
				<TableHeadCell>Status</TableHeadCell>
			</TableHead>
			<TableBody tableBodyClass="divide-y">
				{#each data.notes as note}
					<TableBodyRow color={isAjournee(note.matiere?.numero ?? '') ? 'red' : 'default'}>
						<TableBodyCell>
							{#if note.matiere != undefined}
								{note.matiere.numero} - {note.matiere.nom}
							{/if}
						</TableBodyCell>
						<TableBodyCell>
							{note.valeur}
							{#if note.valeur > 1}
								pts
							{:else}
								pt
							{/if}
						</TableBodyCell>
						<TableBodyCell>
							{#if note.status != ReleveNoteUnitStatus.M_AJOURNEE}
								{note.matiere?.credits}
							{:else}
								0
							{/if}
						</TableBodyCell>
						<TableBodyCell>
							{#if note.status == ReleveNoteUnitStatus.M_AJOURNEE}
								Ajournee
							{:else if note.status == ReleveNoteUnitStatus.M_COMPENSEE}
								Compensee
							{:else if note.status == ReleveNoteUnitStatus.M_VALID}
								Valide
							{/if}
						</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>
</div>
