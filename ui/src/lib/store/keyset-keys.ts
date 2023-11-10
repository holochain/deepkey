import { deepkey, keysetRoot } from './deepkey-client-store';
import {
	getKeyAnchor,
	getKeyFromKeyRegistration,
	type KeyRegistration,
	type KeyState
} from '$lib/deepkey-client';
import { asyncDerived } from './loadable';
import { indexableKey } from '$lib/util';

export const keyRegistrations = asyncDerived(
	[deepkey, keysetRoot] as const,
	async ([$deepkey, $keysetRoot]) => {
		return await $deepkey.queryKeysetKeys($keysetRoot);
	}
);

// Gets all keyRegistrations and queries the key state for each
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

export type KeyInfo = {
	keyBytes: Uint8Array;
	keyState: KeyState;
};

export const keysetKeysByAuthor = asyncDerived(
	[deepkey, keysetRoot] as const,
	async ([$deepkey, $keysetRoot]) => {
		const keysetKeysAndAuthors = await $deepkey.queryKeysetKeysWithAuthors($keysetRoot);
		const keys: Record<string, KeyInfo[]> = {};

		for (let i = 0; i < keysetKeysAndAuthors.length; i++) {
			const [authorKey, keyRegistration] = keysetKeysAndAuthors[i];
			const authorIndex = indexableKey(authorKey);

			const key = getKeyFromKeyRegistration(keyRegistration);
			const keyAnchor = getKeyAnchor(key ?? Uint8Array.from([]));
			// It requires an Array, not a Uint8Array, because the Rust needs a Vec<u8> rather than [u8; 32]
			const keyState = await $deepkey.keyState(Array.from(keyAnchor.bytes));

			keys[authorIndex] = [{ keyBytes: keyAnchor.bytes, keyState }, ...(keys[authorIndex] ?? [])];
		}

		return keys;
	}
);
