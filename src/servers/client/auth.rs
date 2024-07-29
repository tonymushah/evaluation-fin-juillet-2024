use diesel::prelude::*;
use diesel_schemas::schema::etudiant;
use proto_client::{auth_server::Auth, LoginRequest, LoginResponse};
use tonic::{Request, Response};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct AuthService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(&self, request: Request<LoginRequest>) -> TonicRpcResult<LoginResponse> {
        let pool = self.pool.clone();
        let numero = request.get_ref().numero.clone();

        let token = crate::spawn_blocking(move || -> crate::Result<String> {
            use self::etudiant::dsl::*;
            let mut con = pool.get()?;
            let res = etudiant
                .filter(etu.eq(numero.clone()))
                .select(etu)
                .get_result::<String>(&mut con);
            Ok(res?)
        })
        .await??;
        Ok(Response::new(LoginResponse { token }))
    }
}
