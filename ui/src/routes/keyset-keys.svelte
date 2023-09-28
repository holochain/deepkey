<script lang="ts">
	import AgentIcon from '~icons/iconoir/laptop';
	import { deepkey } from '$lib/store/deepkey-client-store';
	import { keysetKeys } from '$lib/store/keyset-keys';
	import RegisterKey from '../components/register-key.svelte';
	import CryptographicHash from '../components/cryptographic-hash.svelte';
	import type { KeyState } from '$lib/deepkey-client';

	function keyStateText(keyState: any): string {
		if (keyState.Valid) {
			return 'Valid';
		} else if (keyState.Invalidated) {
			return 'Invalidated';
		} else {
			return 'Not Found';
		}
	}

	// $: console.log($keysetKeys);

	// let localKeyInfo = asyncDerived(deepkey, async ($deepkey) => {
	// 	return await $deepkey.query_local_key_info();
	// });
</script>

<div class="card p-4 m-5">
	<h3 class="text-2xl mb-4">All Keys within this Keyset</h3>
	<!-- 
	generate random keypair
	derive new key from seed, derivation path 
	make a lair call, gen new keypair or provide derivation string
	export a seed bundle; save the seed. lair saves it as encrypted thing
	dual-encrypted: password, security questions
	https://github.com/holochain/lair/tree/main/crates/hc_seed_bundle
-->

	<RegisterKey />

	<ul class="list flex flex-col mt-6">
		{#await keysetKeys.load then}
			{#each $keysetKeys as key}
				<li>
					<span> <AgentIcon class="h-6 w-6" /> </span>
					<CryptographicHash hash={key.keyBytes} />
					<p>{keyStateText(key.keyState)}</p>
				</li>
			{/each}
		{/await}
	</ul>
</div>
