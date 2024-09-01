use diesel::{prelude::*, update};
use proto_admin::{cfg_note_service_server::CfgNoteService, ConfigNote, GetConfigsMessageResponse};
use protos_commons::Empty;
use tonic::{Request, Response};

use crate::{models::table::config_note::ConfigNoteEntry, servers::TonicRpcResult, DbPool};

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
        let pool = self.pool.clone();
        let res = crate::spawn_blocking(move || -> crate::Result<_> {
            use diesel_schemas::schema::configuration_note::dsl::*;
            let mut con = pool.get()?;
            Ok(configuration_note
                .select(ConfigNoteEntry::as_select())
                .get_results(&mut con)?)
        })
        .await??;
        Ok(Response::new(GetConfigsMessageResponse {
            configs: res.into_iter().map(|v| v.into()).collect(),
        }))
    }
    async fn update_config(&self, request: Request<ConfigNote>) -> TonicRpcResult<Empty> {
        let pool = self.pool.clone();
        let entry: ConfigNoteEntry = request.get_ref().clone().into();
        crate::spawn_blocking(move || -> crate::Result<()> {
            use diesel_schemas::schema::configuration_note::dsl::*;
            let mut con = pool.get()?;
            println!("inserting");
            update(configuration_note)
                .filter(code.eq(&entry.code))
                .set(&entry)
                .execute(&mut con)?;
            println!("ok");
            Ok(())
        })
        .await??;
        Ok(Response::new(Empty {}))
    }
}
