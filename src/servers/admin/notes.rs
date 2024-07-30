use std::pin::Pin;

use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::{insert_into, prelude::*};
use proto_admin::{notes_server::Notes, InsertNotesRequest, InsertNotesResponse};
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Streaming};
use uuid::Uuid;

use crate::{
    models::table::{etudiant_note::now, Note},
    servers::TonicRpcResult,
    DbPool,
};

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
        let pool = self.pool.clone();
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(input) = stream.next().await {
                let input = input?;
                let req_id = input.req_id.clone();
                let custom_message = String::new();
                let poll = pool.clone();
                crate::spawn_blocking(move || -> crate::Result<Note> {
                    use diesel_schemas::schema::note::dsl::*;
                    let mut con = poll.get()?;
                    Ok(insert_into(note).values(Note {
                        id_note: Uuid::new_v4(),
                        etudiant: input.etudiant,
                        matiere: input.matiere,
                        valeur: BigDecimal::from_f32(input.note as f32).unwrap_or_default(),
                        submission: now()
                    }).returning(Note::as_returning()).get_result(&mut con)?)
                }).await??;
                yield InsertNotesResponse {
                    req_id,
                    custom_message
                };
            }
        };
        Ok(Response::new(Box::pin(output)))
    }
}
