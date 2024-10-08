use proto_client::{
    releve_server::Releve, GetReleveRequest, GetReleveResponse, ListeRatrapageResponse,
};
use protos_commons::{Empty, ReleveNote};
use tonic::{Request, Response};

use crate::{
    modules::{
        etudiant_note::{get_etudiant_notes, GetReleveNote},
        raptrapages::ExtractRatrapage,
    },
    servers::TonicRpcResult,
    token::{ClientHmac, ExtractSessionData},
    DbPool,
};

#[derive(Debug, Clone)]
pub struct ReleveService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Releve for ReleveService {
    async fn get(&self, request: Request<GetReleveRequest>) -> TonicRpcResult<GetReleveResponse> {
        let id = request.get_current(&{ ClientHmac::extract_client() })?;
        let pool = self.pool.clone();
        let getter = GetReleveNote {
            etudiant: id,
            semestre: request.get_ref().semetre.clone(),
        };
        let releve = crate::spawn_blocking(move || -> crate::Result<ReleveNote> {
            let mut con = pool.get()?;
            Ok(getter.get(&mut con)?)
        })
        .await??;
        Ok(Response::new(GetReleveResponse {
            releves: Some(releve),
        }))
    }
    async fn liste_ratrapage(
        &self,
        request: Request<Empty>,
    ) -> TonicRpcResult<ListeRatrapageResponse> {
        let id = request.get_current(&{ ClientHmac::extract_client() })?;
        let pool = self.pool.clone();
        let releves = crate::spawn_blocking(move || -> crate::Result<Vec<ReleveNote>> {
            let mut con = pool.get()?;
            Ok(get_etudiant_notes(&id, &mut con)?)
        })
        .await??;
        Ok(Response::new(ListeRatrapageResponse {
            list: releves
                .into_iter()
                .flat_map(|releve| releve.extract_ratrapage())
                .collect(),
        }))
    }
}
