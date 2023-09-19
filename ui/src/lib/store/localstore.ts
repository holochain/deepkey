import { writable } from 'svelte/store';

export type Message = {
	type: 'device_invite_acceptance';
	base64: string;
};

export const messages = writable<Message[]>(JSON.parse(localStorage['messages'] ?? []));

messages.subscribe((value) => (localStorage.messages = JSON.stringify(value)));
