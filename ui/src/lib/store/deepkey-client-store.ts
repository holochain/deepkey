import { DeepkeyClient } from '$lib/deepkey-client';
import { cellId, holochain } from './holochain-client-store';
import { asyncDerived } from './loadable';

const app_role = 'deepkey';

export const deepkey = asyncDerived([holochain, cellId] as const, async ([$holochain, $cellId]) => {
	const deepkey = new DeepkeyClient($holochain, $cellId, app_role);
	return deepkey;
});

export const keysetRoot = asyncDerived([deepkey], async ([$deepkey]) => {
	return await $deepkey.queryKeysetRoot();
});

export const keysetMembers = asyncDerived(
	[deepkey, keysetRoot] as const,
	async ([$deepkey, $keysetRoot]) => {
		return await $deepkey.queryKeysetMembers($keysetRoot);
	}
);
