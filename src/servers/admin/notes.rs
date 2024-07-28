use std::pin::Pin;

use proto_admin::{notes_server::Notes, InsertNotesRequest, InsertNotesResponse};
use tokio_stream::Stream;
use tonic::{Request, Streaming};

use crate::{servers::TonicRpcResult, tonic_not_implemented, DbPool};

#[derive(Debug, Clone)]
pub struct NotesService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Notes for NotesService {
    type InsertStream =
        Pin<Box<dyn Stream<Item = Result<InsertNotesResponse, tonic::Status>> + Send + 'static>>;

    async fn insert(
        &self,
        request: Request<Streaming<InsertNotesRequest>>,
    ) -> TonicRpcResult<Self::InsertStream> {
        tonic_not_implemented()
    }
}
