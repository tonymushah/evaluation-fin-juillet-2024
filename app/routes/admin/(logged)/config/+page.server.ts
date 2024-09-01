import { GetConfigsMessageResponse } from '$lib/protos/admin';
import { CfgNoteServiceClient } from '$lib/protos/admin.client';
import { Empty } from '$lib/protos/commons';
import { adminClient } from '$lib/server/protoclients';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const cfg_client = new CfgNoteServiceClient(adminClient);
	const res: GetConfigsMessageResponse = GetConfigsMessageResponse.toJson(
		(await cfg_client.getConfigs(Empty)).response
	);
	return {
		list: res.configs
	};
};
