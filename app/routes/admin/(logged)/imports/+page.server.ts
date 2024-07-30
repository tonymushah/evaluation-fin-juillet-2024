import { ImportDataMessage, ImportType } from '$lib/protos/admin';
import { ImportsClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';
import type { Actions } from '@sveltejs/kit';

type Errors = {
	configuration?: string;
	notes?: string;
	channel?: string;
};

export const ssr = false;

export const actions = {
	async default({ request }) {
		const formData = await request.formData();
		const configuration = formData.get('configuration');
		const notes = formData.get('notes');
		const errors: Errors = {};
		const imports = new ImportsClient(adminClient);
		const channel = imports.import();
		if (configuration instanceof File) {
			if (configuration.size <= 0) {
				errors.configuration = 'Configuration input invalid';
			} else {
				try {
					const stream = configuration.stream();
					for await (const chunk of stream) {
						console.log('streaming');
						console.log(chunk);
						await channel.requests.send({
							iType: ImportType.IT_CONFIG,
							buf: chunk
						});
					}
				} catch (e) {
					errors.configuration = e.message ?? 'Errot on upload';
				}
			}
		} else {
			errors.configuration = 'Configuration input invalid';
		}
		if (notes instanceof File) {
			if (notes.size <= 0) {
				errors.notes = 'Notes input invalid';
			} else {
				try {
					const stream = notes.stream();
					for await (const chunk of stream) {
						await channel.requests.send({
							iType: ImportType.IT_NOTE,
							buf: chunk
						});
					}
				} catch (e) {
					errors.configuration = e.message ?? 'Errot on upload';
				}
			}
		} else {
			errors.notes = 'Notes input invalid';
		}
		try {
			await channel.requests.complete();
		} catch (e) {
			console.log(e);
			errors.channel = e.message ?? 'Channel error';
		}
		return {
			errors
		};
	}
} satisfies Actions;
