use proto_admin::{hello_service_server::HelloService, HelloRequest, HelloResponse};
use tonic::{Request, Response};

use crate::servers::TonicRpcResult;

#[derive(Debug, Clone, Copy)]
pub struct HelloServ;

#[tonic::async_trait]
impl HelloService for HelloServ {
    async fn say(&self, request: Request<HelloRequest>) -> TonicRpcResult<HelloResponse> {
        let content = &request.get_ref().content;
        Ok(Response::new(HelloResponse {
            message: if content.len() > 10 {
                "Sorry! You said something more than 10 characters".into()
            } else {
                format!("Hi! It's the server! You said {content}.")
            },
        }))
    }
}
