<script lang="ts">
	import EditIcon from '~icons/iconoir/edit';
	import CancelIcon from '~icons/iconoir/cancel';
	import SaveIcon from '~icons/iconoir/save-floppy-disk';
	import { TwelveDotsScaleRotate } from 'svelte-svg-spinners';
	import type { AgentPubKey } from '@holochain/client';
	import { onMount } from 'svelte';
	import { deepkey } from '$lib/store/deepkey-client-store';
	import { Base64 } from 'js-base64';

	export let pubkey: AgentPubKey;

	let name: string = 'unnamed';
	let savedName = name;
	let editing = false;
	let dirty = false;
	let saving = false;
	let canEdit = false;

	function toggleEdit() {
		editing = !editing;
		name = savedName;
	}

	async function save() {
		saving = true;
		await $deepkey?.name_device(name);
		saving = false;
		dirty = false;
		savedName = name;
		toggleEdit();
	}

	async function getName() {
		name = (await $deepkey?.get_device_name(pubkey)) ?? name;
	}

	onMount(async () => {
		getName();

		// TODO: appinfo should be stored on a shared sveltestore, so we don't keep querying in different components.
		const appInfo = await $deepkey.client.appInfo();
		const deepkeyAgentPubkey = appInfo.agent_pub_key;
		canEdit = Base64.fromUint8Array(deepkeyAgentPubkey) === Base64.fromUint8Array(pubkey);
	});
</script>

{#if editing}
	<input bind:value={name} on:input={() => (dirty = true)} class="input w-1/3" type="text" />

	{#if saving}
		<TwelveDotsScaleRotate />
	{/if}

	{#if dirty && !saving}
		<button on:click={save}>
			<SaveIcon />
		</button>
	{/if}

	<button on:click={toggleEdit}>
		<CancelIcon />
	</button>
{:else}
	<p class="text-gray-350 text-lg">{name}</p>
	{#if canEdit}
		<button on:click={toggleEdit}>
			<EditIcon />
		</button>
	{/if}
{/if}
