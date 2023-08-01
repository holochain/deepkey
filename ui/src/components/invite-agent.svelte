<script lang="ts">
	import type { DeepkeyClient, DeviceInviteAcceptance } from '$lib/deepkey-client';
	import type { AgentPubKey } from '@holochain/client';
	import { encode, decode } from '@msgpack/msgpack';
	import { Base64 } from 'js-base64';
	import AcceptInviteIcon from '~icons/iconoir/send-mail';
	import AddAgentIcon from '~icons/iconoir/healthcare';

	export let deepkey: DeepkeyClient | undefined;
	let showInviteInput = false;
	let showAcceptInput = false;

	let agentKeyToInviteB64 = '';
	let diaPayload = '';

	let pastedDiaPayload = '';
	let acceptInvitationError: string = '';

	async function inviteAgent() {
		const agentKeyToInvite: AgentPubKey = Base64.toUint8Array(agentKeyToInviteB64);
		// TODO: Validate input here
		const dia = await deepkey?.invite_agent(agentKeyToInvite);
		diaPayload = Base64.fromUint8Array(encode(dia));
	}

	async function acceptInvitation() {
		try {
			const dia = decode(Base64.toUint8Array(pastedDiaPayload)) as DeviceInviteAcceptance;
			console.log('Decoded Device invite acceptance', dia);
			const diaHash = await deepkey?.accept_invitation(dia);
		} catch (e) {
			console.error(e);
			acceptInvitationError = (e as Error).message;
		}
		showInviteInput = false;
	}
</script>

{#if showInviteInput}
	<label class="label">
		<div>Enter the Deepkey Agent Key to invite into this Keyset</div>
		<div class="flex flex-row space-x-2">
			<input
				class="input max-w-lg"
				bind:value={agentKeyToInviteB64}
				type="text"
				title="Invite"
				placeholder="Input"
			/>
			<button type="button" class="btn variant-filled-primary" on:click={inviteAgent}>
				Invite
			</button>
			<button
				type="button"
				class="btn variant-filled-secondary"
				on:click={() => (showInviteInput = false)}
			>
				Cancel
			</button>
		</div>
		{#if diaPayload}
			<div class="card variant-ghost-secondary p-2 w-fit">
				{diaPayload}
			</div>
		{/if}
	</label>
{:else if showAcceptInput}
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
		class="btn btn-sm variant-ghost-primary"
		on:click={() => (showInviteInput = true)}
	>
		<span><AddAgentIcon class="h-6 w-6" /></span>
		<span>Invite a New Agent</span>
	</button>
	<button
		type="button"
		class="btn btn-sm variant-ghost-tertiary"
		on:click={() => (showAcceptInput = true)}
	>
		<span><AcceptInviteIcon class="h-6 w-6" /></span>
		<span>Accept an Invitation</span>
	</button>
{/if}
