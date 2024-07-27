<script lang="ts">
	import { Label, Input, Button } from 'flowbite-svelte';
</script>

<div class="flex h-screen w-screen items-center justify-center">
	<div class="grid h-2/3 w-2/3 grid-cols-2 rounded-md bg-slate-300 p-4 dark:bg-slate-700">
		<div class="flex h-full w-full items-center justify-center">
			<h1 class="text-3xl font-extrabold">Eval</h1>
		</div>
		<div class="flex h-full w-full flex-col items-center justify-center gap-5">
			<h2 class="text-2xl font-bold dark:text-white">Se Connecter</h2>
			<form method="post" class=" flex w-11/12 flex-col gap-5">
				<div>
					<Label class="mb-2" for="nom_user">Nom d'utilisateur</Label>
					<Input name="nom" type="text" id="nom_user" placeholder="Rkoto" required />
				</div>
				<div>
					<Label class="mb-2" for="password">Mot de passe</Label>
					<Input name="password" type="text" id="password" placeholder="Password" required />
				</div>
				<Button type="submit">Se Connecter</Button>
			</form>
		</div>
	</div>
</div>
