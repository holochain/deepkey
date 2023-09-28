import { browser } from '$app/environment';
import type { DeviceInviteAcceptance } from '$lib/deepkey-client';
import { writable } from 'svelte/store';
import { deepkey } from './deepkey-client-store';
import { nanoid } from 'nanoid';

export type Message = {
	id: string;
	type: 'device_invite_acceptance';
	bytes: Uint8Array;
};

const storageKey = 'messages';

// Messages:
function createMessages() {
	const { subscribe, set, update } = writable<Message[]>([]);

	if (browser) {
		// Updates to the Store write to LocalStorage
		subscribe((messages) => {
			const storageValue: string = JSON.stringify(messages);
			window.localStorage.setItem(storageKey, storageValue);
		});

		// Initialize by reading from LocalStorage
		const storedValue = window.localStorage.getItem(storageKey);
		if (storedValue !== 'undefined' && storedValue !== null) {
			set(JSON.parse(storedValue));
		}
	}

	return {
		subscribe,
		add(msg: Message) {
			update((messages) => [...messages, msg]);
		},
		remove(id: string) {
			update((msgs) => msgs.filter((msg) => msg.id !== id));
		}
	};
}

export const messages = createMessages();

deepkey.load.then($deepkey => {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	$deepkey.on((data: any) => {
		console.log(data);
		if (data.type === 'InvitationReceived') {
			const dia = data.device_invite_acceptance;
			const message: Message = { id: nanoid(), type: 'device_invite_acceptance', bytes: dia };
			messages.add(message);
		}
	});
});
