<script lang="ts">
	import renderIcon from '$lib/identicon/bishop';
	// import renderIcon from '$lib/identicon/discs';
	import { onMount } from 'svelte';

	export let bytes: Uint8Array;
	export let size = 32;

	let canvas: HTMLCanvasElement;

	onMount(() => {
		const seedString = String.fromCharCode.apply(null, Array.from(bytes));
		renderIcon({ size: size, gridSize: 10, seed: seedString }, canvas);
	});
</script>

<canvas width={size} height={size} bind:this={canvas} />
