import { browser } from '$app/environment';
import { writable, type Writable } from 'svelte/store';

export type Message = {
	type: 'device_invite_acceptance';
	base64: string;
};

// Get value from localStorage if in browser and the value is stored, otherwise fallback
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function fromLocalStorage(storageKey: string, fallbackValue: any) {
	if (browser) {
		const storedValue = window.localStorage.getItem(storageKey);

		if (storedValue !== 'undefined' && storedValue !== null) {
			return typeof fallbackValue === 'object' ? JSON.parse(storedValue) : storedValue;
		}
	}

	return fallbackValue;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function toLocalStorage(store: Writable<any>, storageKey: string) {
	if (browser) {
		store.subscribe((value) => {
			const storageValue: string = typeof value === 'object' ? JSON.stringify(value) : value;

			window.localStorage.setItem(storageKey, storageValue);
		});
	}
}

export const messages = writable<Message[]>(fromLocalStorage('messages', []));

toLocalStorage(messages, 'messages');
