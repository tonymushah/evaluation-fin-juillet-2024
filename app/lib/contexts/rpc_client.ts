import { GrpcWebFetchTransport, type GrpcWebOptions } from '@protobuf-ts/grpcweb-transport';
import { getContext, setContext } from 'svelte';

const CLIENT_KEY = 'GRPC-CLIENT-CONTEXT';

export function getGRPCClientContext(): GrpcWebFetchTransport {
	const context = getContext<GrpcWebFetchTransport | undefined>(CLIENT_KEY);
	if (context != undefined) {
		return context;
	} else {
		throw new Error(`${CLIENT_KEY} is not defined`);
	}
}

export function setGRPCClientContext(client: GrpcWebFetchTransport): GrpcWebFetchTransport {
	return setContext(CLIENT_KEY, client);
}

export function initGRPCClientContext(options: GrpcWebOptions): GrpcWebFetchTransport {
	return setGRPCClientContext(new GrpcWebFetchTransport(options));
}
