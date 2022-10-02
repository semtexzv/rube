use std::sync::Arc;

use futures::StreamExt;
use protokit::reflect::Registry;

mod server;
mod store;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();

    let mut registry = Registry::default();
    protokit::types::register_types(&mut registry);
    rubeapi::register_types(&mut registry);

    let reflect = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(rubeapi::DESCRIPTOR)
        .build()?;

    let store = store::chdb::CHDBStore::new();
    let service =
        rubeapi::api::api::ApiServer::from(server::ServerImpl::new(Arc::new(registry), store));

    tonic::transport::Server::builder()
        .add_service(reflect)
        .add_service(service)
        .serve("127.0.0.1:8080".parse().unwrap())
        .await?;

    Ok(())
}
