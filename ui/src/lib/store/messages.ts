import { browser } from '$app/environment';
import type { DeviceInviteAcceptance } from '$lib/deepkey-client';
import { writable } from 'svelte/store';

export type Message = {
	id: string;
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

export function removeMessage(id: string) {
	messages.update((msgs) => {
		return msgs.filter((msg) => msg.id !== id);
	});
}

if (browser) {
	messages.subscribe((messages) => {
		const storageValue: string = JSON.stringify(messages);

		window.localStorage.setItem(storageKey, storageValue);
	});
}
