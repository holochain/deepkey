<script lang="ts">
	import EditIcon from '~icons/iconoir/edit';
	import CancelIcon from '~icons/iconoir/cancel';
	import SaveIcon from '~icons/iconoir/save-floppy-disk';
	import type { DeepkeyClient } from '$lib/deepkey-client';
	import { TwelveDotsScaleRotate } from 'svelte-svg-spinners';

	export let name: string = 'unnamed';
	export let deepkey: DeepkeyClient | undefined;

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
		await deepkey?.name_device(name);
		saving = false;
		dirty = false;
		savedName = name;
		toggleEdit();
	}
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
