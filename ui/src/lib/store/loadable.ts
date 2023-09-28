import { writable, type Readable, derived } from 'svelte/store';

export type Loadable<T> = Readable<T> & { load: Promise<T> };
export type LateInitLoadable<T> = Loadable<T> & { init: () => Promise<T> };

// type Types = unknown[];
type Loadables = readonly Loadable<unknown>[];
// type Promises<L> = { [K in keyof L]: L extends Loadable<infer U> ? Promise<U> : never };
type Values<L> = {
	[K in keyof L]: L[K] extends Loadable<infer U> | LateInitLoadable<infer U> ? Awaited<U> : never;
};

export function asyncDerived<L extends Loadables, T>(
	deps: L,
	cb: (values: Values<L>) => Promise<T>
): Loadable<T> {
	// const d = derived()
	const { subscribe, set } = writable<T>();
	const load = new Promise<T>((resolve) => {
		const promises = deps.map((l) => l.load);
		Promise.all(promises).then((resolvedDeps) => {
			cb(resolvedDeps as Values<L>).then((value) => {
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
