pub mod admin;
pub mod client;

pub type TonicResult<T> = Result<T, tonic::Status>;

pub type TonicRpcResult<T> = TonicResult<tonic::Response<T>>;
