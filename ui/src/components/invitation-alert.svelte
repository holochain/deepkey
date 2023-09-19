<script lang="ts">
	import { messages, type Message } from '$lib/store/messages';
	import { decode } from '@msgpack/msgpack';
	import { Base64 } from 'js-base64';
	import KeyPlusIcon from '~icons/iconoir/key-alt-plus';
	import CryptographicHash from './cryptographic-hash.svelte';
	import { derived, type Readable, type Writable } from 'svelte/store';
	import type { DeviceInviteAcceptance } from '$lib/deepkey-client';
	import { deepkey } from '$lib/store/deepkey-client-store';

	export let invite: DeviceInviteAcceptance;

	async function acceptInvitation() {
		try {
			const diaHash = await $deepkey?.accept_invitation(invite);
			console.log("successfully accepted invitation", diaHash);
			// TODO: show success!
			// remove Invitation from Messages
		} catch (e) {
			console.error(e);
			// acceptInvitationError = (e as Error).message;
		}
		// showAcceptInput = false;
	}
</script>

<aside class="alert variant-ghost m-5 bg-gradient-to-br variant-gradient-secondary-primary">
	<!-- Icon -->
	<KeyPlusIcon class="h-12 w-12" />
	<!-- Message -->
	<div class="alert-message">
		<div class="flex items-center gap-3">
			<CryptographicHash hash={invite.keyset_root_authority} />
			<div class="flex flex-col">
				<h3 class="h3">Device Invitation Received</h3>
				<p>You have received a request to join a Keyset.</p>
			</div>
		</div>
	</div>
	<!-- Actions -->
	<div class="alert-actions">
		<button type="button" on:click={acceptInvitation} class="btn variant-ghost-success">Accept Invitation</button>
		<button type="button" class="btn variant-ghost-surface">Delete Request</button>
	</div>
</aside>
