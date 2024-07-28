use std::env;

use evaluation_fin_juillet_2024::{self as backend};

use backend::servers::client::{
    AuthService, CurrentService, HelloServ, ReleveService, SemestresService,
};

use proto_client::{
    auth_server::AuthServer, current_server::CurrentServer,
    hello_service_server::HelloServiceServer, releve_server::ReleveServer,
    semestres_server::SemestresServer,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = env::var("CLIENT_BACK_ADDR")
        .map(|r| {
            println!("{r}");
            r
        })?
        .parse()?;
    let pool = backend::etablish_connection();
    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(HelloServiceServer::new(HelloServ)))
        .add_service(tonic_web::enable(AuthServer::new(AuthService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(CurrentServer::new(CurrentService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(ReleveServer::new(ReleveService {
            pool: pool.clone(),
        })))
        .add_service(tonic_web::enable(SemestresServer::new(SemestresService {
            pool: pool.clone(),
        })))
        .serve(addr)
        .await?;
    Ok(())
}
