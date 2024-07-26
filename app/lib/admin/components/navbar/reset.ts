import { Empty } from '$lib/protos/admin';
import { DatabaseClient } from '$lib/protos/admin.client';
import type { RpcTransport } from '@protobuf-ts/runtime-rpc';

export default async function resetDB(client: RpcTransport) {
	const db = new DatabaseClient(client);
	const res = await db.reset(Empty);
	return res;
}
