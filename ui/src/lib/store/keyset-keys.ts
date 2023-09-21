import { derived } from 'svelte/store';
import { deepkey } from './deepkey-client-store';
import { getKeyAnchor, getKeyFromKeyRegistration, type KeyAnchor } from '$lib/deepkey-client';

export const keysetKeys = derived(deepkey, async (deepkey) => {
	if (deepkey) {
		const keysetRootAuthority = await deepkey.keyset_authority();
		const keyAnchors = await deepkey.query_keyset_key_anchors(keysetRootAuthority);
		const keyRegistrations = await deepkey.query_keyset_keys(keysetRootAuthority);
		// console.log(keyAnchors[0].bytes);
		// console.log(getKeyAnchor(getKeyFromKeyRegistration(keyRegistrations[0]) ?? Uint8Array.from([])).bytes);

		return Promise.all(
			// keyAnchors.map(async (keyAnchor) => {
			keyRegistrations.map(async (keyRegistration) => {
				const key = getKeyFromKeyRegistration(keyRegistration);
				const keyAnchor = getKeyAnchor(key ?? Uint8Array.from([]));
				const keyState = await deepkey.key_state(Array.from(keyAnchor.bytes));
				return { keyBytes: keyAnchor.bytes, keyState };
			})
		);
	} else {
		return [];
	}
});
