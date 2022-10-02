#![feature(bound_map)]
#![feature(btree_drain_filter)]

use std::borrow::Cow;
use std::cell::Cell;
use std::convert::identity;
use std::ops::{Deref, RangeBounds};
use std::path::Path;
use std::process::id;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use futures::future::BoxFuture;
use futures::stream::FuturesUnordered;
use futures::Stream;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;

use crate::kv::{IntoOwned, LogEntryOwned, LogEntryRef, LogKey, LogValueOwned, LogValueRef, KV};
use crate::watch::{SlicePtr, WatchGroup, Watcher};

mod bin;
pub mod kv;
pub mod watch;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("File IO: {0}")]
    IO(Box<std::io::Error>),
    #[error("Heed DB: {0}")]
    DB(Box<heed::Error>),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(Box::new(e))
    }
}

impl From<heed::Error> for Error {
    fn from(e: heed::Error) -> Self {
        Error::DB(Box::new(e))
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Database implementation with watchers
pub struct DB {
    kv: Arc<RwLock<KV>>,
    synced: Arc<Mutex<WatchGroup>>,
    unsynced: Arc<Mutex<Vec<Watcher>>>,
    blocked: UnboundedSender<(Watcher, LogEntryOwned)>,
    unblocked: Arc<Mutex<UnboundedReceiver<Watcher>>>,
}

/// stale watchers are totally blocked by their receivers
async fn blocked(
    mut rx: UnboundedReceiver<(Watcher, LogEntryOwned)>,
    unsync: Arc<Mutex<Vec<Watcher>>>,
) {
    let mut unordered: FuturesUnordered<
        BoxFuture<'static, Result<Watcher, tokio::time::error::Elapsed>>,
    > = FuturesUnordered::new();
    loop {
        tokio::select! {
            Some((mut watcher, it)) = rx.recv() => {
                // We spawn a future, in which we wait for actual unblocking
                // Everywhere else we assume the channel is not blocked, and if it is,
                // We shove it here to be waited upon.
                // Should get us minimal latency for writes, and a stable mechanism for catching
                // up that does not grow memory use with the queue size
                unordered.push(Box::pin(async move {
                    // TODO: make this configurable, 60 secs seems like a good default after which
                    // we drop the watcher.
                    tokio::time::timeout(Duration::from_secs(60), watcher.send(it)).await?;
                    Ok(watcher)
                }));
            }
            Some(watcher) = unordered.next() => {
                match watcher {
                    // If timeout didn't pass, we send the watcher to unsync queue
                    Ok(w) => {
                        unsync.lock().unwrap().push(w)
                    }
                    // The sending timed out, we drop the entry and watcher
                    Err(e) => {}
                }
            }
        }
    }
}

async fn unsync(
    unsync: Arc<Mutex<Vec<Watcher>>>,
    to_blocked: UnboundedSender<(Watcher, LogEntryOwned)>,
    to_unblocked: UnboundedSender<Watcher>,
    kv: Arc<RwLock<KV>>,
) {
    // We can have a situation here watcher keeps ping ponging between queues.
    // Maybe there is a solution, where we don't use a watchgroup here, but a straight up vector
    // Because on each iteration, we just go through all watchers, and either shove them into a blocked
    // queue, or back into the unblocked tree.
    loop {
        // Task waits for channels to clean up before trying to sync them
        tokio::time::sleep(Duration::from_millis(50)).await;
        let mut unsync = unsync.lock().unwrap();

        // We use drain to take the elements from vec, if we take all elements, then
        // We'll have a no-op for the fixup, if there are some left we'll have to copy them
        // to start of the vec.
        'outer: for watcher in unsync.drain(..) {
            let revisions = watcher.nextrev.get().rev() ..;
            let keys = watcher.key.0.deref() ..= watcher.key.1.deref();

            // Unlock the rwlock
            let kv = kv.read().unwrap();
            // Create a read tx
            let kvread = kv.read().unwrap();

            let mut it = kvread.range(keys, revisions);
            while let Some(entry) = it.next() {
                let rev = entry.0;
                match watcher.chan.try_send(entry.into_owned()) {
                    Ok(()) => {
                        // Just mark the next revision in the watcher, this is writing 16 bytes
                        watcher.nextrev.set(rev.next());
                    }
                    Err(TrySendError::Full(full_entry)) => {
                        // Drop the iterator to satisfy the borrow checker, the iterator might
                        // use the key reference from the watcher struct, and if it is dropped after the
                        // watcher, then we'd be pointing to already freed memory
                        drop(it);
                        // Blocked when sending last entry, send watcher to blocked queue
                        let _ = to_blocked.send((watcher, full_entry));
                        continue 'outer;
                    }
                    Err(TrySendError::Closed(closed)) => {
                        drop(it);
                        continue 'outer;
                    }
                }
            }
            // We have sucessfully processed all stored entries, sending to unblocked queue.
            drop(it);
            let _ = to_unblocked.send(watcher);
        }
    }
}

impl DB {
    pub fn new(
        path: impl AsRef<Path>,
    ) -> Result<(Self, BoxFuture<'static, ()>, BoxFuture<'static, ()>)> {
        // Using unbounded channes for sending watchers. Their actual size is bounded by number of
        // watchers in the system, since they are never cloned, just copied around.
        let (synctx, syncrx) = tokio::sync::mpsc::unbounded_channel::<Watcher>();
        let (blocktx, blockrx) = tokio::sync::mpsc::unbounded_channel::<(Watcher, LogEntryOwned)>();

        // Vector to represent unsynchronized, since on each iteration of the unsync loop
        // we attempt to just process all elements in this list. The better solution might be
        // a lockfree list of segments, where each segment is protected by mutex.
        let unsynced = Arc::new(Mutex::new(Vec::new()));
        let kv = Arc::new(RwLock::new(kv::KV::new(path)?));

        let do_stale = Box::pin(blocked(blockrx, unsynced.clone()));
        let do_unsynced = Box::pin(unsync(
            unsynced.clone(),
            blocktx.clone(),
            synctx,
            kv.clone(),
        ));

        Ok((
            Self {
                kv,
                synced: Arc::new(Mutex::new(WatchGroup::new())),
                unsynced,
                blocked: blocktx,
                unblocked: Arc::new(Mutex::new(syncrx)),
            },
            do_stale,
            do_unsynced,
        ))
    }

    pub fn first(
        &self,
        k: impl AsRef<[u8]>,
        since: impl RangeBounds<u64>,
    ) -> Result<Option<LogEntryOwned>> {
        let k = k.as_ref();
        let kv = self.kv.read().unwrap();
        let read = kv.read().unwrap();
        let mut it = read.range(&k ..= &k, since);
        Ok(it.next().map(IntoOwned::into_owned))
    }

    pub fn last(
        &self,
        k: impl AsRef<[u8]>,
        until: impl RangeBounds<u64>,
    ) -> Result<Option<LogEntryOwned>> {
        let k = k.as_ref();
        let kv = self.kv.read().unwrap();
        let read = kv.read().unwrap();
        let mut it = read.rev_range(&k ..= &k, until);
        Ok(it.next().map(IntoOwned::into_owned))
    }

    pub fn list<'a, K>(&'a self, k: K) -> Vec<LogEntryOwned>
    where
        K: RangeBounds<Cow<'a, [u8]>> + 'a,
    {
        let read = self.kv.read().unwrap();
        let read = read.read().unwrap();

        let versions = read.versions(k);

        let v = versions
            .map(|(_, k)| read.get(k).map(|v| (k, v)))
            .filter_map(identity)
            .map(|v| v.into_owned())
            .collect();

        drop(read);
        v
    }

    pub fn put(&self, k: impl AsRef<[u8]>, v: impl AsRef<[u8]>) -> Result<LogEntryOwned> {
        let k = k.as_ref();
        let v = v.as_ref();

        // First write to storage
        let lk = {
            let mut kv = self.kv.write().unwrap();
            let mut write = kv.write()?;
            write.put(&k, &v)?
        };
        // Update watchers and return owned (ref lifetime is tied to tx)
        self.do_append((
            lk,
            LogValueRef {
                key: k,
                val: v,
                del: false,
            },
        ))
    }

    pub fn delete(&self, k: impl AsRef<[u8]>) -> Result<LogEntryOwned> {
        let k = k.as_ref();

        // First write to storage
        let lk = {
            let mut kv = self.kv.write().unwrap();
            let mut write = kv.write()?;
            write.del(&k)?
        };

        self.do_append((
            lk,
            LogValueRef {
                key: k,
                val: &[],
                del: true,
            },
        ))
    }

    fn do_append(&self, entry: LogEntryRef) -> Result<LogEntryOwned> {
        let mut synced = self.synced.lock().unwrap();
        {
            // Restore all unblocked watchers so we don't miss any notifications
            let mut unblocked = self.unblocked.lock().unwrap();
            while let Ok(it) = unblocked.try_recv() {
                synced.insert(it);
            }
        }

        let mut blocked = smallvec::SmallVec::<[_; 4]>::new();

        // Send messages to all free watchers, marking ones that are blocked
        for (k, w) in synced.overlaps(&SlicePtr::from(entry.1.key.as_ref())) {
            match w.chan.try_send(entry.clone().into_owned()) {
                Ok(()) => {}
                Err(TrySendError::Full(e)) => blocked.push((*k, e, true)),
                Err(TrySendError::Closed(e)) => blocked.push((*k, e, false)),
            }
        }

        // Move blocked watchers to blocked queue
        for (k, e, open) in blocked.into_iter() {
            if let Some(w) = synced.watchers.remove(&k.0, &k.1) {
                if open {
                    let _ = self.blocked.send((w, e));
                }
            }
        }

        Ok(entry.into_owned())
    }

    pub fn watch(
        &self,
        k1: impl Into<Box<[u8]>>,
        k2: Option<impl Into<Box<[u8]>>>,
        from: u64,
    ) -> impl Stream<Item = LogEntryOwned> {
        let k1 = k1.into();
        let k2 = k2.map(|v| v.into()).unwrap_or_else(|| k1.clone());

        let (tx, rx) = tokio::sync::mpsc::channel::<LogEntryOwned>(4);
        let mut unsync = self.unsynced.lock().unwrap();

        unsync.push(Watcher {
            chan: tx,
            key: (k1, k2),
            nextrev: Cell::new(LogKey::new(from, 0)),
        });

        tokio_stream::wrappers::ReceiverStream::new(rx)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn setup() -> DB {
        let td = tempdir::TempDir::new("a").unwrap();
        let (db, t1, t2) = DB::new(td.path().join("a")).unwrap();
        tokio::spawn(t1);
        tokio::spawn(t2);
        db
    }

    // #[tokio::test]
    // async fn test_basic() {
    //     let db = setup().await;
    //     db.put(b"Hello", b"World").await.unwrap();
    //
    //     assert_eq!(db.first(b"Hello", 1..).await.unwrap(), Some(LogEntry {
    //         rev: LogKey::new(1, 0),
    //         val: LogValueOwned {
    //             key: Box::from(&b"Hello"[..]),
    //             val: Box::from(&b"World"[..]),
    //             del: false,
    //         },
    //     }));
    //     assert_eq!(db.first(b"Hello", 2..).await.unwrap(), None);
    // }
    //
    // #[tokio::test]
    // async fn test_watch() {
    //     let db = setup().await;
    //     let mut w1 = db.watch(*b"Hello", Some(*b"Hello"), 0);
    //     let mut w2 = db.watch(*b"Hellw", Some(*b"Hellw"), 1);
    //
    //     db.put(b"Hello", "World").await.unwrap();
    //     db.put(b"Hellw", "World").await.unwrap();
    //
    //     let n = w1.next().await;
    //     assert_eq!(n, Some(LogEntry {
    //         rev: logkey(1),
    //         val: LogValueOwned {
    //             key: Box::new(*b"Hello"),
    //             val: Box::new(*b"World"),
    //             del: false,
    //         },
    //     }));
    //
    //     let n = w2.next().await;
    //     assert_eq!(n, Some(LogEntry {
    //         rev: logkey(2),
    //         val: LogValueOwned {
    //             key: Box::new(*b"Hellw"),
    //             val: Box::new(*b"World"),
    //             del: false,
    //         },
    //     }));
    // }
}
