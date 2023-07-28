<script lang="ts">
	import { onMount } from 'svelte';
	import TreeView from 'svelte-tree-view';

	import type { ActionHash, AgentPubKey, AppAgentClient } from '@holochain/client';
	import { decode, encode } from '@msgpack/msgpack';
	// import { getCookie, deleteCookie, setCookie } from 'svelte-cookie';
	import { Base64 } from 'js-base64';
	import KeyAltIcon from '~icons/iconoir/key-alt-remove';
	import KeyPlusIcon from '~icons/iconoir/key-alt-plus';
	import AgentIcon from '~icons/iconoir/laptop';


	import { DeepkeyClient } from '../lib/deepkey-client';
	import { authorizeClient, setupHolochain } from '$lib/holochain-client';
	import InviteAgent from '../components/invite-agent.svelte';

	const data = {
		a: [1, 2, 3],
		b: new Map([
			['c', { d: null }],
			['e', { f: [9, 8, 7] }]
		])
	};

	let client: AppAgentClient | undefined;
	let deepkey: DeepkeyClient | undefined;
	let deepkeyAgentPubkey: AgentPubKey | undefined;
	let keysetRootAuthority: ActionHash | undefined;
	let keysetMembers: AgentPubKey[] = [];

	$: client;
	$: deepkey;
	$: keysetRootAuthority;
	$: deepkeyAgentPubkey;
	$: keysetMembers;

	onMount(async () => {
		let app_role = 'deepkey';

		client = await setupHolochain();

		deepkey = new DeepkeyClient(client, app_role);

		keysetRootAuthority = await deepkey.keyset_authority();

		// const res2 = await deepkey.key_state(client.myPubKey);
		// console.log('res2', res2);

		const appInfo = await client.appInfo();
		deepkeyAgentPubkey = appInfo.agent_pub_key;
		console.log('appInfo', appInfo);

		keysetMembers = await deepkey.query_keyset_members(keysetRootAuthority);
	});

	let visible: boolean = true;
	let message: string = 'This is a notification message';
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
			<p>You have received a request to revoke key 0x239828h2ff</p>
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
			<p>You have received a request to join the Keyset of Root Agent #298fFfA9</p>
		</div>
		<!-- Actions -->
		<div class="alert-actions">
			<button type="button" class="btn variant-ghost-success">Accept Invitation</button>
			<button type="button" class="btn variant-ghost-surface">Delete Request</button>
		</div>
	</aside>
{/if}
<div class="card p-4 m-5">
	<h1 class="text-2xl font-bold mb-2">Current Deepkey Agent Key</h1>
	<p class="text-gray-500 text-lg">
		{deepkeyAgentPubkey && Base64.fromUint8Array(deepkeyAgentPubkey)}
	</p>
	<h3 class="text-2xl font-bold mb-2 mt-5">Keyset Root Authority</h3>
	<p class="text-gray-500 text-lg">
		{keysetRootAuthority && Base64.fromUint8Array(keysetRootAuthority)}
	</p>
</div>

<div class="card p-4 m-5">
	<h3 class="text-2xl mb-4">Members of this Keyset</h3>
	<InviteAgent {deepkey} />

	<ul class="list flex flex-col mt-6">
		{#each keysetMembers as member}
			<li>
				<span> <AgentIcon class="h-6 w-6" /> </span>
				<p class="text-gray-500 text-lg">{Base64.fromUint8Array(member)}</p>
				<span class="chip bg-gradient-to-br variant-gradient-secondary-tertiary">Me</span
				>
			</li>
		{/each}
	</ul>
	
</div>

<main class="card p-4 m-5">
	<h3 class="text-2xl mb-4">Tree of Keys</h3>
	<TreeView
		{data}
		recursionOpts={{
			maxDepth: 16,
			shouldExpandNode: () => true
		}}
	/>
</main>

<footer class="h-32 m-12" />
