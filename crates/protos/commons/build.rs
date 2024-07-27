fn main() {
    let default_conf = tonic_build::configure().build_client(false);
    default_conf
        .clone()
        .compile(&["../../../protos/commons.proto"], &["../../../protos"])
        .unwrap();
}
