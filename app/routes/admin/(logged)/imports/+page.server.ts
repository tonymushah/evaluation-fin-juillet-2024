import type { Actions } from '@sveltejs/kit';

type Errors = {
	configuration?: string;
	notes?: string;
};

export const ssr = false;

export const actions = {
	async default({ request }) {
		const formData = await request.formData();
		const configuration = formData.get('configuration');
		const notes = formData.get('notes');
		const errors: Errors = {};
		if (configuration instanceof File) {
			if (configuration.size <= 0) {
				errors.configuration = 'Configuration input invalid';
			} else {
				// TODO
			}
		} else {
			errors.configuration = 'Configuration input invalid';
		}
		if (notes instanceof File) {
			if (notes.size <= 0) {
				errors.notes = 'Notes input invalid';
			} else {
				// TODO
			}
		} else {
			errors.notes = 'Notes input invalid';
		}
		return {
			errors
		};
	}
} satisfies Actions;
