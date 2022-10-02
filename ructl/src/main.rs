use std::io::Read;
use std::sync::Arc;

use anyhow::bail;
use protokit::grpc::futures::StreamExt;
use rubeapi::api::api::{DeleteRequest, GetRequest, ListRequest, PutRequest, WatchRequest};
// use rubeapi::api::systemd::unit::Unit;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let registry = rubeapi::registry();
    println!("{:#?}", registry);

    let mut client = rubeapi::api::api::ApiClient::connect("tcp://localhost:8080").await?;
    if args.len() < 2 {
        bail!("Misssing command")
    }

    match args[1].as_str() {
        "put" => {
            let mut all = String::new();
            let _ = std::io::stdin().read_to_string(&mut all)?;

            let mut msg = PutRequest::default();
            protokit::textformat::decode_into(&all, &registry, &mut msg)?;
            let response = client.put(msg).await?;

            let out = protokit::textformat::encode(&response.into_inner(), &registry)?;
            println!("{}", out);
        }
        "list" => {
            let mut all = String::new();
            let _ = std::io::stdin().read_to_string(&mut all)?;

            let mut msg = ListRequest::default();
            protokit::textformat::decode_into(&all, &registry, &mut msg)?;
            let response = client.list(msg).await?;

            let out = protokit::textformat::encode(&response.into_inner(), &registry)?;
            println!("{}", out);
        }
        "get" => {
            let mut all = String::new();
            let _ = std::io::stdin().read_to_string(&mut all)?;

            let mut msg = GetRequest::default();
            protokit::textformat::decode_into(&all, &registry, &mut msg)?;
            let response = client.get(msg).await?.into_inner();

            let out = protokit::textformat::encode(&response, &registry)?;
            println!("{}", out);
        }
        "watch" => {
            let mut all = String::new();
            let _ = std::io::stdin().read_to_string(&mut all)?;

            let mut msg = WatchRequest::default();
            protokit::textformat::decode_into(&all, &registry, &mut msg)?;
            let response = client.watch(msg).await?.into_inner();

            response
                .for_each(|item| async {
                    let out = protokit::textformat::encode(&item.unwrap(), &registry).unwrap();
                    println!("{}", out);
                })
                .await;
        }
        "delete" => {
            let mut all = String::new();
            let _ = std::io::stdin().read_to_string(&mut all)?;

            let mut msg = DeleteRequest::default();
            protokit::textformat::decode_into(&all, &registry, &mut msg)?;
            let response = client.delete(msg).await?;

            let out = protokit::textformat::encode(&response.into_inner(), &registry)?;
            println!("{}", out);
        }
        other => {
            bail!("Unknown command: {other}");
        }
    }

    Ok(())
}
