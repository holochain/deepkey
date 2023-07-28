<script lang="ts">
	import type { DeepkeyClient } from '$lib/deepkey-client';
	import type { AgentPubKey } from '@holochain/client';
	import { encode } from '@msgpack/msgpack';
	import { Base64 } from 'js-base64';
	import AddAgentIcon from '~icons/iconoir/healthcare';

	export let deepkey: DeepkeyClient | undefined;
	let showInviteInput = true;
	let agentKeyToInviteB64 = '';
	let diaPayload = '';

	async function inviteAgent() {
		const agentKeyToInvite: AgentPubKey = Base64.toUint8Array(agentKeyToInviteB64);
		// TODO: Validate input here
		const dia = await deepkey?.invite_agent(agentKeyToInvite);
		const diaPayload = Base64.fromUint8Array(encode(dia));
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
{:else}
	<button
		type="button"
		class="btn btn-sm variant-ghost-primary"
		on:click={() => (showInviteInput = true)}
	>
		<span><AddAgentIcon class="h-6 w-6" /></span>
		<span>Invite a New Agent</span>
	</button>
{/if}
