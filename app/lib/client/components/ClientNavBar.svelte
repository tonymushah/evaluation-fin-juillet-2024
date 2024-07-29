<script lang="ts">
	import { route } from '$lib/ROUTES';

	import { DarkMode, Navbar, NavBrand, NavHamburger, NavLi, NavUl } from 'flowbite-svelte';
</script>

<Navbar let:NavContainer color="primary">
	<NavContainer class="rounded-lg border bg-white px-5 py-2 dark:bg-gray-600">
		<NavBrand href={route('/client')}>
			<span class="self-center whitespace-nowrap text-xl font-semibold">Etudiant</span>
		</NavBrand>
		<NavHamburger />
		<NavUl>
			<NavLi>Se deconnecter</NavLi>
		</NavUl>
		<div>
			<DarkMode size="md" />
		</div>
	</NavContainer>
</Navbar>
