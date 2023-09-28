<script lang="ts">
	import CryptographicHash from '../components/cryptographic-hash.svelte';
	import RevocationAlert from '../components/revocation-alert.svelte';
	import KeysetDevices from './keyset-devices.svelte';
	import { deepkey, keysetRoot } from '$lib/store/deepkey-client-store';
	import ManualInviteAcceptance from '../components/manual-invite-acceptance.svelte';
	import { messages } from '$lib/store/messages';
	import Invitations from '../components/invitations.svelte';
	import KeysetKeys from './keyset-keys.svelte';
	import { onMount } from 'svelte/internal';
	import { deepkeyAgentPubkey } from '$lib/store/holochain-client-store';

	// async function registerTestKey() {
	// 	const keyreg = await $deepkey.callZome('register_test_key', null);
	// 	console.log(keyreg);
	// }

	let showInvitationAlert: boolean = true;
	let visible: boolean = false;

	onMount(async () => {
	});
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
		{#await keysetRoot.load then $keysetRoot}
			<CryptographicHash hash={$keysetRoot} />
		{/await}
		<h3 class="text-2xl font-bold">Keyset Root Hash</h3>
	</div>
	<p>All devices managed under this keyset root are under the same key management rules.</p>

	<div class="flex items-center gap-3 mt-4">
		{#await deepkeyAgentPubkey.load then $deepkeyAgentPubkey}
			<CryptographicHash hash={$deepkeyAgentPubkey} />
		{/await}
		<h1 class="text-2xl">This Device's Deepkey Agent Key</h1>
	</div>
</div>

<KeysetDevices />

<KeysetKeys />

<div class="m-5">
	<ManualInviteAcceptance />
</div>

<footer class="h-32 m-12" />
