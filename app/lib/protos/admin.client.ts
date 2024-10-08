// @generated by protobuf-ts 2.9.4 with parameter server_none
// @generated from protobuf file "admin.proto" (package "mg.tonymushah.evalfjuil24.admin", syntax proto3)
// tslint:disable
import { SemetresServ } from "./admin";
import type { EtudiantRangsResponse } from "./admin";
import type { EtudiantRangsRequest } from "./admin";
import { CfgNoteService } from "./admin";
import type { ConfigNote } from "./admin";
import type { GetConfigsMessageResponse } from "./admin";
import { Imports } from "./admin";
import type { ImportDataMessage } from "./admin";
import type { ClientStreamingCall } from "@protobuf-ts/runtime-rpc";
import { Auth } from "./admin";
import type { LoginResponse } from "./admin";
import type { LoginRequest } from "./admin";
import { Getters } from "./admin";
import type { EtudiantAdmisOuNonResponse } from "./admin";
import type { GetPromotionsResponse } from "./admin";
import type { GetPromotionsRequest } from "./admin";
import type { GetMatieresResponse } from "./admin";
import type { GetMatieresRequest } from "./admin";
import type { GetSemetresResponse } from "./admin";
import type { GetSemetresRequest } from "./admin";
import { Notes } from "./admin";
import type { InsertNotesParPromotionRequest } from "./admin";
import type { InsertNotesResponse } from "./admin";
import type { InsertNotesRequest } from "./admin";
import type { DuplexStreamingCall } from "@protobuf-ts/runtime-rpc";
import { Etudiants } from "./admin";
import type { EtudiantReleveNoteResponse } from "./admin";
import type { EtudiantReleveNoteRequest } from "./admin";
import type { EtudiantInfoResponse } from "./admin";
import type { EtudiantInfoRequest } from "./admin";
import type { EtudiantsListResponse } from "./admin";
import type { EtudiantsListRequest } from "./admin";
import { HelloService } from "./admin";
import type { HelloResponse } from "./admin";
import type { HelloRequest } from "./admin";
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { Database } from "./admin";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { Empty } from "./commons";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Database
 */
export interface IDatabaseClient {
    /**
     * @generated from protobuf rpc: Reset(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.Empty);
     */
    reset(input: Empty, options?: RpcOptions): UnaryCall<Empty, Empty>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Database
 */
export class DatabaseClient implements IDatabaseClient, ServiceInfo {
    typeName = Database.typeName;
    methods = Database.methods;
    options = Database.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Reset(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.Empty);
     */
    reset(input: Empty, options?: RpcOptions): UnaryCall<Empty, Empty> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<Empty, Empty>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.HelloService
 */
export interface IHelloServiceClient {
    /**
     * @generated from protobuf rpc: Say(mg.tonymushah.evalfjuil24.admin.HelloRequest) returns (mg.tonymushah.evalfjuil24.admin.HelloResponse);
     */
    say(input: HelloRequest, options?: RpcOptions): UnaryCall<HelloRequest, HelloResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.HelloService
 */
export class HelloServiceClient implements IHelloServiceClient, ServiceInfo {
    typeName = HelloService.typeName;
    methods = HelloService.methods;
    options = HelloService.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Say(mg.tonymushah.evalfjuil24.admin.HelloRequest) returns (mg.tonymushah.evalfjuil24.admin.HelloResponse);
     */
    say(input: HelloRequest, options?: RpcOptions): UnaryCall<HelloRequest, HelloResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<HelloRequest, HelloResponse>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Etudiants
 */
export interface IEtudiantsClient {
    /**
     * @generated from protobuf rpc: List(mg.tonymushah.evalfjuil24.admin.EtudiantsListRequest) returns (mg.tonymushah.evalfjuil24.admin.EtudiantsListResponse);
     */
    list(input: EtudiantsListRequest, options?: RpcOptions): UnaryCall<EtudiantsListRequest, EtudiantsListResponse>;
    /**
     * @generated from protobuf rpc: Info(mg.tonymushah.evalfjuil24.admin.EtudiantInfoRequest) returns (mg.tonymushah.evalfjuil24.admin.EtudiantInfoResponse);
     */
    info(input: EtudiantInfoRequest, options?: RpcOptions): UnaryCall<EtudiantInfoRequest, EtudiantInfoResponse>;
    /**
     * @generated from protobuf rpc: ReleveNote(mg.tonymushah.evalfjuil24.admin.EtudiantReleveNoteRequest) returns (mg.tonymushah.evalfjuil24.admin.EtudiantReleveNoteResponse);
     */
    releveNote(input: EtudiantReleveNoteRequest, options?: RpcOptions): UnaryCall<EtudiantReleveNoteRequest, EtudiantReleveNoteResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Etudiants
 */
export class EtudiantsClient implements IEtudiantsClient, ServiceInfo {
    typeName = Etudiants.typeName;
    methods = Etudiants.methods;
    options = Etudiants.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: List(mg.tonymushah.evalfjuil24.admin.EtudiantsListRequest) returns (mg.tonymushah.evalfjuil24.admin.EtudiantsListResponse);
     */
    list(input: EtudiantsListRequest, options?: RpcOptions): UnaryCall<EtudiantsListRequest, EtudiantsListResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<EtudiantsListRequest, EtudiantsListResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: Info(mg.tonymushah.evalfjuil24.admin.EtudiantInfoRequest) returns (mg.tonymushah.evalfjuil24.admin.EtudiantInfoResponse);
     */
    info(input: EtudiantInfoRequest, options?: RpcOptions): UnaryCall<EtudiantInfoRequest, EtudiantInfoResponse> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<EtudiantInfoRequest, EtudiantInfoResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: ReleveNote(mg.tonymushah.evalfjuil24.admin.EtudiantReleveNoteRequest) returns (mg.tonymushah.evalfjuil24.admin.EtudiantReleveNoteResponse);
     */
    releveNote(input: EtudiantReleveNoteRequest, options?: RpcOptions): UnaryCall<EtudiantReleveNoteRequest, EtudiantReleveNoteResponse> {
        const method = this.methods[2], opt = this._transport.mergeOptions(options);
        return stackIntercept<EtudiantReleveNoteRequest, EtudiantReleveNoteResponse>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Notes
 */
export interface INotesClient {
    /**
     * @generated from protobuf rpc: Insert(stream mg.tonymushah.evalfjuil24.admin.InsertNotesRequest) returns (stream mg.tonymushah.evalfjuil24.admin.InsertNotesResponse);
     */
    insert(options?: RpcOptions): DuplexStreamingCall<InsertNotesRequest, InsertNotesResponse>;
    /**
     * @generated from protobuf rpc: InsertParPromotion(mg.tonymushah.evalfjuil24.admin.InsertNotesParPromotionRequest) returns (mg.tonymushah.evalfjuil24.Empty);
     */
    insertParPromotion(input: InsertNotesParPromotionRequest, options?: RpcOptions): UnaryCall<InsertNotesParPromotionRequest, Empty>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Notes
 */
export class NotesClient implements INotesClient, ServiceInfo {
    typeName = Notes.typeName;
    methods = Notes.methods;
    options = Notes.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Insert(stream mg.tonymushah.evalfjuil24.admin.InsertNotesRequest) returns (stream mg.tonymushah.evalfjuil24.admin.InsertNotesResponse);
     */
    insert(options?: RpcOptions): DuplexStreamingCall<InsertNotesRequest, InsertNotesResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<InsertNotesRequest, InsertNotesResponse>("duplex", this._transport, method, opt);
    }
    /**
     * @generated from protobuf rpc: InsertParPromotion(mg.tonymushah.evalfjuil24.admin.InsertNotesParPromotionRequest) returns (mg.tonymushah.evalfjuil24.Empty);
     */
    insertParPromotion(input: InsertNotesParPromotionRequest, options?: RpcOptions): UnaryCall<InsertNotesParPromotionRequest, Empty> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<InsertNotesParPromotionRequest, Empty>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Getters
 */
export interface IGettersClient {
    /**
     * @generated from protobuf rpc: Semetres(mg.tonymushah.evalfjuil24.admin.GetSemetresRequest) returns (mg.tonymushah.evalfjuil24.admin.GetSemetresResponse);
     */
    semetres(input: GetSemetresRequest, options?: RpcOptions): UnaryCall<GetSemetresRequest, GetSemetresResponse>;
    /**
     * @generated from protobuf rpc: Matieres(mg.tonymushah.evalfjuil24.admin.GetMatieresRequest) returns (mg.tonymushah.evalfjuil24.admin.GetMatieresResponse);
     */
    matieres(input: GetMatieresRequest, options?: RpcOptions): UnaryCall<GetMatieresRequest, GetMatieresResponse>;
    /**
     * @generated from protobuf rpc: Promotions(mg.tonymushah.evalfjuil24.admin.GetPromotionsRequest) returns (mg.tonymushah.evalfjuil24.admin.GetPromotionsResponse);
     */
    promotions(input: GetPromotionsRequest, options?: RpcOptions): UnaryCall<GetPromotionsRequest, GetPromotionsResponse>;
    /**
     * @generated from protobuf rpc: EtudiantAdmisOuNon(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.admin.EtudiantAdmisOuNonResponse);
     */
    etudiantAdmisOuNon(input: Empty, options?: RpcOptions): UnaryCall<Empty, EtudiantAdmisOuNonResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Getters
 */
export class GettersClient implements IGettersClient, ServiceInfo {
    typeName = Getters.typeName;
    methods = Getters.methods;
    options = Getters.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Semetres(mg.tonymushah.evalfjuil24.admin.GetSemetresRequest) returns (mg.tonymushah.evalfjuil24.admin.GetSemetresResponse);
     */
    semetres(input: GetSemetresRequest, options?: RpcOptions): UnaryCall<GetSemetresRequest, GetSemetresResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetSemetresRequest, GetSemetresResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: Matieres(mg.tonymushah.evalfjuil24.admin.GetMatieresRequest) returns (mg.tonymushah.evalfjuil24.admin.GetMatieresResponse);
     */
    matieres(input: GetMatieresRequest, options?: RpcOptions): UnaryCall<GetMatieresRequest, GetMatieresResponse> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetMatieresRequest, GetMatieresResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: Promotions(mg.tonymushah.evalfjuil24.admin.GetPromotionsRequest) returns (mg.tonymushah.evalfjuil24.admin.GetPromotionsResponse);
     */
    promotions(input: GetPromotionsRequest, options?: RpcOptions): UnaryCall<GetPromotionsRequest, GetPromotionsResponse> {
        const method = this.methods[2], opt = this._transport.mergeOptions(options);
        return stackIntercept<GetPromotionsRequest, GetPromotionsResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: EtudiantAdmisOuNon(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.admin.EtudiantAdmisOuNonResponse);
     */
    etudiantAdmisOuNon(input: Empty, options?: RpcOptions): UnaryCall<Empty, EtudiantAdmisOuNonResponse> {
        const method = this.methods[3], opt = this._transport.mergeOptions(options);
        return stackIntercept<Empty, EtudiantAdmisOuNonResponse>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Auth
 */
export interface IAuthClient {
    /**
     * @generated from protobuf rpc: Login(mg.tonymushah.evalfjuil24.admin.LoginRequest) returns (mg.tonymushah.evalfjuil24.admin.LoginResponse);
     */
    login(input: LoginRequest, options?: RpcOptions): UnaryCall<LoginRequest, LoginResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Auth
 */
export class AuthClient implements IAuthClient, ServiceInfo {
    typeName = Auth.typeName;
    methods = Auth.methods;
    options = Auth.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: Login(mg.tonymushah.evalfjuil24.admin.LoginRequest) returns (mg.tonymushah.evalfjuil24.admin.LoginResponse);
     */
    login(input: LoginRequest, options?: RpcOptions): UnaryCall<LoginRequest, LoginResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<LoginRequest, LoginResponse>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Imports
 */
export interface IImportsClient {
    /**
     * @generated from protobuf rpc: import(stream mg.tonymushah.evalfjuil24.admin.ImportDataMessage) returns (mg.tonymushah.evalfjuil24.Empty);
     */
    import(options?: RpcOptions): ClientStreamingCall<ImportDataMessage, Empty>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.Imports
 */
export class ImportsClient implements IImportsClient, ServiceInfo {
    typeName = Imports.typeName;
    methods = Imports.methods;
    options = Imports.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: import(stream mg.tonymushah.evalfjuil24.admin.ImportDataMessage) returns (mg.tonymushah.evalfjuil24.Empty);
     */
    import(options?: RpcOptions): ClientStreamingCall<ImportDataMessage, Empty> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<ImportDataMessage, Empty>("clientStreaming", this._transport, method, opt);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.CfgNoteService
 */
export interface ICfgNoteServiceClient {
    /**
     * @generated from protobuf rpc: getConfigs(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.admin.GetConfigsMessageResponse);
     */
    getConfigs(input: Empty, options?: RpcOptions): UnaryCall<Empty, GetConfigsMessageResponse>;
    /**
     * @generated from protobuf rpc: updateConfig(mg.tonymushah.evalfjuil24.admin.ConfigNote) returns (mg.tonymushah.evalfjuil24.Empty);
     */
    updateConfig(input: ConfigNote, options?: RpcOptions): UnaryCall<ConfigNote, Empty>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.CfgNoteService
 */
export class CfgNoteServiceClient implements ICfgNoteServiceClient, ServiceInfo {
    typeName = CfgNoteService.typeName;
    methods = CfgNoteService.methods;
    options = CfgNoteService.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: getConfigs(mg.tonymushah.evalfjuil24.Empty) returns (mg.tonymushah.evalfjuil24.admin.GetConfigsMessageResponse);
     */
    getConfigs(input: Empty, options?: RpcOptions): UnaryCall<Empty, GetConfigsMessageResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<Empty, GetConfigsMessageResponse>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: updateConfig(mg.tonymushah.evalfjuil24.admin.ConfigNote) returns (mg.tonymushah.evalfjuil24.Empty);
     */
    updateConfig(input: ConfigNote, options?: RpcOptions): UnaryCall<ConfigNote, Empty> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<ConfigNote, Empty>("unary", this._transport, method, opt, input);
    }
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.SemetresServ
 */
export interface ISemetresServClient {
    /**
     * @generated from protobuf rpc: EtudiantRangs(mg.tonymushah.evalfjuil24.admin.EtudiantRangsRequest) returns (mg.tonymushah.evalfjuil24.admin.EtudiantRangsResponse);
     */
    etudiantRangs(input: EtudiantRangsRequest, options?: RpcOptions): UnaryCall<EtudiantRangsRequest, EtudiantRangsResponse>;
}
/**
 * @generated from protobuf service mg.tonymushah.evalfjuil24.admin.SemetresServ
 */
export class SemetresServClient implements ISemetresServClient, ServiceInfo {
    typeName = SemetresServ.typeName;
    methods = SemetresServ.methods;
    options = SemetresServ.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: EtudiantRangs(mg.tonymushah.evalfjuil24.admin.EtudiantRangsRequest) returns (mg.tonymushah.evalfjuil24.admin.EtudiantRangsResponse);
     */
    etudiantRangs(input: EtudiantRangsRequest, options?: RpcOptions): UnaryCall<EtudiantRangsRequest, EtudiantRangsResponse> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<EtudiantRangsRequest, EtudiantRangsResponse>("unary", this._transport, method, opt, input);
    }
}
