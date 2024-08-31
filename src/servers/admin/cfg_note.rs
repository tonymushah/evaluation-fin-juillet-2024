use proto_admin::{cfg_note_service_server::CfgNoteService, ConfigNote, GetConfigsMessageResponse};
use protos_commons::Empty;
use tonic::Request;

use crate::{servers::TonicRpcResult, tonic_not_implemented, DbPool};

#[derive(Debug, Clone)]
pub struct CfgNoteServiceImpl {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl CfgNoteService for CfgNoteServiceImpl {
    async fn get_configs(
        &self,
        _request: Request<Empty>,
    ) -> TonicRpcResult<GetConfigsMessageResponse> {
        // TODO
        tonic_not_implemented()
    }
    async fn update_config(&self, _request: Request<ConfigNote>) -> TonicRpcResult<Empty> {
        // TODO
        tonic_not_implemented()
    }
}
