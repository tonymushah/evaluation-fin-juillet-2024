// @generated by protobuf-ts 2.9.4 with parameter server_none
// @generated from protobuf file "client.proto" (package "mg.tonymushah.evalfjuil24.client", syntax proto3)
// tslint:disable
import { Current } from "./client";
import type { Etudiant } from "./commons";
import { Releve } from "./client";
import type { ListeRatrapageResponse } from "./client";
import type { Empty } from "./commons";
import type { GetReleveResponse } from "./client";
import type { GetReleveRequest } from "./client";
import { Semestres } from "./client";
import type { SemestresListResponse } from "./client";
import type { SemestresListRequest } from "./client";
import { Auth } from "./client";
import type { LoginResponse } from "./client";
import type { LoginRequest } from "./client";
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { HelloService } from "./client";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { HelloResponse } from "./client";
import type { HelloRequest } from "./client";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.HelloService
 */
export interface IHelloServiceClient {
    /**
     * @generated from protobuf rpc: Say(mg.tonymushah.evalfjuil24.client.HelloRequest) returns (mg.tonymushah.evalfjuil24.client.HelloResponse);
     */
    say(input: HelloRequest, options?: RpcOptions): UnaryCall<HelloRequest, HelloResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.HelloService
 */
export class HelloServiceClient implements IHelloServiceClient, ServiceInfo {
    typeName = HelloService.typeName;
    methods = HelloService.methods;
    options = HelloService.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Say(mg.tonymushah.evalfjuil24.client.HelloRequest) returns (mg.tonymushah.evalfjuil24.client.HelloResponse);
     */
    say(input: HelloRequest, options?: RpcOptions): UnaryCall<HelloRequest, HelloResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<HelloRequest, HelloResponse>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.Auth
 */
export interface IAuthClient {
    /**
     * @generated from protobuf rpc: Login(mg.tonymushah.evalfjuil24.client.LoginRequest) returns (mg.tonymushah.evalfjuil24.client.LoginResponse);
     */
    login(input: LoginRequest, options?: RpcOptions): UnaryCall<LoginRequest, LoginResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.Auth
 */
export class AuthClient implements IAuthClient, ServiceInfo {
    typeName = Auth.typeName;
    methods = Auth.methods;
    options = Auth.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Login(mg.tonymushah.evalfjuil24.client.LoginRequest) returns (mg.tonymushah.evalfjuil24.client.LoginResponse);
     */
    login(input: LoginRequest, options?: RpcOptions): UnaryCall<LoginRequest, LoginResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<LoginRequest, LoginResponse>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.Semestres
 */
export interface ISemestresClient {
    /**
     * @generated from protobuf rpc: List(mg.tonymushah.evalfjuil24.client.SemestresListRequest) returns (mg.tonymushah.evalfjuil24.client.SemestresListResponse);
     */
    list(input: SemestresListRequest, options?: RpcOptions): UnaryCall<SemestresListRequest, SemestresListResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.Semestres
 */
export class SemestresClient implements ISemestresClient, ServiceInfo {
    typeName = Semestres.typeName;
    methods = Semestres.methods;
    options = Semestres.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: List(mg.tonymushah.evalfjuil24.client.SemestresListRequest) returns (mg.tonymushah.evalfjuil24.client.SemestresListResponse);
     */
    list(input: SemestresListRequest, options?: RpcOptions): UnaryCall<SemestresListRequest, SemestresListResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<SemestresListRequest, SemestresListResponse>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.Releve
 */
export interface IReleveClient {
    /**
     * @generated from protobuf rpc: Get(mg.tonymushah.evalfjuil24.client.GetReleveRequest) returns (mg.tonymushah.evalfjuil24.client.GetReleveResponse);
     */
    get(input: GetReleveRequest, options?: RpcOptions): UnaryCall<GetReleveRequest, GetReleveResponse>;
    /**
     * @generated from protobuf rpc: ListeRatrapage(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.client.ListeRatrapageResponse);
     */
    listeRatrapage(input: Empty, options?: RpcOptions): UnaryCall<Empty, ListeRatrapageResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.Releve
 */
export class ReleveClient implements IReleveClient, ServiceInfo {
    typeName = Releve.typeName;
    methods = Releve.methods;
    options = Releve.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Get(mg.tonymushah.evalfjuil24.client.GetReleveRequest) returns (mg.tonymushah.evalfjuil24.client.GetReleveResponse);
     */
    get(input: GetReleveRequest, options?: RpcOptions): UnaryCall<GetReleveRequest, GetReleveResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetReleveRequest, GetReleveResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: ListeRatrapage(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.client.ListeRatrapageResponse);
     */
    listeRatrapage(input: Empty, options?: RpcOptions): UnaryCall<Empty, ListeRatrapageResponse> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<Empty, ListeRatrapageResponse>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.Current
 */
export interface ICurrentClient {
    /**
     * @generated from protobuf rpc: Get(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.Etudiant);
     */
    get(input: Empty, options?: RpcOptions): UnaryCall<Empty, Etudiant>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.client.Current
 */
export class CurrentClient implements ICurrentClient, ServiceInfo {
    typeName = Current.typeName;
    methods = Current.methods;
    options = Current.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Get(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.Etudiant);
     */
    get(input: Empty, options?: RpcOptions): UnaryCall<Empty, Etudiant> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<Empty, Etudiant>("unary", this._transport, method, opt, input);
    }
}
