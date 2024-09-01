use proto_admin::{
    etudiants_server::Etudiants, EtudiantInfoRequest, EtudiantInfoResponse,
    EtudiantReleveNoteRequest, EtudiantReleveNoteResponse, EtudiantsListRequest,
    EtudiantsListResponse,
};
use protos_commons::ReleveNote;
use tonic::{Request, Response};

use crate::{
    models::table::Etudiant, modules::etudiant_note::GetReleveNote, paginate::Paginate,
    servers::TonicRpcResult, DbPool,
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
        let pool = self.pool.clone();
        let req_data = request.get_ref().clone();
        let req = req_data.clone();
        let (data, total) = crate::spawn_blocking(
            move || -> crate::Result<(Vec<protos_commons::Etudiant>, i64)> {
                use diesel::prelude::*;
                use diesel_schemas::schema::etudiant::dsl::*;
                let mut con = pool.get()?;
                let offset = req.offset.unwrap_or_default();
                let limit = req.limit.unwrap_or(10);
                let mut query = etudiant.into_boxed();
                if !req.promotion.is_empty() {
                    query = query.filter(promotion.eq_any(req.promotion));
                }
                if let Some(nom_) = req.nom.as_ref() {
                    query = query.filter(nom.like(nom_));
                }
                Ok(query
                    .paginate(offset as i64, limit as i64)
                    .load_data(&mut con)
                    .and_then(|(data, total): (Vec<Etudiant>, _)| {
                        Ok((
                            data.into_iter()
                                .map(|e| e.into_proto(&mut con))
                                .collect::<Result<Vec<_>, _>>()?,
                            total,
                        ))
                    })?)
            },
        )
        .await??;
        let offset = req_data.offset.unwrap_or_default();
        let limit = req_data.limit.unwrap_or(10);
        Ok(Response::new(EtudiantsListResponse {
            list: data,
            offset,
            limit,
            total: total as u64,
        }))
    }
    async fn info(
        &self,
        request: Request<EtudiantInfoRequest>,
    ) -> TonicRpcResult<EtudiantInfoResponse> {
        let EtudiantInfoRequest { numero } = request.get_ref().clone();
        let pool = self.pool.clone();
        let etu = crate::spawn_blocking(move || -> crate::Result<_> {
            use diesel::prelude::*;
            use diesel_schemas::schema::etudiant::dsl::*;
            let mut con = pool.get()?;
            Ok(etudiant
                .filter(etu.eq(numero))
                .select(Etudiant::as_select())
                .get_result(&mut con)
                .and_then(|et| et.into_proto(&mut con))?)
        })
        .await??;
        Ok(Response::new(EtudiantInfoResponse { current: Some(etu) }))
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
        //println!("{:#?} - {}", sems, etu);
        let pool = self.pool.clone();
        let releves = crate::spawn_blocking(move || -> crate::Result<Vec<ReleveNote>> {
            let mut con = pool.get()?;
            Ok(get_releves
                .into_iter()
                .flat_map(|get| get.get(&mut con))
                .collect())
        })
        .await??;
        //println!("{:#?}", releves);
        Ok(Response::new(EtudiantReleveNoteResponse { releves }))
    }
}
