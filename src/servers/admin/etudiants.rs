use proto_admin::{
    etudiants_server::Etudiants, EtudiantInfoRequest, EtudiantInfoResponse,
    EtudiantReleveNoteRequest, EtudiantReleveNoteResponse, EtudiantsListRequest,
    EtudiantsListResponse,
};
use protos_commons::ReleveNote;
use tonic::{Request, Response};

use crate::{
    models::table::{etudiant_note::GetReleveNote, Etudiant},
    servers::TonicRpcResult,
    DbPool,
};

#[derive(Debug, Clone)]
pub struct EtudiantsService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Etudiants for EtudiantsService {
    async fn list(
        &self,
        request: Request<EtudiantsListRequest>,
    ) -> TonicRpcResult<EtudiantsListResponse> {
        crate::tonic_not_implemented()
    }
    async fn info(
        &self,
        request: Request<EtudiantInfoRequest>,
    ) -> TonicRpcResult<EtudiantInfoResponse> {
        let EtudiantInfoRequest { numero } = request.get_ref().clone();
        let pool = self.pool.clone();
        let etu = crate::spawn_blocking(move || -> crate::Result<Etudiant> {
            use diesel::prelude::*;
            use diesel_schemas::schema::etudiant::dsl::*;
            let mut con = pool.get()?;
            Ok(etudiant
                .filter(etu.eq(numero))
                .select(Etudiant::as_select())
                .get_result(&mut con)?)
        })
        .await??;
        Ok(Response::new(EtudiantInfoResponse {
            current: Some(etu.into()),
        }))
    }
    async fn releve_note(
        &self,
        request: Request<EtudiantReleveNoteRequest>,
    ) -> TonicRpcResult<EtudiantReleveNoteResponse> {
        let EtudiantReleveNoteRequest {
            etudiant: etu,
            semestre: sems,
        } = request.get_ref();
        let get_releves = sems
            .iter()
            .map(|sem| GetReleveNote {
                etudiant: etu.clone(),
                semestre: sem.clone(),
            })
            .collect::<Vec<_>>();
        let pool = self.pool.clone();
        let releves = crate::spawn_blocking(move || -> crate::Result<Vec<ReleveNote>> {
            let mut con = pool.get()?;
            Ok(get_releves
                .into_iter()
                .flat_map(|get| get.get(&mut con))
                .collect())
        })
        .await??;
        Ok(Response::new(EtudiantReleveNoteResponse { releves }))
    }
}
