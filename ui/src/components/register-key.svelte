<script lang="ts">
	import type {
		DeepkeyClient,
		DeviceInviteAcceptance,
		KeyGeneration,
		KeyRegistration
	} from '$lib/deepkey-client';
	import { deepkey } from '$lib/store/deepkey-client-store';
	import type { ActionHash, AgentPubKey } from '@holochain/client';
	import { encode, decode } from '@msgpack/msgpack';
	import { Base64 } from 'js-base64';

	import RegisterKeyIcon from '~icons/iconoir/key-alt-plus';

	let showRegisterInput = false;

	let pubKeyB64 = '';
	let diaPayload = '';

	let pastedDiaPayload = '';
	let acceptInvitationError: string = '';

	async function registerKey() {
		const key = Base64.toUint8Array(pubKeyB64);

		// TODO: This should be a real signature
		const signature = Uint8Array.from(new Array(64));

		const keyGeneration: KeyGeneration = {
			new_key: key,
			new_key_signing_of_author: signature
		};
		// create keyRegistration, with self-signed signature from private key
		const keyRegistration: KeyRegistration = { Create: keyGeneration };
		// register the keyRegistration
		const keyRegHash: ActionHash = await $deepkey.register_key(keyRegistration);
		console.log("Successfully registered key", keyRegHash);
	}
</script>

{#if showRegisterInput}
	<label class="label">
		<div>Paste in the agent's public key</div>
		<div class="flex flex-row space-x-2">
			<input
				class="input max-w-lg"
				bind:value={pubKeyB64}
				type=""
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
