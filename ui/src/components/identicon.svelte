<script lang="ts">
	// import renderIcon from '$lib/identicon/bishop';
	import renderIcon from '$lib/identicon/discs';
	import { onMount } from 'svelte';

	export let bytes: Uint8Array;

	let canvas: HTMLCanvasElement;

	onMount(() => {
		const seedString = String.fromCharCode.apply(null, Array.from(bytes));
		renderIcon({ size: 50, gridSize: 10, seed: seedString }, canvas);
	});
</script>

<canvas width={1} height={1} bind:this={canvas} />
