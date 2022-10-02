use std::collections::HashMap;

use futures::StreamExt;
use protokit::binformat::Message;
use crate::api::api::{ApiClient, ListRequest};
use protokit::grpc::tonic::transport::Channel;
use crate::{Client, Event, Object};

mod systemd;

#[async_trait::async_trait]
pub trait Control<O: Message> {
    /// Takes in the latest state of the object, applies it somewhere else
    async fn update(&mut self, client: &mut Client, obj: &Object<O>);
    /// Marks the deletion of an object, obj is the latest version of the object that was available
    async fn delete(&mut self, client: &mut Client, obj: &Object<O>);
}

pub struct Controller<O: Message, C: Control<O>> {
    client: Client,
    control: C,
    state: HashMap<Box<str>, Object<O>>,
    version: u64,
}

impl<O: Message, C: Control<O>> Controller<O, C> {
    pub async fn run(mut client: Client, namespace: String, control: C) -> anyhow::Result<()> {
        let items = client.list::<O>(namespace.clone()).await?;
        let state = HashMap::from_iter(items.into_iter().map(|it| (it.meta.name.clone(), it)));
        let mut this = Controller {
            client,
            control,
            version: state.iter().map(|v| v.1.version).max().unwrap_or(0),
            state,
        };

        for (_, obj) in this.state.iter_mut() {
            this.control.update(&mut this.client, &obj).await;
        }

        let mut watch = this
            .client
            .clone()
            .watch::<O>(Some(namespace), None::<String>, Some(this.version))
            .await?;

        while let Some(it) = watch.next().await {
            let it = it?;
            match it {
                Event::Modified(o) => {
                    this.control.update(&mut this.client, &o).await;
                    this.version = o.version;
                    this.state.insert(o.meta.name.clone(), o);
                }
                Event::Deleted(o) => {
                    this.control.delete(&mut this.client, &o).await;
                    this.version = o.version;
                    this.state.remove(&o.meta.name);
                }
            }
        }

        Ok(())
    }
}
