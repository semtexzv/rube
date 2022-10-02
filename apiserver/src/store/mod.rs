use std::future::Future;

use anyhow::Result;
use futures::Stream;
use protokit::grpc::LocalBoxFuture;
use protokit::types::any::Any;
use rubeapi::Object;

pub mod chdb;

enum WatchSpec<'s> {
    All {
        r#type: &'s str,
    },
    Namespace {
        r#type: &'s str,
        namespace: &'s str,
    },
    Object {
        r#type: &'s str,
        namespace: &'s str,
        name: &'s str,
    },
}

#[async_trait::async_trait]
pub trait Store {
    /// Get an object at particular version, latest if none
    async fn get(
        &self,
        r#type: &str,
        namespace: &str,
        name: &str,
        version: Option<u64>,
    ) -> Result<Option<Object<Any>>>;
    /// List all objects of a type, in single, or all namespaces
    async fn list(&self, r#type: &str, namespace: Option<&str>) -> Result<Vec<Object<Any>>>;
    /// Write an object into the database, returning modified version
    async fn put(&self, namespace: &str, name: &str, spec: Any) -> Result<Object<Any>>;
    /// Delete the object from database. Returning only the deleted object metadata.
    async fn delete(&self, r#type: &str, namespace: &str, name: &str) -> Result<Object<()>>;

    type WatchStream: Stream<Item = Result<Object<Option<Any>>>> + Send;
    /// Watch for either
    async fn watch(
        &self,
        r#type: &str,
        namespace: &str,
        name: &str,
        from_revision: Option<u64>,
    ) -> Result<Self::WatchStream>;
}
