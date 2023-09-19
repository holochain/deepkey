<script lang="ts">
	import EditIcon from '~icons/iconoir/edit';
	import CancelIcon from '~icons/iconoir/cancel';
	import SaveIcon from '~icons/iconoir/save-floppy-disk';
	import { TwelveDotsScaleRotate } from 'svelte-svg-spinners';
	import type { AgentPubKey } from '@holochain/client';
	import { onMount } from 'svelte';
	import { deepkey } from '$lib/store/deepkey-client-store';

	export let pubkey: AgentPubKey;

	let name: string = 'unnamed';
	let savedName = name;
	let editing = false;
	let dirty = false;
	let saving = false;

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

	onMount(() => {
		getName();
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
	<button on:click={toggleEdit}>
		<EditIcon />
	</button>
{/if}
