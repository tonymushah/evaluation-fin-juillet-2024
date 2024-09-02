use proto_admin::{
    semetres_serv_server::SemetresServ, EtudiantRangsRequest, EtudiantRangsResponse,
};
use tonic::{Request, Response};

use crate::{
    modules::get_semestre_etudiant_rangs::get_semestre_etudiant_rangs, servers::TonicRpcResult,
    DbPool,
};

#[derive(Debug, Clone)]
pub struct SemestresImplService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl SemetresServ for SemestresImplService {
    async fn etudiant_rangs(
        &self,
        request: Request<EtudiantRangsRequest>,
    ) -> TonicRpcResult<EtudiantRangsResponse> {
        let pool = self.pool.clone();
        let sem = request.get_ref().semestre.clone();
        let list = crate::spawn_blocking(move || -> crate::Result<_> {
            let mut con = pool.get()?;
            Ok(get_semestre_etudiant_rangs(&sem, &mut con)?)
        })
        .await??;
        Ok(Response::new(EtudiantRangsResponse { entries: list }))
    }
}
