use proto_admin::{auth_server::Auth, LoginRequest, LoginResponse};
use tonic::Request;

use crate::{servers::TonicRpcResult, tonic_not_implemented, DbPool};

#[derive(Debug, Clone)]
pub struct AuthService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(&self, _reuest: Request<LoginRequest>) -> TonicRpcResult<LoginResponse> {
        tonic_not_implemented()
    }
}
