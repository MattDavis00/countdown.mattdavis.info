import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit()],
	preview: {
		port: 4173,
		strictPort: true
	}
};

export default config;
