use std::env;

use evaluation_fin_juillet_2024::{self as backend};

use backend::servers::admin::{
    AuthService, CfgNoteServiceImpl, DatabaseService, EtudiantsService, GettersService, HelloServ,
    ImportService, NotesService, SemestresImplService,
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
    let addr = env::var("ADMIN_BACK_ADDR_WEB")
        .map(|r| {
            println!("{r}");
            r
        })?
        .parse()?;
    let pool = backend::etablish_connection();
    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(HelloServiceServer::new(HelloServ)))
        .add_service(tonic_web::enable(DatabaseServer::new(DatabaseService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(AuthServer::new(AuthService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(EtudiantsServer::new(EtudiantsService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(GettersServer::new(GettersService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(NotesServer::new(NotesService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(ImportsServer::new(ImportService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(CfgNoteServiceServer::new(
            CfgNoteServiceImpl { pool: pool.clone() },
        )))
        .add_service(tonic_web::enable(SemetresServServer::new(
            SemestresImplService { pool: pool.clone() },
        )))
        .serve(addr)
        .await?;
    Ok(())
}
