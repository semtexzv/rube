
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use anyhow::Result;
use futures::{Stream, StreamExt};
use protokit::binformat::{Decodable, Encodable};
use protokit::reflect::Registry;
use protokit::grpc::tonic::transport::Channel;
use protokit::grpc::tonic::{Request, Status};

use crate::api::api::{
    ApiClient, DeleteRequest, GetRequest, ListRequest, Meta, PutRequest, WatchRequest,
    WatchResponseOneOfChange,
};

mod gen;
mod controllers;

pub use gen::*;

// include!(concat!(env!("OUT_DIR"), "/mod.rs"));
// pub const DESCRIPTOR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/descriptor.bin"));
pub const DESCRIPTOR: &[u8] = include_bytes!("./gen/descriptor.bin");

pub fn registry() -> Registry {
    let mut reg = Registry::default();
    self::register_types(&mut reg);
    reg
}

fn missing(n: &'static str) -> impl FnOnce() -> Status {
    move || Status::invalid_argument(format!("missing `{n}`"))
}

pub trait FieldExt<'a, T> {
    fn require(&'a self, name: &'static str) -> Result<&'a T, Status>;
    fn require_take(self, name: &'static str) -> Result<T, Status>;
}

impl<'a, T> FieldExt<'a, T> for Option<T> {
    fn require(&'a self, name: &'static str) -> Result<&'a T, Status> {
        self.as_ref().ok_or_else(missing(name))
    }

    fn require_take(mut self, name: &'static str) -> Result<T, Status> {
        self.take().ok_or_else(missing(name))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct ObjectMeta {
    pub namespace: Box<str>,
    pub name: Box<str>,
}

impl From<Box<Meta>> for ObjectMeta {
    fn from(meta: Box<Meta>) -> Self {
        Self {
            namespace: meta
                .namespace
                .unwrap_or("default".to_string())
                .into_boxed_str(),
            name: meta.name.into_boxed_str(),
        }
    }
}

impl Into<Meta> for ObjectMeta {
    fn into(self) -> Meta {
        Meta {
            namespace: Some(String::from(self.namespace)),
            name: String::from(self.name),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Object<T> {
    pub meta: ObjectMeta,
    pub version: u64,
    pub spec: T,
}

impl<T> Deref for Object<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl<T> DerefMut for Object<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spec
    }
}

pub fn typeurl(qualified_name: &str) -> String {
    format!("rube/{qualified_name}")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event<T> {
    Modified(Object<T>),
    Deleted(Object<T>),
}

#[derive(Debug, Clone)]
pub struct Client {
    pub registry: Arc<Registry>,
    pub client: ApiClient<Channel>,
}

impl Client {
    pub async fn put<R: Decodable + Encodable + Default>(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
        obj: R,
    ) -> Result<Object<R>> {
        let req = PutRequest::default();
            // .with_object(rube::)
            // .with_metadata(Meta {
            //     namespace: Some(namespace.into()),
            //     name: name.into(),
            //     ..Default::default()
            // })
            // .with_spec(Any::pack(&obj)?);
        let resp = self.client.put(req).await?.into_inner();
        Ok(Object {
            meta: resp.metadata.require_take("metadata")?.into(),
            version: resp.revision,
            spec: resp.spec.require_take("spec")?.unpack()?,
        })
    }

    pub async fn delete<R: Encodable + Decodable + Default>(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Object<R>> {
        let req = DeleteRequest::default()
            .with_metadata(Meta {
                namespace: Some(namespace.into()),
                name: name.into(),
                ..Default::default()
            })
            .with_type(R::default().qualified_name().to_string());

        let resp = self.client.delete(req).await?.into_inner();
        Ok(Object {
            meta: resp.metadata.require_take("metadata")?.into(),
            version: resp.revision,
            spec: resp.spec.require_take("spec")?.unpack()?,
        })
    }

    pub async fn get<R: Decodable + Encodable + Default>(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Object<R>> {
        let req = GetRequest::default()
            .with_type(typeurl(R::default().qualified_name()))
            .with_metadata(Meta {
                namespace: Some(namespace.into()),
                name: name.into(),
                ..Default::default()
            });
        let response = self.client.get(req).await?.into_inner();
        let spec = response.spec.require_take("spec")?;
        let obj = spec.unpack::<R>()?;

        Ok(Object {
            meta: response.metadata.require_take("metadata")?.into(),
            version: response.revision,
            spec: obj,
        })
    }

    pub async fn list<R: Decodable + Encodable + Default>(
        &mut self,
        namespace: impl Into<String>,
    ) -> Result<Vec<Object<R>>> {
        let req = ListRequest::default()
            .with_namespace(namespace.into())
            .with_type(R::default().qualified_name().into());

        let list = self.client.list(req).await?.into_inner();
        Ok(list
            .object
            .into_iter()
            .map(|o| {
                Ok(Object {
                    meta: o.metadata.require_take("metadata")?.into(),
                    version: o.revision,
                    spec: o.spec.require_take("spec")?.unpack()?,
                })
            })
            .collect::<Result<Vec<_>>>()?)
    }

    pub async fn watch<R: Decodable + Encodable + Default>(
        &mut self,
        namespace: Option<impl Into<String>>,
        name: Option<impl Into<String>>,
        version: Option<u64>,
    ) -> Result<impl Stream<Item = Result<Event<R>>>> {
        let mut req = WatchRequest::default().with_type(typeurl(R::default().qualified_name()));

        if let Some(namespace) = namespace {
            req.set_namespace(namespace.into());
            if let Some(name) = name {
                req.set_name(name.into());
                if let Some(version) = version {
                    req.set_from_revision(version);
                }
            }
        }

        let request = Request::new(req);
        let stream = self.client.watch(request).await?;
        let stream = stream.into_inner();
        Ok(stream.map(|v| {
            let v = v.unwrap();
            let mut obj = R::default();
            match v.change {
                WatchResponseOneOfChange::Update(up) => {
                    let spec = up.spec.require_take("spec")?;
                    let meta = up.metadata.require_take("metadata")?;

                    protokit::binformat::decode_into(&spec.value, &mut obj)?;
                    Ok(Event::Modified(Object {
                        meta: ObjectMeta::from(meta),
                        version: up.revision,
                        spec: obj,
                    }))
                }
                WatchResponseOneOfChange::Delete(del) => {
                    let spec = del.spec.require_take("spec")?;
                    let meta = del.metadata.require_take("metadata")?;
                    protokit::binformat::decode_into(&spec.value, &mut obj)?;
                    Ok(Event::Deleted(Object {
                        meta: ObjectMeta::from(meta),
                        version: del.revision,
                        spec: obj,
                    }))
                }
                WatchResponseOneOfChange::Unknown(o) => {
                    panic!("Unknown event variant: {o:?}")
                }
            }
        }))
    }
}
