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
	import { derived, get, writable, type Writable } from 'svelte/store';
	import { v4 } from 'uuid';
	import ParEtudiantRow from './par-etudiant/ParEtudiantRow.svelte';
	type InsertValue = {
		matiere?: string;
		valeur?: number;
	};
	enum InsertStatus {
		Inserted,
		Pending,
		Error
	}
	const etudiant = writable<string | undefined>();
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
	function pendAll() {
		status.update((m) => {
			Array.from(m.keys()).forEach((key) => m.set(key, InsertStatus.Pending));
			return m;
		});
	}
	function errorAll() {
		status.update((m) => {
			Array.from(m.keys()).forEach((key) => m.set(key, InsertStatus.Error));
			return m;
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
	function bind_matiere(key: string): Writable<string | undefined> {
		const read = derived(values, ($values) => $values.get(key)?.matiere);
		return {
			subscribe(run, invalidate) {
				return read.subscribe(run, invalidate);
			},
			set(value) {
				values.update((vs) => {
					let val = vs.get(key) ?? {};
					val.matiere = value;
					vs.set(key, val);
					return vs;
				});
			},
			update(updater) {
				values.update((vs) => {
					let val = vs.get(key) ?? {};
					val.matiere = updater(val.matiere);
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
	let isLoading = false;
	async function insert() {
		const etu = get(etudiant);
		const values_ = Object.fromEntries(get(values));
		if (!isLoading && etu != undefined) {
			isLoading = true;
			try {
				const req = new Request(`/admin/note-insert/etudiant/${etu}`, {
					body: JSON.stringify(values_),
					method: 'POST'
				});
				pendAll();
				const res = await fetch(req);
				const body: Record<string, string> = await res.json();
				const body_res = new Map(Object.entries(body));

				status.update((m) => {
					Array.from(m.keys()).forEach((key) => {
						const res = body_res.get(key);
						if (res) {
							m.set(key, InsertStatus.Inserted);
						} else {
							m.set(key, InsertStatus.Error);
						}
					});
					return m;
				});
			} catch (error) {
				errorAll();
				console.error(error);
			} finally {
				isLoading = false;
			}
		}
	}
</script>

<TabItem open title="Par etudiants">
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
			<Button
				color="green"
				disabled={isLoading}
				on:click={async () => {
					await insert();
				}}
			>
				<UploadSolid />
				Inserer
			</Button>
		</ButtonGroup>
		<TableSearch bind:inputValue={$etudiant} placeholder="Etudiant">
			<TableHead>
				<TableHeadCell>Matiere</TableHeadCell>
				<TableHeadCell>Note</TableHeadCell>
				<TableHeadCell>Status d'insertion</TableHeadCell>
				<TableHeadCell>Actions :3</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each $lines as line (line.key)}
					<TableBodyRow>
						<ParEtudiantRow valeur={bind_valeur(line.key)} matiere={bind_matiere(line.key)} />
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
