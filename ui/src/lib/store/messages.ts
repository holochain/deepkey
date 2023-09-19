import { browser } from '$app/environment';
import type { DeviceInviteAcceptance } from '$lib/deepkey-client';
import { writable } from 'svelte/store';

export type Message = {
	type: 'device_invite_acceptance';
	bytes: Uint8Array;
};

const storageKey = 'messages';

export const messages = writable<Message[]>(
	((): Message[] => {
		if (browser) {
			const storedValue = window.localStorage.getItem(storageKey);

			if (storedValue !== 'undefined' && storedValue !== null) {
				return JSON.parse(storedValue);
			}
		}

		return [];
	})()
);

if (browser) {
	messages.subscribe((messages) => {
		const storageValue: string = JSON.stringify(messages);

		window.localStorage.setItem(storageKey, storageValue);
	});
}
