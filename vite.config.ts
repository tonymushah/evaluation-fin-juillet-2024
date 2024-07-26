import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { kitRoutes } from 'vite-plugin-kit-routes';
import type { KIT_ROUTES } from './app/lib/ROUTES';
import { spawn } from 'node:child_process';

export default defineConfig({
	plugins: [
		{
			name: 'mushah-proto-build',
			async buildStart(options) {
				return new Promise((res, rej) => {
					const proto = spawn('pnpm', ['proto:all']);
					proto.on('error', rej);
					proto.stdout.on('data', (e: Buffer) => {
						console.log(e.toString());
					});
					proto.stderr.on('data', (e) => {
						console.error(new Error(e));
					});
					proto.on('close', () => {
						res();
					});
				});
			}
		},
		sveltekit(),
		kitRoutes<KIT_ROUTES>({
			routes_path: 'app/routes',
			generated_file_path: 'app/lib/ROUTES.ts'
		})
	],
	server: {
		fs: {
			deny: ['src', 'target'],
			allow: ['./app/app.css']
		}
	},
	test: {
		include: ['app/**/*.{test,spec}.{js,ts}']
	},
	optimizeDeps: {
		exclude: ['@urql/svelte']
	}
});
