use diesel::prelude::*;
use diesel_schemas::schema::etudiant;
use jwt::SignWithKey;
use proto_client::{auth_server::Auth, LoginRequest, LoginResponse};
use tonic::{Request, Response, Status};

use crate::{servers::TonicRpcResult, token::ClientHmac, DbPool};

#[derive(Debug, Clone)]
pub struct AuthService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(&self, request: Request<LoginRequest>) -> TonicRpcResult<LoginResponse> {
        let pool = self.pool.clone();
        let numero = request.get_ref().numero.clone();
        let hmac = ClientHmac::extract_client();
        let token = crate::spawn_blocking(move || -> crate::Result<String> {
            use self::etudiant::dsl::*;
            let mut con = pool.get()?;
            let res = etudiant
                .filter(etu.eq(numero.clone()))
                .select(etu)
                .get_result::<String>(&mut con);
            Ok(res?)
        })
        .await??
        .sign_with_key(&*hmac)
        .map_err(|e| Status::unknown(e.to_string()))?;
        Ok(Response::new(LoginResponse { token }))
    }
}
