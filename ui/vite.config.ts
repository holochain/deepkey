import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import Icons from 'unplugin-icons/vite';

export default defineConfig({
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	plugins: [sveltekit(), Icons({ compiler: 'svelte' }) as any],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
