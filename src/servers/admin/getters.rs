use proto_admin::{
    getters_server::Getters, GetMatieresRequest, GetMatieresResponse, GetPromotionsRequest,
    GetPromotionsResponse, GetSemetresRequest, GetSemetresResponse,
};

use tonic::Request;

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct GettersService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Getters for GettersService {
    async fn semetres(
        &self,
        request: Request<GetSemetresRequest>,
    ) -> TonicRpcResult<GetSemetresResponse> {
        crate::tonic_not_implemented()
    }
    async fn matieres(
        &self,
        request: Request<GetMatieresRequest>,
    ) -> TonicRpcResult<GetMatieresResponse> {
        crate::tonic_not_implemented()
    }
    async fn promotions(
        &self,
        request: Request<GetPromotionsRequest>,
    ) -> TonicRpcResult<GetPromotionsResponse> {
        crate::tonic_not_implemented()
    }
}
