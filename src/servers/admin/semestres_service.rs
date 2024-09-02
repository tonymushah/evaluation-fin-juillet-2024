use proto_admin::{
    semetres_serv_server::SemetresServ, EtudiantRangsRequest, EtudiantRangsResponse,
};
use tonic::Request;

use crate::{servers::TonicRpcResult, tonic_not_implemented, DbPool};

#[derive(Debug, Clone)]
pub struct SemestresImplService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl SemetresServ for SemestresImplService {
    async fn etudiant_rangs(
        &self,
        _request: Request<EtudiantRangsRequest>,
    ) -> TonicRpcResult<EtudiantRangsResponse> {
        tonic_not_implemented()
    }
}
