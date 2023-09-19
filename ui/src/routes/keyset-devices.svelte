<script lang="ts">
	import { Base64 } from 'js-base64';
	import type { ActionHash, AgentPubKey } from '@holochain/client';
	import AgentIcon from '~icons/iconoir/laptop';
	import InviteAgent from '../components/invite-agent.svelte';
	import { deepkey } from '$lib/store/deepkey-client-store';
	import EditableName from '../components/editable-name.svelte';
	import CryptographicHash from '../components/cryptographic-hash.svelte';

	let deepkeyAgentPubkey: AgentPubKey | undefined;
	let keysetMembers: AgentPubKey[] = [];
	let keysetRootAuthority: ActionHash | undefined;

	deepkey.subscribe(async (deepkeyClient) => {
		if (!$deepkey) return;
		const appInfo = await $deepkey.client.appInfo();
		deepkeyAgentPubkey = appInfo.agent_pub_key;
		keysetRootAuthority = await $deepkey.keyset_authority();
		keysetMembers = await $deepkey.query_keyset_members(keysetRootAuthority);
	});
</script>

<div class="card p-4 m-5">
	<h3 class="text-2xl mb-4">Devices in this Keyset</h3>
	<InviteAgent />

	<ul class="list flex flex-col mt-6">
		{#each keysetMembers as member}
			<li>
				<span> <AgentIcon class="h-6 w-6" /> </span>
				{#if member}
					<CryptographicHash hash={member} />
				{/if}
				<EditableName pubkey={member} />

				{#if Base64.fromUint8Array(member) === Base64.fromUint8Array(deepkeyAgentPubkey ?? Uint8Array.from([]))}
					<span class="chip bg-gradient-to-br variant-gradient-secondary-tertiary">
						This Device's Deepkey Agent Key
					</span>
				{/if}
			</li>
		{/each}
	</ul>
</div>
