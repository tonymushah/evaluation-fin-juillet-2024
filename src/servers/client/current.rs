use proto_client::current_server::Current;
use protos_commons::{Empty, Etudiant};
use tonic::Request;

use crate::{servers::TonicRpcResult, tonic_not_implemented, DbPool};

#[derive(Debug, Clone)]
pub struct CurrentService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Current for CurrentService {
    async fn get(&self, _request: Request<Empty>) -> TonicRpcResult<Etudiant> {
        tonic_not_implemented()
    }
}
