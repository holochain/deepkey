import type { DeepkeyClient } from '$lib/deepkey-client';
import { writable } from 'svelte/store';

export const deepkey = writable<DeepkeyClient>();
