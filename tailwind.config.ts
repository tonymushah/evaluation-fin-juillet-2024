import flowbitePlugin from 'flowbite/plugin';
import typo from '@tailwindcss/typography';

import type { Config } from 'tailwindcss';

export default {
	content: [
		'./app/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte-icons/**/*.{html,js,svelte,ts}'
	],
	darkMode: 'class',
	theme: {
		extend: {
			colors: {
				// flowbite-svelte
				primary: {
					50: '#FFF5F2',
					100: '#FFF1EE',
					200: '#FFE4DE',
					300: '#FFD5CC',
					400: '#FFBCAD',
					500: '#FE795D',
					600: '#EF562F',
					700: '#EB4F27',
					800: '#CC4522',
					900: '#A5371B'
				}
			}
		}
	},

	plugins: [flowbitePlugin, typo()],
	fontFamily: {
		mono: ["'Fira Mono'", 'monospace'],
		serif: ["'Inter Variable'", 'sans-serif'],
		sans: ["'Poppins'", 'sans-serif'],
		brand: ["'Freckle Face'", 'system-ui']
	}
} as Config;
