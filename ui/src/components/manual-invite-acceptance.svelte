<script lang="ts">
	import type { DeviceInviteAcceptance } from '$lib/deepkey-client';
	import { deepkey } from '$lib/store/deepkey-client-store';
	import { encode, decode } from '@msgpack/msgpack';
	import { Base64 } from 'js-base64';
	import AcceptInviteIcon from '~icons/iconoir/send-mail';

	let showAcceptInput = false;

	let pastedDiaPayload = '';
	let acceptInvitationError: string = '';

	async function acceptInvitation() {
		try {
			const dia = decode(Base64.toUint8Array(pastedDiaPayload)) as DeviceInviteAcceptance;
			console.log('Decoded Device invite acceptance', dia);
			const diaHash = await $deepkey?.accept_invitation(dia);
      // TODO: Could show this in a
		} catch (e) {
			console.error(e);
			acceptInvitationError = (e as Error).message;
		}
		showAcceptInput = false;
	}
</script>

{#if showAcceptInput}
	<label class="label">
		<div>Paste in the invitation here</div>
		<div class="flex flex-row space-x-2">
			<input
				class="input max-w-lg"
				bind:value={pastedDiaPayload}
				type="text"
				title="Invite"
				placeholder="Device Invite Acceptance Payload"
			/>
			<button type="button" class="btn variant-filled-primary" on:click={acceptInvitation}>
				Accept
			</button>
			{#if acceptInvitationError}
				<div class="text-error-50">{acceptInvitationError}</div>
			{/if}
			<button
				type="button"
				class="btn variant-filled-secondary"
				on:click={() => (showAcceptInput = false)}
			>
				Cancel
			</button>
		</div>
	</label>
{:else}
	<button
		type="button"
		class="btn btn-sm variant-ghost-tertiary"
		on:click={() => (showAcceptInput = true)}
	>
		<span><AcceptInviteIcon class="h-6 w-6" /></span>
		<span>Manually Accept an Invitation</span>
	</button>
{/if}
