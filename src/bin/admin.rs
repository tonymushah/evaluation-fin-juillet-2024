use std::env;

use evaluation_fin_juillet_2024::{self as backend};

use backend::servers::admin::{
    AuthService, DatabaseService, EtudiantsService, GettersService, HelloServ, ImportService,
    NotesService,
};

use proto_admin::{
    auth_server::AuthServer, database_server::DatabaseServer, etudiants_server::EtudiantsServer,
    getters_server::GettersServer, hello_service_server::HelloServiceServer,
    imports_server::ImportsServer, notes_server::NotesServer,
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
        .serve(addr)
        .await?;
    Ok(())
}
