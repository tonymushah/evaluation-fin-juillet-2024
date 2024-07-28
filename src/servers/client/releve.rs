use proto_client::{releve_server::Releve, GetReleveRequest, GetReleveResponse};
use tonic::Request;

use crate::{servers::TonicRpcResult, tonic_not_implemented, DbPool};

#[derive(Debug, Clone)]
pub struct ReleveService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Releve for ReleveService {
    async fn get(&self, request: Request<GetReleveRequest>) -> TonicRpcResult<GetReleveResponse> {
        tonic_not_implemented()
    }
}
