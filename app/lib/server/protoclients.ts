import { ADMIN_BACK_ADDR, CLIENT_BACK_URL } from '$env/static/private';
import { GrpcTransport } from '@protobuf-ts/grpc-transport';
import { ChannelCredentials } from '@grpc/grpc-js';

export const adminClient = new GrpcTransport({
	host: ADMIN_BACK_ADDR,
	channelCredentials: ChannelCredentials.createInsecure()
});

export const clientClient = new GrpcTransport({
	host: CLIENT_BACK_URL,
	channelCredentials: ChannelCredentials.createInsecure()
});
