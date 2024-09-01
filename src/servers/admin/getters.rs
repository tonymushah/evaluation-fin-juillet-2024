use proto_admin::{
    getters_server::Getters, EtudiantAdmisOuNonResponse, GetMatieresRequest, GetMatieresResponse,
    GetPromotionsRequest, GetPromotionsResponse, GetSemetresRequest, GetSemetresResponse, Semestre,
};

use diesel::prelude::*;
use diesel_schemas::schema::{promotion, semestre};

use protos_commons::Empty;
use tonic::{Request, Response};

use crate::{
    models::table::Matiere, modules::etudiant_status::get_etudiants_status, paginate::Paginate,
    servers::TonicRpcResult, DbPool,
};

#[derive(Debug, Clone)]
pub struct GettersService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Getters for GettersService {
    async fn semetres(
        &self,
        _request: Request<GetSemetresRequest>,
    ) -> TonicRpcResult<GetSemetresResponse> {
        let pool = self.pool.clone();
        let sems = crate::spawn_blocking(move || -> crate::Result<Vec<Semestre>> {
            use self::semestre::dsl::*;
            let mut con = pool.get()?;
            Ok(semestre
                .select(id_sem)
                .get_results::<String>(&mut con)?
                .into_iter()
                .map(|sem| Semestre { numero: sem })
                .collect())
        })
        .await??;
        Ok(Response::new(GetSemetresResponse { semestre: sems }))
    }
    async fn matieres(
        &self,
        request: Request<GetMatieresRequest>,
    ) -> TonicRpcResult<GetMatieresResponse> {
        let pool = self.pool.clone();
        let req_data = request.get_ref().clone();
        let req = req_data.clone();
        let (data, total) = crate::spawn_blocking(move || -> crate::Result<(Vec<Matiere>, i64)> {
            use diesel::prelude::*;
            use diesel_schemas::schema::matiere::dsl::*;
            let mut con = pool.get()?;
            let offset = req.offset.unwrap_or_default();
            let limit = req.limit.unwrap_or(10);
            let mut query = matiere.into_boxed();
            if !req.semstres.is_empty() {
                query = query.filter(semestre.eq_any(req.semstres));
            }
            Ok(query
                .paginate(offset as i64, limit as i64)
                .load_data(&mut con)?)
        })
        .await??;
        let offset = req_data.offset.unwrap_or_default();
        let limit = req_data.limit.unwrap_or(10);
        Ok(Response::new(GetMatieresResponse {
            matieres: data.into_iter().map(|e| e.into()).collect(),
            offset,
            limit,
            total: total as u64,
        }))
    }
    async fn promotions(
        &self,
        _request: Request<GetPromotionsRequest>,
    ) -> TonicRpcResult<GetPromotionsResponse> {
        let pool = self.pool.clone();
        let proms = crate::spawn_blocking(move || -> crate::Result<Vec<String>> {
            use self::promotion::dsl::*;
            let mut con = pool.get()?;
            Ok(promotion
                .select(id_promotion)
                .get_results::<String>(&mut con)?)
        })
        .await??;
        Ok(Response::new(GetPromotionsResponse { promotions: proms }))
    }
    async fn etudiant_admis_ou_non(
        &self,
        _request: Request<Empty>,
    ) -> TonicRpcResult<EtudiantAdmisOuNonResponse> {
        let pool = self.pool.clone();
        let res = crate::spawn_blocking(move || -> crate::Result<_> {
            let mut con = pool.get()?;
            Ok(get_etudiants_status(&mut con)?.values().fold(
                EtudiantAdmisOuNonResponse {
                    admis: 0,
                    ajournee: 0,
                },
                |mut acc, v| {
                    match v {
                        protos_commons::EtudiantStatus::EAdmis => acc.admis += 1,
                        protos_commons::EtudiantStatus::EAjournee => acc.ajournee += 1,
                    };
                    acc
                },
            ))
        })
        .await??;
        Ok(Response::new(res))
    }
}
