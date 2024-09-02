use std::env;

use evaluation_fin_juillet_2024::{
    self as backend,
    servers::admin::{cfg_note::CfgNoteServiceImpl, SemestresImplService},
};

use backend::servers::admin::{
    AuthService, DatabaseService, EtudiantsService, GettersService, HelloServ, ImportService,
    NotesService,
};

use proto_admin::{
    auth_server::AuthServer, cfg_note_service_server::CfgNoteServiceServer,
    database_server::DatabaseServer, etudiants_server::EtudiantsServer,
    getters_server::GettersServer, hello_service_server::HelloServiceServer,
    imports_server::ImportsServer, notes_server::NotesServer,
    semetres_serv_server::SemetresServServer,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = env::var("ADMIN_BACK_ADDR")
        .map(|r| {
            println!("{r}");
            r
        })?
        .parse()?;
    let pool = backend::etablish_connection();
    Server::builder()
        .add_service(HelloServiceServer::new(HelloServ))
        .add_service(DatabaseServer::new(DatabaseService { pool: pool.clone() }))
        .add_service(AuthServer::new(AuthService { pool: pool.clone() }))
        .add_service(EtudiantsServer::new(EtudiantsService {
            pool: pool.clone(),
        }))
        .add_service(GettersServer::new(GettersService { pool: pool.clone() }))
        .add_service(NotesServer::new(NotesService { pool: pool.clone() }))
        .add_service(ImportsServer::new(ImportService { pool: pool.clone() }))
        .add_service(CfgNoteServiceServer::new(CfgNoteServiceImpl {
            pool: pool.clone(),
        }))
        .add_service(SemetresServServer::new(SemestresImplService {
            pool: pool.clone(),
        }))
        .serve(addr)
        .await?;
    Ok(())
}
