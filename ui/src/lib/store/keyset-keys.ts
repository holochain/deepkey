import { derived } from 'svelte/store';
import { deepkey } from './deepkey-client-store';

export const keysetKeys = derived(deepkey, async (deepkey) => {
	if (deepkey) {
		const keysetRootAuthority = await deepkey.keyset_authority();

		const keys = await deepkey.query_keyset_keys(keysetRootAuthority);
    return Promise.all(keys.map(async (key) => {
      const keyState = await deepkey.key_state(key.bytes);
      return { keyBytes: Uint8Array.from(key.bytes), keyState }
    }));
	} else {
		return [];
	}
});
