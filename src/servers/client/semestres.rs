use diesel::prelude::*;
use proto_client::{
    semestres_server::Semestres, EtudiantSemestre, SemestresListRequest, SemestresListResponse,
};
use protos_commons::ReleveNote;
use tonic::{Request, Response};

use crate::{
    modules::etudiant_note::GetReleveNote,
    servers::TonicRpcResult,
    token::{ClientHmac, ExtractSessionData},
    DbPool,
};

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
        let id = request.get_current(&{ ClientHmac::extract_client() })?;
        let sems = request.get_ref().semestre.clone();
        let pool = self.pool.clone();
        let res = crate::spawn_blocking(move || -> crate::Result<Vec<ReleveNote>> {
            let mut con = pool.get()?;
            let re_sems: Vec<String> = if sems.is_empty() {
                use diesel_schemas::schema::semestre::dsl::*;
                semestre.select(id_sem).get_results(&mut con)?
            } else {
                sems
            };
            Ok(re_sems
                .iter()
                .map(|sem| GetReleveNote {
                    etudiant: id.clone(),
                    semestre: sem.clone(),
                })
                .flat_map(|get| get.get(&mut con))
                .collect())
        })
        .await??;
        Ok(Response::new(SemestresListResponse {
            semetres: res
                .into_iter()
                .map(|note| EtudiantSemestre {
                    semetre: note.semestre,
                    status: note.status,
                    moyenne: note.moyenne,
                })
                .collect(),
        }))
    }
}
