<script lang="ts">
	import { Button, P } from 'flowbite-svelte';
	import { ArrowLeftOutline } from 'flowbite-svelte-icons';
</script>

<Button color="purple" on:click={() => history.back()}>
	<div class="flex flex-row flex-nowrap items-center gap-2">
		<ArrowLeftOutline />
		Back to lists
	</div>
</Button>

<slot />
