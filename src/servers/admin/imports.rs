use std::io::{BufReader, BufWriter, Write};

use csv::Reader;
use diesel::insert_into;
use itu_csv_import::{config::CSVConfigNote, CSVNote};
use proto_admin::{imports_server::Imports, ImportDataMessage};
use protos_commons::Empty;
use tempfile::tempfile;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status, Streaming};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct ImportService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Imports for ImportService {
    async fn import(
        &self,
        mut request: Request<Streaming<ImportDataMessage>>,
    ) -> TonicRpcResult<Empty> {
        let pool = self.pool.clone();
        let mut config_file = tempfile().map_err(|e| Status::aborted(e.to_string()))?;
        let mut notes = tempfile().map_err(|e| Status::aborted(e.to_string()))?;
        {
            let mut config_file_writer = BufWriter::new(&mut config_file);
            let mut notes_writer = BufWriter::new(&mut notes);
            while let Some(input) = request.get_mut().next().await {
                let input = input?;
                match input.i_type {
                    2 => {
                        config_file_writer.write_all(&input.buf)?;
                    }
                    1 => {
                        notes_writer.write_all(&input.buf)?;
                    }
                    _ => {}
                };
            }
            config_file_writer.flush()?;
            notes_writer.flush()?;
        }
        let pool1 = pool.clone();
        crate::spawn_blocking(move || -> crate::Result<()> {
            let vs = CSVNote::read(Reader::from_reader(BufReader::new(notes)));
            let mut con = pool1.get()?;
            CSVNote::inserts(vs, &mut con)?;
            Ok(())
        })
        .await??;
        let pool2 = pool.clone();
        crate::spawn_blocking(move || -> crate::Result<()> {
            use diesel::prelude::*;
            use diesel_schemas::schema::configuration_note::dsl::*;
            let cfg_notes = CSVConfigNote::read(Reader::from_reader(BufReader::new(config_file)));
            let mut con = pool2.get()?;
            insert_into(configuration_note)
                .values(cfg_notes)
                .execute(&mut con)?;
            Ok(())
        })
        .await??;
        Ok(Response::new(Empty {}))
    }
}
