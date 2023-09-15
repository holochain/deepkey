<script lang="ts">
	import { onMount } from 'svelte';

	import type { ActionHash, AgentPubKey, AppAgentClient } from '@holochain/client';
	import { decode, encode } from '@msgpack/msgpack';
	// import { getCookie, deleteCookie, setCookie } from 'svelte-cookie';
	import { Base64 } from 'js-base64';
	import KeyAltIcon from '~icons/iconoir/key-alt-remove';
	import KeyPlusIcon from '~icons/iconoir/key-alt-plus';
	import EditIcon from '~icons/iconoir/edit';
	import AgentIcon from '~icons/iconoir/laptop';

	import { DeepkeyClient, type KeyAnchor } from '../lib/deepkey-client';
	import { authorizeClient, setupHolochain } from '$lib/holochain-client';
	import InviteAgent from '../components/invite-agent.svelte';
	import RegisterKey from '../components/register-key.svelte';
	import Identicon from '../components/identicon.svelte';
	import CryptographicHash from '../components/cryptographicHash.svelte';

	let client: AppAgentClient | undefined;
	let deepkey: DeepkeyClient | undefined;
	let deepkeyAgentPubkey: AgentPubKey | undefined;
	let keysetRootAuthority: ActionHash | undefined;
	let keysetMembers: AgentPubKey[] = [];
	let keysetKeys: KeyAnchor[] = [];

	onMount(async () => {
		let app_role = 'deepkey';

		client = await setupHolochain();

		deepkey = new DeepkeyClient(client, app_role);

		keysetRootAuthority = await deepkey.keyset_authority();
		console.log('keysetRootAuthority', Base64.fromUint8Array(keysetRootAuthority));

		// const res2 = await deepkey.key_state(client.myPubKey);
		// console.log('res2', res2);

		const appInfo = await client.appInfo();
		// console.log('appInfo', appInfo);
		deepkeyAgentPubkey = appInfo.agent_pub_key;

		keysetMembers = await deepkey.query_keyset_members(keysetRootAuthority);
		keysetKeys = await deepkey.query_keyset_keys(keysetRootAuthority);
	});

	let visible: boolean = false;
</script>

<!-- Top section -->
<div class="bg-cover bg-center h-1/3" style="background-image: url('/deepkey-hero.jpg')">
	<div class="text-center pt-40 pb-20">
		<h1 class="text-4xl font-bold">
			<span class="gradient-heading">Deepkey Explorer</span>
		</h1>
	</div>
</div>

{#if visible}
	<aside class="alert variant-ghost m-5 bg-gradient-to-br variant-gradient-secondary-primary">
		<!-- Icon -->
		<KeyAltIcon class="h-12 w-12" />
		<!-- Message -->
		<div class="alert-message">
			<h3 class="h3">Key Revocation Request</h3>
			<p>You have received a request to revoke key 0x</p>
		</div>
		<!-- Actions -->
		<div class="alert-actions">
			<button type="button" class="btn variant-ghost-error">Sign Revocation</button>
			<button type="button" class="btn variant-ghost-surface">Delete Request</button>
		</div>
	</aside>
{/if}

{#if visible}
	<aside class="alert variant-ghost m-5 bg-gradient-to-br variant-gradient-secondary-primary">
		<!-- Icon -->
		<KeyPlusIcon class="h-12 w-12" />
		<!-- Message -->
		<div class="alert-message">
			<h3 class="h3">Device Invitation Received</h3>
			<p>You have received a request to join the Keyset of Root Agent 0x</p>
		</div>
		<!-- Actions -->
		<div class="alert-actions">
			<button type="button" class="btn variant-ghost-success">Accept Invitation</button>
			<button type="button" class="btn variant-ghost-surface">Delete Request</button>
		</div>
	</aside>
{/if}
<div class="card p-4 m-5">
	<!-- identicon on the left -->

	<div class="flex items-center gap-3">
		<h3 class="text-2xl font-bold">Keyset Root Hash</h3>
		{#if keysetRootAuthority}
			<CryptographicHash hash={keysetRootAuthority} />
		{/if}
	</div>
	<p>All devices managed under this keyset root are under the same key management rules.</p>

	<div class="flex items-center gap-3">
		<h1 class="text-2xl font-bold">This Device's Agent Key</h1>
		{#if deepkeyAgentPubkey}
			<CryptographicHash hash={deepkeyAgentPubkey} />
		{/if}
	</div>
</div>

<div class="card p-4 m-5">
	<h3 class="text-2xl mb-4">Devices in this Keyset</h3>
	<InviteAgent {deepkey} />

	<ul class="list flex flex-col mt-6">
		{#each keysetMembers as member}
			<li>
				<span> <AgentIcon class="h-6 w-6" /> </span>
				<p class="text-gray-350 text-lg">unnamed</p>
				<EditIcon />
				{#if member}
					<CryptographicHash hash={member} />
				{/if}

				{#if Base64.fromUint8Array(member) === Base64.fromUint8Array(deepkeyAgentPubkey ?? Uint8Array.from([]))}
					<span class="chip bg-gradient-to-br variant-gradient-secondary-tertiary"
						>This device's key</span
					>
				{/if}
			</li>
		{/each}
	</ul>
</div>

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
	<RegisterKey {deepkey} />
	<ul class="list flex flex-col mt-6">
		{#each keysetKeys as key}
			<li>
				<span> <AgentIcon class="h-6 w-6" /> </span>
				<p class="text-gray-500 text-lg">{Base64.fromUint8Array(key.bytes)}</p>
			</li>
		{/each}
	</ul>
</div>

<footer class="h-32 m-12" />
