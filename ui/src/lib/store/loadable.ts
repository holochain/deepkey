import { writable, type Readable } from 'svelte/store';

export type Loadable<T> = Readable<T> & { load: Promise<T> };
export type LateInitLoadable<T> = Loadable<T> & { init: () => Promise<T> };

export function asyncDerived<S extends readonly unknown[], T>(
	deps: S,
	cb: (values: { [K in keyof S]: Awaited<S[K]> }) => Promise<T>
): Loadable<T> {
	const { subscribe, set } = writable<T>();
	const load = new Promise<T>((resolve) => {
		Promise.all(deps).then((resolvedDeps) => {
			cb(resolvedDeps).then((value) => {
				resolve(value);
				set(value);
			});
		});
	});

	return {
		subscribe,
		load
	};
}

export function lateInitLoadable<T>(lateInitFn: () => Promise<T>): LateInitLoadable<T> {
	const { subscribe, set } = writable<T>();
	// eslint-disable-next-line @typescript-eslint/no-empty-function, @typescript-eslint/no-unused-vars
	let loadResolver: (value: T) => void = (_: T) => {};
	const load = new Promise<T>((resolve) => {
		loadResolver = resolve;
	});

	return {
		subscribe,
		async init() {
			const value = await lateInitFn();
			set(value as T);
			loadResolver(value as T);
			return value as T;
		},
		load
	};
}
