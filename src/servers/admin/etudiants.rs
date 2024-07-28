use proto_admin::{
    etudiants_server::Etudiants, EtudiantInfoRequest, EtudiantInfoResponse,
    EtudiantReleveNoteRequest, EtudiantReleveNoteResponse, EtudiantsListRequest,
    EtudiantsListResponse,
};
use tonic::Request;

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct EtudiantsService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Etudiants for EtudiantsService {
    async fn list(
        &self,
        request: Request<EtudiantsListRequest>,
    ) -> TonicRpcResult<EtudiantsListResponse> {
        crate::tonic_not_implemented()
    }
    async fn info(
        &self,
        request: Request<EtudiantInfoRequest>,
    ) -> TonicRpcResult<EtudiantInfoResponse> {
        crate::tonic_not_implemented()
    }
    async fn releve_note(
        &self,
        request: Request<EtudiantReleveNoteRequest>,
    ) -> TonicRpcResult<EtudiantReleveNoteResponse> {
        crate::tonic_not_implemented()
    }
}
