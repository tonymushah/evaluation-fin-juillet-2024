use proto_client::{semestres_server::Semestres, SemestresListRequest, SemestresListResponse};
use tonic::Request;

use crate::{servers::TonicRpcResult, tonic_not_implemented, DbPool};

#[derive(Debug, Clone)]
pub struct SemestresService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Semestres for SemestresService {
    async fn list(
        &self,
        request: Request<SemestresListRequest>,
    ) -> TonicRpcResult<SemestresListResponse> {
        tonic_not_implemented()
    }
}
