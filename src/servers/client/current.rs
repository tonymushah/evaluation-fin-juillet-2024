use diesel::prelude::*;
use diesel_schemas::schema::etudiant;
use proto_client::current_server::Current;
use protos_commons::{Empty, Etudiant};
use tonic::{Request, Response, Status};

use crate::{
    models::table::Etudiant as CEtudiant,
    servers::TonicRpcResult,
    spawn_blocking,
    token::{ClientHmac, ExtractSessionData},
    DbPool,
};

#[derive(Debug, Clone)]
pub struct CurrentService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Current for CurrentService {
    async fn get(&self, request: Request<Empty>) -> TonicRpcResult<Etudiant> {
        let pool = self.pool.clone();
        let id = request.get_current(&{ ClientHmac::extract_client() })?;
        Ok(Response::new(
            spawn_blocking(move || -> crate::Result<CEtudiant> {
                use self::etudiant::dsl::*;
                let mut con = pool.get()?;
                Ok(etudiant
                    .filter(etu.eq(id))
                    .select(CEtudiant::as_select())
                    .get_result(&mut con)?)
            })
            .await??
            .into(),
        ))
    }
}
