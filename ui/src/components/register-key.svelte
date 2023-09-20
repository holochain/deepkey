<script lang="ts">
	import type { DeepkeyClient, DeviceInviteAcceptance } from '$lib/deepkey-client';
	import { deepkey } from '$lib/store/deepkey-client-store';
	import type { AgentPubKey } from '@holochain/client';
	import { encode, decode } from '@msgpack/msgpack';
	import { Base64 } from 'js-base64';

	import RegisterKeyIcon from '~icons/iconoir/key-alt-plus';

	let showRegisterInput = false;

	let privateKeyToInvite = '';
	let diaPayload = '';

	let pastedDiaPayload = '';
	let acceptInvitationError: string = '';

	async function registerKey() {
		// const agentKeyToInvite: AgentPubKey = Base64.toUint8Array(privateKeyToInvite);
		privateKeyToInvite;
		// make a keypair / derive public key
		// create keyRegistration, with self-signed signature from private key
		// register the keyRegistration

		// TODO: Validate input here
		// const dia = await deepkey?.invite_agent(agentKeyToInvite);
		// diaPayload = Base64.fromUint8Array(encode(dia));
	}

</script>

{#if showRegisterInput}
	<label class="label">
		<div>Paste in the private key</div>
		<div class="flex flex-row space-x-2">
			<input
				class="input max-w-lg"
				bind:value={privateKeyToInvite}
				type="password"
				title="Register Key"
				placeholder="Input"
			/>
			<button type="button" class="btn variant-filled-primary" on:click={registerKey}>
				Register
			</button>
			<button
				type="button"
				class="btn variant-filled-secondary"
				on:click={() => (showRegisterInput = false)}
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
		on:click={() => (showRegisterInput = true)}
	>
		<span><RegisterKeyIcon class="h-6 w-6" /></span>
		<span>Register a new key</span>
	</button>

{/if}
