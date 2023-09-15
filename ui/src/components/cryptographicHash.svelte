<script lang="ts">
	import { clipboard } from '@skeletonlabs/skeleton';

	import CopyIcon from '~icons/iconoir/copy';

	import { Base64 } from 'js-base64';
	import Identicon from './identicon.svelte';
	export let hash: Uint8Array;
	const base64Hash = Base64.fromUint8Array(hash);

	let copiedConfirm = false;

	function showCopied() {
		copiedConfirm = true;
		setTimeout(() => (copiedConfirm = false), 700);
	}
</script>

<div class="group relative inline-block">
	<p class="p-1 variant-ringed-primary">
		<Identicon bytes={hash} />
	</p>

	<div class="absolute z-10 hidden group-hover:block transform translate-x-6 -translate-y-5">
		<aside class="alert bg-gradient-to-br variant-gradient-primary-secondary rounded-sm">
			<!-- Icon -->
			{#if copiedConfirm}
				<button>👍</button>
			{:else}
				<button use:clipboard={base64Hash} on:click={showCopied}><CopyIcon /></button>
			{/if}
			<!-- Message -->
			<div class="alert-message">
				{Base64.fromUint8Array(hash)}
			</div>
		</aside>
	</div>
</div>
