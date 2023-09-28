import { DeepkeyClient } from '$lib/deepkey-client';
import { writable } from 'svelte/store';
import { holochain } from './holochain-client-store';
import { asyncDerived } from './loadable';

const app_role = 'deepkey';

export const deepkey = asyncDerived([holochain.load], async ([$holochain]) => {
	const deepkey = new DeepkeyClient($holochain, app_role);
	return deepkey;
});

export const keysetRoot = asyncDerived([deepkey.load], async ([$deepkey]) => {
	return await $deepkey.queryKeysetRoot();
});

export const keysetMembers = asyncDerived(
	[deepkey.load, keysetRoot.load] as const,
	async ([$deepkey, $keysetRoot]) => {
		return await $deepkey.queryKeysetMembers($keysetRoot);
	}
);
