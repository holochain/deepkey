<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type { ActionHash, AgentPubKey, AppAgentClient } from '@holochain/client';
	import type { UnsubscribeFunction } from 'emittery';
	import { Base64 } from 'js-base64';
	import AgentIcon from '~icons/iconoir/laptop';
	import { setupHolochain } from '$lib/holochain-client';
	import { DeepkeyClient, type KeyAnchor } from '../lib/deepkey-client';
	import RegisterKey from '../components/register-key.svelte';
	import CryptographicHash from '../components/cryptographic-hash.svelte';
	import RevocationAlert from '../components/revocation-alert.svelte';
	import InvitationAlert from '../components/invitation-alert.svelte';
	import KeysetDevices from './keyset-devices.svelte';
	import { deepkey } from '$lib/store/deepkey-client-store';
	import ManualInviteAcceptance from '../components/manual-invite-acceptance.svelte';
	import { messages } from '$lib/store/messages';
	import { decode } from '@msgpack/msgpack';
	import Invitations from '../components/invitations.svelte';

	let client: AppAgentClient | undefined;
	let deepkeyClient: DeepkeyClient | undefined;
	let deepkeyAgentPubkey: AgentPubKey | undefined;
	let keysetRootAuthority: ActionHash | undefined;
	let keysetKeys: KeyAnchor[] = [];
	let unsubscribe: UnsubscribeFunction | undefined;

	onMount(async () => {
		let app_role = 'deepkey';

		// TODO: Maybe this init could be done in the $deepkey store.
		client = await setupHolochain();
		deepkeyClient = new DeepkeyClient(client, app_role);
		$deepkey = deepkeyClient;

		unsubscribe = deepkeyClient.on((data: any) => {
			// console.log(data);
			if (data.type === 'InvitationReceived') {
				const dia = data.device_invite_acceptance;
				$messages = [...$messages, { type: 'device_invite_acceptance', bytes: dia }];
			}
		});

		keysetRootAuthority = await deepkeyClient.keyset_authority();
		// console.log('keysetRootAuthority', Base64.fromUint8Array(keysetRootAuthority));

		// const res2 = await deepkey.key_state(client.myPubKey);
		// console.log('res2', res2);

		const appInfo = await client.appInfo();
		deepkeyAgentPubkey = appInfo.agent_pub_key;

		keysetKeys = await deepkeyClient.query_keyset_keys(keysetRootAuthority);
	});

	onDestroy(async () => {
		unsubscribe && unsubscribe();
	});

	let showInvitationAlert: boolean = true;
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
	<RevocationAlert />
{/if}

{#if showInvitationAlert}
	<Invitations />
{/if}

<div class="card p-4 m-5">
	<!-- identicon on the left -->

	<div class="flex items-center gap-3">
		{#if keysetRootAuthority}
			<CryptographicHash hash={keysetRootAuthority} />
		{/if}
		<h3 class="text-2xl font-bold">Keyset Root Hash</h3>
	</div>
	<p>All devices managed under this keyset root are under the same key management rules.</p>

	<div class="flex items-center gap-3 mt-4">
		{#if deepkeyAgentPubkey}
			<CryptographicHash hash={deepkeyAgentPubkey} />
		{/if}
		<h1 class="text-2xl">This Device's Deepkey Agent Key</h1>
	</div>
</div>

<KeysetDevices />

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
	<RegisterKey deepkey={deepkeyClient} />
	<ul class="list flex flex-col mt-6">
		{#each keysetKeys as key}
			<li>
				<span> <AgentIcon class="h-6 w-6" /> </span>
				<p class="text-gray-500 text-lg">{Base64.fromUint8Array(key.bytes)}</p>
			</li>
		{/each}
	</ul>
</div>

<div class="m-5">
	<ManualInviteAcceptance />
</div>
<footer class="h-32 m-12" />
