<script lang="ts">
	import type { KeyState } from '$lib/deepkey-client';

	export let keyState: KeyState;
</script>

{#if 'Valid' in keyState}
	<span class="chip variant-filled-success my-auto">Valid</span>
{:else if 'Invalidated' in keyState}
	<span class="chip variant-filled-warning my-auto">Invalidated</span>
{:else}
	<span class="chip variant-filled-error my-auto">Not Found</span>
{/if}
