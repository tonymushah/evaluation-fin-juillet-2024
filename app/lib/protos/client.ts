// @generated by protobuf-ts 2.9.4 with parameter server_none
// @generated from protobuf file "client.proto" (package "mg.tonymushah.evalfjuil24.client", syntax proto3)
// tslint:disable
import { Etudiant } from "./commons";
import { Empty } from "./commons";
import { ServiceType } from "@protobuf-ts/runtime-rpc";
import { MessageType } from "@protobuf-ts/runtime";
import { ReleveNote } from "./commons";
import { ReleveNoteStatus } from "./commons";
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.HelloRequest
 */
export interface HelloRequest {
    /**
     * @generated from protobuf field: string content = 1;
     */
    content: string;
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.HelloResponse
 */
export interface HelloResponse {
    /**
     * @generated from protobuf field: string message = 1;
     */
    message: string;
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.LoginRequest
 */
export interface LoginRequest {
    /**
     * @generated from protobuf field: string numero = 1;
     */
    numero: string;
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.LoginResponse
 */
export interface LoginResponse {
    /**
     * @generated from protobuf field: string token = 1;
     */
    token: string;
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.SemestresListRequest
 */
export interface SemestresListRequest {
    /**
     * @generated from protobuf field: repeated string semestre = 1;
     */
    semestre: string[];
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.EtudiantSemestre
 */
export interface EtudiantSemestre {
    /**
     * @generated from protobuf field: string semetre = 1;
     */
    semetre: string;
    /**
     * @generated from protobuf field: mg.tonymushah.evalfjuil24.ReleveNoteStatus status = 2;
     */
    status: ReleveNoteStatus;
    /**
     * @generated from protobuf field: double moyenne = 3;
     */
    moyenne: number;
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.SemestresListResponse
 */
export interface SemestresListResponse {
    /**
     * @generated from protobuf field: repeated mg.tonymushah.evalfjuil24.client.EtudiantSemestre semetres = 1;
     */
    semetres: EtudiantSemestre[];
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.GetReleveRequest
 */
export interface GetReleveRequest {
    /**
     * @generated from protobuf field: string semetre = 1;
     */
    semetre: string;
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.GetReleveResponse
 */
export interface GetReleveResponse {
    /**
     * @generated from protobuf field: mg.tonymushah.evalfjuil24.ReleveNote releves = 1;
     */
    releves?: ReleveNote;
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.Ratrapage
 */
export interface Ratrapage {
    /**
     * @generated from protobuf field: string semestre = 1;
     */
    semestre: string;
    /**
     * @generated from protobuf field: string matiere = 2;
     */
    matiere: string;
    /**
     * @generated from protobuf field: double note = 3;
     */
    note: number;
    /**
     * @generated from protobuf field: double montant = 4;
     */
    montant: number;
}
/**
 * @generated from protobuf message mg.tonymushah.evalfjuil24.client.ListeRatrapageResponse
 */
export interface ListeRatrapageResponse {
    /**
     * @generated from protobuf field: repeated mg.tonymushah.evalfjuil24.client.Ratrapage list = 1;
     */
    list: Ratrapage[];
}
// @generated message type with reflection information, may provide speed optimized methods
class HelloRequest$Type extends MessageType<HelloRequest> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.HelloRequest", [
            { no: 1, name: "content", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.HelloRequest
 */
export const HelloRequest = new HelloRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class HelloResponse$Type extends MessageType<HelloResponse> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.HelloResponse", [
            { no: 1, name: "message", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.HelloResponse
 */
export const HelloResponse = new HelloResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoginRequest$Type extends MessageType<LoginRequest> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.LoginRequest", [
            { no: 1, name: "numero", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.LoginRequest
 */
export const LoginRequest = new LoginRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoginResponse$Type extends MessageType<LoginResponse> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.LoginResponse", [
            { no: 1, name: "token", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.LoginResponse
 */
export const LoginResponse = new LoginResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SemestresListRequest$Type extends MessageType<SemestresListRequest> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.SemestresListRequest", [
            { no: 1, name: "semestre", kind: "scalar", repeat: 2 /*RepeatType.UNPACKED*/, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.SemestresListRequest
 */
export const SemestresListRequest = new SemestresListRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class EtudiantSemestre$Type extends MessageType<EtudiantSemestre> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.EtudiantSemestre", [
            { no: 1, name: "semetre", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "status", kind: "enum", T: () => ["mg.tonymushah.evalfjuil24.ReleveNoteStatus", ReleveNoteStatus] },
            { no: 3, name: "moyenne", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.EtudiantSemestre
 */
export const EtudiantSemestre = new EtudiantSemestre$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SemestresListResponse$Type extends MessageType<SemestresListResponse> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.SemestresListResponse", [
            { no: 1, name: "semetres", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => EtudiantSemestre }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.SemestresListResponse
 */
export const SemestresListResponse = new SemestresListResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetReleveRequest$Type extends MessageType<GetReleveRequest> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.GetReleveRequest", [
            { no: 1, name: "semetre", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.GetReleveRequest
 */
export const GetReleveRequest = new GetReleveRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetReleveResponse$Type extends MessageType<GetReleveResponse> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.GetReleveResponse", [
            { no: 1, name: "releves", kind: "message", T: () => ReleveNote }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.GetReleveResponse
 */
export const GetReleveResponse = new GetReleveResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class Ratrapage$Type extends MessageType<Ratrapage> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.Ratrapage", [
            { no: 1, name: "semestre", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "matiere", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "note", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ },
            { no: 4, name: "montant", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.Ratrapage
 */
export const Ratrapage = new Ratrapage$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListeRatrapageResponse$Type extends MessageType<ListeRatrapageResponse> {
    constructor() {
        super("mg.tonymushah.evalfjuil24.client.ListeRatrapageResponse", [
            { no: 1, name: "list", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => Ratrapage }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message mg.tonymushah.evalfjuil24.client.ListeRatrapageResponse
 */
export const ListeRatrapageResponse = new ListeRatrapageResponse$Type();
/**
 * @generated ServiceType for protobuf service mg.tonymushah.evalfjuil24.client.HelloService
 */
export const HelloService = new ServiceType("mg.tonymushah.evalfjuil24.client.HelloService", [
    { name: "Say", options: {}, I: HelloRequest, O: HelloResponse }
]);
/**
 * @generated ServiceType for protobuf service mg.tonymushah.evalfjuil24.client.Auth
 */
export const Auth = new ServiceType("mg.tonymushah.evalfjuil24.client.Auth", [
    { name: "Login", options: {}, I: LoginRequest, O: LoginResponse }
]);
/**
 * @generated ServiceType for protobuf service mg.tonymushah.evalfjuil24.client.Semestres
 */
export const Semestres = new ServiceType("mg.tonymushah.evalfjuil24.client.Semestres", [
    { name: "List", options: {}, I: SemestresListRequest, O: SemestresListResponse }
]);
/**
 * @generated ServiceType for protobuf service mg.tonymushah.evalfjuil24.client.Releve
 */
export const Releve = new ServiceType("mg.tonymushah.evalfjuil24.client.Releve", [
    { name: "Get", options: {}, I: GetReleveRequest, O: GetReleveResponse },
    { name: "ListeRatrapage", options: {}, I: Empty, O: ListeRatrapageResponse }
]);
/**
 * @generated ServiceType for protobuf service mg.tonymushah.evalfjuil24.client.Current
 */
export const Current = new ServiceType("mg.tonymushah.evalfjuil24.client.Current", [
    { name: "Get", options: {}, I: Empty, O: Etudiant }
]);
