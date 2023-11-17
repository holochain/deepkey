<script lang="ts">
	import AgentIcon from '~icons/iconoir/laptop';
	import KeyIcon from '~icons/iconoir/key-alt';
	import ConfigureIcon from '~icons/iconoir/tools';
	import { deepkey, keysetMembers } from '$lib/store/deepkey-client-store';
	import { keysetKeys, keysetKeysByAuthor } from '$lib/store/keyset-keys';
	import RegisterKey from '../components/register-key.svelte';
	import CryptographicHash from '../components/cryptographic-hash.svelte';
	import type { KeyState } from '$lib/deepkey-client';

	import { TreeView, TreeViewItem, type TreeViewNode } from '@skeletonlabs/skeleton';
	import EditableName from '../components/editable-name.svelte';
	import { indexableKey } from '$lib/util';
	import KeyStateBadge from '../components/key-state-badge.svelte';
	function keyStateText(keyState: any): string {
		if (keyState.Valid) {
			return 'Valid';
		} else if (keyState.Invalidated) {
			return 'Invalidated';
		} else {
			return 'Not Found';
		}
	}

	$: console.log($keysetKeysByAuthor);
	$: console.log($keysetKeys);

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

	{#await Promise.all([keysetMembers.load, keysetKeysByAuthor.load])}
		<p>Loading...</p>
	{:then}
		<TreeView open={true}>
			<!-- For Each Agent -->
			{#each $keysetMembers as member}
				<TreeViewItem>
					<svelte:fragment slot="lead"><AgentIcon class="h-8 w-8" /></svelte:fragment>
					<div class="flex gap-x-4">
						<CryptographicHash hash={member} />
						<span class="my-auto">
							<EditableName pubkey={member} />
						</span>
						<span class="chip variant-filled-surface my-auto">
							{$keysetKeysByAuthor[indexableKey(member)]?.length ?? 0} Key(s)
						</span>
					</div>
					<svelte:fragment slot="children">
						{#each $keysetKeysByAuthor[indexableKey(member)] ?? [] as keyInfo}
							<TreeViewItem>
								<svelte:fragment slot="lead"><KeyIcon /></svelte:fragment>
								<div class="flex gap-x-4">
									<CryptographicHash hash={keyInfo.keyBytes} size={24} />
									<KeyStateBadge keyState={keyInfo.keyState} />
								</div>
							</TreeViewItem>
						{/each}
					</svelte:fragment>
				</TreeViewItem>
			{/each}
		</TreeView>
	{/await}

	<!-- Flat View of All Keys -->
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
