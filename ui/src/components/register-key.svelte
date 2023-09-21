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

	let pubKeyB64 = 'uhCAknJ7xQSPhBZiy8mnEDrk07cqbcR27wIiVQ6bOAjB3KyxeeL77';
	let appName = 'asdffffff';
	let dnaHashB64 = 'uhC0kW585xNycWDHYPUj9Lw9a8xe5OxdPalhh5jNLta6Da1hJJlkU';

	let keyError = '';
	let dnaHashError = '';
	let appNameError = '';

	let acceptInvitationError: string = '';

	async function registerKey() {
		let key;
		try {
			console.log(pubKeyB64.replace(/[\-_]/g, ''));
			key = Base64.toUint8Array(pubKeyB64.substring(1));
		} catch (err) {
			console.error(err);
			keyError = err as string;
		}
		let dnaHash;
		try {
			dnaHash = Base64.toUint8Array(dnaHashB64.substring(1));
		} catch (err) {
			console.error(err);
			dnaHashError = err as string;
		}

		// TODO: This should be a real signature
		const signature = Uint8Array.from(new Array(64));

		if (key && dnaHash) {
			// register the keyRegistration
			const keyRegHash: ActionHash = await $deepkey.register_key(key, signature, dnaHash, appName);
			console.log('Successfully registered key', keyRegHash);
			showRegisterInput = false;
		}
	}
</script>

{#if showRegisterInput}
	<div class="mx-auto mt-10 w-1/3">
		<form class="bg-surface-400 shadow-md rounded px-8 pt-6 pb-8 mb-4">
			<div class="mb-4">
				<label class="block text-secondary-900 text-sm font-bold mb-1" for="public-key">
					Public Key
				</label>
				<input
					class="shadow appearance-none border rounded w-full py-2 px-3 text-surface-700 leading-tight focus:outline-none focus:shadow-outline"
					id="public-key"
					type="text"
					bind:value={pubKeyB64}
					placeholder="Public Key"
				/>
			</div>
			<div class="mb-5">
				<label class="block text-secondary-900 text-sm font-bold mb-1" for="app-name">
					App Name
				</label>
				<input
					class="shadow appearance-none border rounded w-full py-2 px-3 text-surface-700 leading-tight focus:outline-none focus:shadow-outline"
					id="app-name"
					bind:value={appName}
					type="text"
					placeholder="App Name"
				/>
			</div>
			<div class="mb-4">
				<label class="block text-secondary-900 text-sm font-bold mb-1" for="dna-hash">
					Dna Hash
				</label>
				<input
					class="shadow appearance-none border rounded w-full py-2 px-3 text-surface-700 leading-tight focus:outline-none focus:shadow-outline"
					id="dna-hash"
					bind:value={dnaHashB64}
					type="text"
					placeholder="Dna Hash"
				/>
			</div>
			<div class="flex items-center justify-between">
				<button type="button" class="btn variant-ghost-primary" on:click={registerKey}>
					Register
				</button>
				<button
					type="button"
					class="btn variant-ghost-error"
					on:click={() => (showRegisterInput = false)}
				>
					Cancel
				</button>
			</div>
		</form>
	</div>
{:else}
	<div class="container mx-auto">
		<button
			type="button"
			class="btn btn-sm variant-ghost-primary"
			on:click={() => (showRegisterInput = true)}
		>
			<span><RegisterKeyIcon class="h-6 w-6" /></span>
			<span>Register a new key</span>
		</button>
	</div>
{/if}
