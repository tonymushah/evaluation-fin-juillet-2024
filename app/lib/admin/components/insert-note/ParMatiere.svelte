<script lang="ts">
	import {
		Button,
		ButtonGroup,
		Input,
		TabItem,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		TableSearch
	} from 'flowbite-svelte';
	import { PlusOutline, TrashBinSolid, UploadSolid } from 'flowbite-svelte-icons';
	import { derived, writable, type Writable } from 'svelte/store';
	import { v4 } from 'uuid';
	import ParEtudiantRow from './par-etudiant/ParEtudiantRow.svelte';
	const matiere = writable<string | undefined>();
	type InsertValue = {
		etudiant?: string;
		valeur?: number;
	};
	enum InsertStatus {
		Inserted,
		Pending,
		Error
	}
	const values = writable(new Map<string, InsertValue>());
	const status = writable(new Map<string, InsertStatus>());
	function clear() {
		values.update((vs) => {
			vs.clear();
			return vs;
		});
		status.update((ss) => {
			ss.clear();
			return ss;
		});
	}
	function addLine() {
		values.update((vs) => {
			vs.set(v4(), {});
			return vs;
		});
	}
	const lines = derived([values, status], ([$values, $status]) =>
		Array.from($values).map(([key, insert]) => {
			return {
				key,
				...insert,
				status: $status.get(key)
			};
		})
	);
	function remove(key: string) {
		values.update((vs) => {
			vs.delete(key);
			return vs;
		});
		status.update((ss) => {
			ss.delete(key);
			return ss;
		});
	}
	function bind_etudiant(key: string): Writable<string | undefined> {
		const read = derived(values, ($values) => $values.get(key)?.etudiant);
		return {
			subscribe(run, invalidate) {
				return read.subscribe(run, invalidate);
			},
			set(value) {
				values.update((vs) => {
					let val = vs.get(key) ?? {};
					val.etudiant = value;
					vs.set(key, val);
					return vs;
				});
			},
			update(updater) {
				values.update((vs) => {
					let val = vs.get(key) ?? {};
					val.etudiant = updater(val.etudiant);
					vs.set(key, val);
					return vs;
				});
			}
		};
	}
	function bind_valeur(key: string): Writable<number | undefined> {
		const read = derived(values, ($values) => $values.get(key)?.valeur);
		return {
			subscribe(run, invalidate) {
				return read.subscribe(run, invalidate);
			},
			set(value) {
				values.update((vs) => {
					let val = vs.get(key) ?? {};
					val.valeur = value;
					vs.set(key, val);
					return vs;
				});
			},
			update(updater) {
				values.update((vs) => {
					let val = vs.get(key) ?? {};
					val.valeur = updater(val.valeur);
					vs.set(key, val);
					return vs;
				});
			}
		};
	}
</script>

<TabItem title="Par matiere">
	<div class="grid gap-4">
		<ButtonGroup>
			<Button on:click={() => addLine()}>
				<PlusOutline />
				Ajouter un ligne
			</Button>
			<Button on:click={() => clear()}>
				<TrashBinSolid />
				Clear
			</Button>
			<Button color="green">
				<UploadSolid />
				Inserer
			</Button>
		</ButtonGroup>
		<TableSearch placeholder="Matiere" bind:inputValue={$matiere}>
			<TableHead>
				<TableHeadCell>Etudiant</TableHeadCell>
				<TableHeadCell>Note</TableHeadCell>
				<TableHeadCell>Status d'insertion</TableHeadCell>
				<TableHeadCell>Actions :3</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each $lines as line (line.key)}
					<TableBodyRow>
						<ParEtudiantRow valeur={bind_valeur(line.key)} matiere={bind_etudiant(line.key)} />
						<TableBodyCell>
							{#if line.status == InsertStatus.Pending}
								Pending
							{:else if line.status == InsertStatus.Error}
								Error
							{:else if line.status == InsertStatus.Inserted}
								Done
							{:else}
								NaN
							{/if}
						</TableBodyCell>
						<TableBodyCell>
							<Button on:click={() => remove(line.key)} color="red">Remove</Button>
						</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</TableSearch>
	</div>
</TabItem>
