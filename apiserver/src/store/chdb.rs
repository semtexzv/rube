use std::borrow::Cow;
use std::ops::Deref;

use anyhow::{anyhow, format_err};
use chdb::kv::LogEntryOwned;
use futures::stream::BoxStream;
use futures::{SinkExt, StreamExt};
use protokit::reflect::AnyMessage;
use protokit::types::any::Any;
use rubeapi::{Object, ObjectMeta};
use tokio::task::JoinHandle;
use tracing_subscriber::filter::FilterExt;
use rubeapi::api::api::Meta;

use crate::store::Store;

pub fn list_key(
    namespace: Option<impl AsRef<str>>,
    typ: impl AsRef<str>,
) -> String {
    let namespace = namespace.as_ref().map(|v| v.as_ref()).unwrap_or("default");
    let typ = typ.as_ref();
    format!("/objects/{namespace}/{typ}/")
}

pub fn object_key(
    namespace: Option<impl AsRef<str>>,
    typ: impl AsRef<str>,
    name: impl AsRef<str>,
) -> String {
    let namespace = namespace.as_ref().map(|v| v.as_ref()).unwrap_or("default");
    let typ = typ.as_ref();
    let name = name.as_ref();
    format!("/objects/{namespace}/{typ}/{name}")
}

pub fn next_key(mut v: Vec<u8>) -> Vec<u8> {
    while v.len() > 0 {
        match v.last_mut() {
            Some(255) => {
                v.pop();
            },
            Some(x) => {
                *x = *x + 1;
                break;
            }
            None => return v,
        }
    }
    return v
}

pub fn parse_object_key(key: &str) -> Option<(&str, &str, &str)> {
    println!("Parsing object key: {:?}", key);
    let (_slash, key) = key.split_once('/')?;
    let (_objects, key) = key.split_once('/')?;
    let (namespace, key) = key.split_once('/')?;
    let (typ, key) = key.split_once('/')?;
    let name = key.split_once('/').map(|v| v.0).unwrap_or(key);

    Some((namespace, typ, name))
}

fn spec_type_name(spec: &Any) -> anyhow::Result<&str> {
    let r#type = spec
        .type_url
        .split_once('/')
        .ok_or(anyhow!("Wrong type "))?
        .1;
    Ok(r#type)
}

pub struct CHDBStore {
    db: chdb::DB,
    blocked: JoinHandle<()>,
    stale: JoinHandle<()>,
}

impl CHDBStore {
    pub fn new() -> Self {
        let (db, blocked, stale) = chdb::DB::new("tmp-db").unwrap();

        CHDBStore {
            db,
            blocked: tokio::spawn(blocked),
            stale: tokio::spawn(stale),
        }
    }
}

#[inline]
pub fn log_to_spec(type_url: String, entry: LogEntryOwned) -> anyhow::Result<Any> {
    Ok(Any::set(type_url, entry.1.val.into_vec()))
}

#[inline]
pub fn log_to_object(
    meta: ObjectMeta,
    type_url: String,
    entry: chdb::kv::LogEntryOwned,
) -> anyhow::Result<Object<Any>> {
    println!("Parsing: {:?}", entry.1.val);
    Ok(Object::<Any> {
        meta,
        version: entry.0.rev(),
        spec: Any::set(type_url, entry.1.val.into_vec()),
    })
}

#[async_trait::async_trait]
impl Store for CHDBStore {
    async fn get(
        &self,
        r#type: &str,
        namespace: &str,
        name: &str,
        version: Option<u64>,
    ) -> anyhow::Result<Option<Object<Any>>> {
        let k = object_key(Some(namespace), r#type, name);

        let entry = self.db.first(k.as_bytes(), version.unwrap_or(0) ..);
        match entry {
            Ok(Some(entry)) => {
                let meta: ObjectMeta = ObjectMeta {
                    namespace: namespace.to_string().into_boxed_str(),
                    name: name.to_string().into_boxed_str(),
                };

                return Ok(Some(log_to_object(
                    meta,
                    format!("rube/{}", r#type),
                    entry,
                )?));
            }
            Ok(None) => return Ok(None),
            Err(e) => return Err(format_err!("Db error: {:?}", e)),
        }
    }

    async fn list(
        &self,
        r#type: &str,
        namespace: Option<&str>,
    ) -> anyhow::Result<Vec<Object<Any>>> {
        let k1 = list_key(namespace, r#type).into_bytes();
        let k2 = next_key(k1.clone());

        let k1 = Cow::Borrowed(k1.as_slice());
        let k2 = Cow::Borrowed(k2.as_slice());
        let items = self.db.list(k1..k2);
        unsafe {
            items.into_iter().map(|(k, v)| {
                let (ns, typ, n) = parse_object_key(std::str::from_utf8_unchecked(v.key.deref())).unwrap();
                let meta = ObjectMeta {
                    namespace: ns.to_string().into_boxed_str(),
                    name: n.to_string().into_boxed_str(),
                };
                log_to_object(meta, format!("rube/{}", typ), (k, v))
            }).collect()
        }

    }

    async fn put(&self, namespace: &str, name: &str, spec: Any) -> anyhow::Result<Object<Any>> {
        let k = object_key(Some(namespace), spec_type_name(&spec)?, name);
        let obj = self.db.put(k.as_bytes(), spec.value).unwrap();
        let meta: ObjectMeta = ObjectMeta {
            namespace: namespace.to_string().into_boxed_str(),
            name: name.to_string().into_boxed_str(),
        };
        log_to_object(meta, spec.type_url, obj)
    }

    async fn delete(
        &self,
        r#type: &str,
        namespace: &str,
        name: &str,
    ) -> anyhow::Result<Object<()>> {
        let k = object_key(Some(namespace), r#type, name);
        let entry = self.db.delete(k).unwrap();
        let meta: ObjectMeta = ObjectMeta {
            namespace: namespace.to_string().into_boxed_str(),
            name: name.to_string().into_boxed_str(),
        };
        Ok(Object {
            meta,
            version: entry.0.rev(),
            spec: (),
        })
    }

    type WatchStream = BoxStream<'static, anyhow::Result<Object<Option<Any>>>>;

    async fn watch(
        &self,
        r#type: &str,
        namespace: &str,
        name: &str,
        from_revision: Option<u64>,
    ) -> anyhow::Result<Self::WatchStream> {
        let k = object_key(Some(namespace), r#type, name);
        let k = k.into_bytes();

        let watch = self
            .db
            .watch(k.clone(), Some(k.clone()), from_revision.unwrap_or(0));

        Ok(watch
            .map(|entry: LogEntryOwned| {
                let key = unsafe { std::str::from_utf8_unchecked(entry.1.key.deref()) };
                let (namespace, r#type, name) = parse_object_key(key).unwrap();
                let meta: ObjectMeta = ObjectMeta {
                    namespace: namespace.to_string().into_boxed_str(),
                    name: name.to_string().into_boxed_str(),
                };

                let ret = if entry.1.del {
                    Ok(Object {
                        meta,
                        version: entry.0.rev(),
                        spec: None,
                    })
                } else {
                    Ok(Object {
                        meta,
                        version: entry.0.rev(),
                        spec: Some(log_to_spec(format!("rube/{}", r#type), entry)?),
                    })
                };
                println!("Ret: {:?}", ret);
                ret
            })
            .boxed())
    }
}

#[test]
fn test_obj_key() {
    let (a, b, c) = parse_object_key("/objects/a/b/c/").unwrap();
    assert_eq!(a, "a");
    assert_eq!(b, "b");
    assert_eq!(c, "c");
}
