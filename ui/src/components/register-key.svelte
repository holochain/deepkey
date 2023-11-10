<script lang="ts">
	import { deepkey } from '$lib/store/deepkey-client-store';
	import type { ActionHash, AgentPubKey } from '@holochain/client';
	import { Base64 } from 'js-base64';

	import RegisterKeyIcon from '~icons/iconoir/key-alt-plus';
	import { getToastStore, type ToastSettings } from '@skeletonlabs/skeleton';

	const toastStore = getToastStore();

	let showRegisterInput = false;

	let pubKeyB64 = '';
	let appName = '';
	let dnaHashB64 = '';

	pubKeyB64 = 'uhCAknJ7xQSPhBZiy8mnEDrk07cqbcR27wIiVQ6bOAjB3KyxeeL77';
	appName = 'Holochain App!';
	dnaHashB64 = 'uhC0kW585xNycWDHYPUj9Lw9a8xe5OxdPalhh5jNLta6Da1hJJlkU';

	let keyError = '';
	let dnaHashError = '';
	let appNameError = '';

	let acceptInvitationError: string = '';

	async function registerKey() {
		let key;
		key = Base64.toUint8Array(pubKeyB64[0] === 'u' ? pubKeyB64.substring(1) : pubKeyB64);
		let dnaHash;
		dnaHash = Base64.toUint8Array(dnaHashB64[0] === 'u' ? dnaHashB64.substring(1) : dnaHashB64);

		// TODO: This should be a real signature
		const signature = Uint8Array.from(new Array(64));

		if (key && dnaHash) {
			// register the keyRegistration
			const keyRegHash: ActionHash = await $deepkey.registerKey(key, signature, dnaHash, appName);
			console.log('Successfully registered key', keyRegHash);

			const t: ToastSettings = {
				message: 'Successfully registered key.',
				hideDismiss: true,
				timeout: 3000
			};
			toastStore.trigger(t);

			showRegisterInput = false;
		}
	}
</script>

{#if showRegisterInput}
	<div class="mx-auto card-hover mt-4 w-1/3">
		<form class="bg-primary-400 shadow-md rounded px-8 pt-6 pb-8 mb-4">
			<div class="mb-4">
				<label class="block text-secondary-900 text-sm font-bold mb-1" for="public-key">
					Public Key
				</label>
				<input
					class="input py-2 px-3"
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
					class="input py-2 px-3"
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
					class="input py-2 px-3"
					id="dna-hash"
					bind:value={dnaHashB64}
					type="text"
					placeholder="Dna Hash"
				/>
			</div>
			<div class="flex items-center justify-between">
				<button type="button" class="btn variant-filled-secondary" on:click={registerKey}>
					Register
				</button>
				<button
					type="button"
					class="btn variant-filled-error"
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
