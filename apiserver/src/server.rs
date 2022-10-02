use std::ops::Deref;
use std::sync::Arc;

use futures::stream::BoxStream;
use futures::TryStreamExt;
use protokit::reflect::Registry;
use protokit::types::any::Any;
use protokit::types::empty::Empty;
use rubeapi::api::api::{
    self, Api, DeleteRequest, DeleteResponse, GetRequest, GetResponse, ListRequest, ListResponse,
    Meta, PatchRequest, PatchResponse, PutRequest, PutResponse, WatchRequest, WatchResponse,
    WatchResponseDelete, WatchResponseOneOfChange, WatchResponseUpdate,
};
use rubeapi::{FieldExt, Object};
use tonic::{Request, Response, Status};

use crate::store::Store;
use crate::StreamExt;

pub struct ServerImpl<S> {
    pub registry: Arc<Registry>,
    pub store: S,
}

impl<S> ServerImpl<S> {
    pub fn new(r: Arc<Registry>, s: S) -> Self {
        Self {
            registry: r,
            store: s,
        }
    }
}

#[protokit::grpc::async_trait]
impl<S: Store + Send + Sync + 'static> Api for ServerImpl<S> {
    async fn put(
        &self,
        req: tonic::Request<PutRequest>,
    ) -> Result<tonic::Response<PutResponse>, Status> {
        let req = req.into_inner();

        // let meta = req.metadata.require("metadata")?;
        // let name = meta.name.as_ref();
        // let namespace = meta.namespace.as_deref().unwrap_or("default");
        // let spec = req.spec.require("spec")?;
        // // let typ = spec.type_url.split_once("/").unwrap().1;
        //
        // let value = self
        //     .store
        //     .put(namespace, name, *spec.clone())
        //     .await
        //     .unwrap();

        let response = PutResponse::default();
            // .with_revision(value.version)
            // .with_spec(value.spec);

        Ok(Response::new(response))
    }

    async fn patch(
        &self,
        req: tonic::Request<PatchRequest>,
    ) -> Result<tonic::Response<PatchResponse>, Status> {
        let _req = req.into_inner();
        // let obj = self
        //     .get(Request::new(
        //         GetRequest::default()
        //             .with_metadata(req.metadata)
        //             .with_type(req.spec.qualified_name().to_string()),
        //     ))
        //     .await
        //     .unwrap()
        //     .into_inner();

        // TODO: take the database protobuf
        // Remove all fields based on the field mask,
        // Append the new protobuf.

        unimplemented!()
    }

    async fn get(
        &self,
        req: tonic::Request<GetRequest>,
    ) -> Result<tonic::Response<GetResponse>, Status> {
        let req = req.into_inner();
        let meta = req.metadata.require("metadata")?;
        let namespace = meta.namespace.as_deref().unwrap_or("default");
        let name = &meta.name;
        let value = self
            .store
            .get(namespace, name, &req.r#type, None)
            .await
            .unwrap();

        let response = match value {
            Some(value) => GetResponse::default()
                .with_revision(value.version)
                .with_spec(value.spec),
            None => GetResponse::default(),
        };

        Ok(Response::new(response))
    }

    async fn list(
        &self,
        req: tonic::Request<ListRequest>,
    ) -> Result<tonic::Response<ListResponse>, tonic::Status> {
        let req = req.into_inner();

        let namespace = req.namespace.as_deref().unwrap_or("default");
        let r#type = req.r#type.require("type")?;

        let list = self.store.list(r#type, namespace.into()).await.unwrap();

        let mut resp = ListResponse::default();
        resp.object = list.into_iter().map(|obj| {
            rubeapi::api::api::Object::default()
                .with_revision(obj.version)
                .with_metadata(obj.meta.into())
                .with_spec(obj.spec)
        }).collect();
        Ok(Response::new(resp))
    }

    async fn delete(
        &self,
        req: tonic::Request<DeleteRequest>,
    ) -> Result<tonic::Response<DeleteResponse>, Status> {
        let req = req.into_inner();

        let meta = req.metadata.require("metadata")?;
        let namespace = meta.namespace.as_deref().unwrap_or("default");

        let deleted = self
            .store
            .delete(&req.r#type, namespace, &meta.name)
            .await
            .unwrap();
        Ok(Response::new(
            DeleteResponse::default().with_revision(deleted.version),
        ))
    }

    type WatchStream = BoxStream<'static, Result<WatchResponse, Status>>;

    async fn watch(
        &self,
        req: tonic::Request<WatchRequest>,
    ) -> Result<tonic::Response<Self::WatchStream>, Status> {
        let req = req.into_inner();

        let namespace = req.namespace.require_take("namespace")?;
        let name = req.name.require_take("name")?;

        let stream = self
            .store
            .watch(&req.r#type, &namespace, &name, req.from_revision)
            .await
            .unwrap();

        Ok(Response::new(
            stream
                .map_ok(|item: Object<Option<Any>>| {
                    let mut response = WatchResponse::default();
                    match item.spec {
                        Some(v) => {
                            response.set_change_update(
                                WatchResponseUpdate::default()
                                    .with_revision(item.version)
                                    .with_spec(v),
                            );
                        }
                        None => {
                            response.set_change_delete(
                                WatchResponseDelete::default().with_revision(item.version),
                            );
                        }
                    }
                    response
                })
                .map_err(|e| Status::internal(e.to_string()))
                .boxed(),
        ))
    }

    async fn inspect(
        &self,
        _req: tonic::Request<Empty>,
    ) -> Result<tonic::Response<api::Registry>, Status> {
        let mut reg = api::Registry::default();
        for r in self.registry.messages.keys() {
            reg.add_type(r.to_string());
        }
        Ok(Response::new(reg))
    }
}
