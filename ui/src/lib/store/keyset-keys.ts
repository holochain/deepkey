import { deepkey, keysetRoot } from './deepkey-client-store';
import { getKeyAnchor, getKeyFromKeyRegistration } from '$lib/deepkey-client';
import { asyncDerived } from './loadable';

export const keyRegistrations = asyncDerived(
	[deepkey, keysetRoot] as const,
	async ([$deepkey, $keysetRoot]) => {
		return await $deepkey.queryKeysetKeys($keysetRoot);
	}
);

export const keysetKeys = asyncDerived(
	[deepkey, keyRegistrations] as const,
	async ([$deepkey, $keyRegistrations]) => {
		return Promise.all(
			$keyRegistrations.map(async (keyRegistration) => {
				const key = getKeyFromKeyRegistration(keyRegistration);
				const keyAnchor = getKeyAnchor(key ?? Uint8Array.from([]));
				// It requires an Array, not a Uint8Array, because the Rust needs a Vec<u8> rather than [u8; 32]
				const keyState = await $deepkey.keyState(Array.from(keyAnchor.bytes));
				return { keyBytes: keyAnchor.bytes, keyState };
			})
		);
	}
);
