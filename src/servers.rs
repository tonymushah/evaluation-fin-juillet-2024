pub mod admin;

pub type TonicResult<T> = Result<T, tonic::Status>;

pub type TonicRpcResult<T> = TonicResult<tonic::Response<T>>;
