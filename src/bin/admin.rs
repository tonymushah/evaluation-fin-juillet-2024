use std::env;

use evaluation_fin_juillet_2024::{self as backend};

use backend::servers::admin::{DatabaseService, HelloServ};

use proto_admin::{database_server::DatabaseServer, hello_service_server::HelloServiceServer};
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
        .accept_http1(true)
        .add_service(tonic_web::enable(HelloServiceServer::new(HelloServ)))
        .add_service(tonic_web::enable(DatabaseServer::new(DatabaseService {
            pool: pool.clone(),
        })))
        .serve(addr)
        .await?;
    Ok(())
}
