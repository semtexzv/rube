use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::ops::{Deref, RangeBounds};
use std::path::Path;

use heed::{RoTxn, RwTxn};
use serde::{Deserialize, Serialize};

use crate::bin::{KeyFormat, ValueFormat};
use crate::Result;

pub type Database = heed::Database<KeyFormat, ValueFormat>;
// We use box everywhere for keys, might want to investigate Arc
pub type Index = BTreeMap<Cow<'static, [u8]>, Vec<LogKey>>;

pub trait IntoOwned {
    type Owned;
    fn into_owned(self) -> Self::Owned;
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct LogKey(u64, u16);

impl LogKey {
    #[inline]
    pub fn new(rev: u64, ent: u16) -> Self {
        Self(rev, ent)
    }
    #[inline]
    pub fn next(self) -> Self {
        // Each object can be modified only once in a given cycle (we can affect multiple objects)
        // in sequence, that's why we have the entity counter. But when we want to get the 'next' revision
        // in which an object could be modified, we reset the entity counter to 0
        Self(self.0 + 1, 0)
    }
    #[inline]
    pub fn rev(&self) -> u64 {
        self.0
    }
    #[inline]
    pub fn ent(&self) -> u16 {
        self.1
    }
}

#[inline]
pub fn logkey(r: u64) -> LogKey {
    LogKey::new(r, 0)
}

impl Debug for LogKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key")
            .field("rev", &self.rev())
            .field("ent", &self.ent())
            .finish()
    }
}

impl Display for LogKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.rev(), self.ent())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct LogValue<T> {
    pub del: bool,
    pub key: T,
    pub val: T,
}

pub type LogValueOwned = LogValue<Box<[u8]>>;
pub type LogValueRef<'a> = LogValue<&'a [u8]>;

impl IntoOwned for LogValueRef<'_> {
    type Owned = LogValueOwned;

    fn into_owned(self) -> Self::Owned {
        LogValueOwned {
            del: self.del,
            key: Box::from(self.key),
            val: Box::from(self.val),
        }
    }
}

pub type LogEntry<T> = (LogKey, LogValue<T>);
pub type LogEntryOwned = LogEntry<Box<[u8]>>;
pub type LogEntryRef<'a> = LogEntry<&'a [u8]>;

impl IntoOwned for LogEntryRef<'_> {
    type Owned = LogEntryOwned;

    fn into_owned(self) -> Self::Owned {
        (self.0, self.1.into_owned())
    }
}

pub struct State {
    /// heed root database
    pub(crate) hdb: Database,
    /// Index describing changes to individual keys
    pub(crate) idx: Index,

    /// Last revision #
    lastrev: u64,
}

impl State {
    pub fn range<'t, K, R>(
        &'t self,
        tx: &'t RoTxn,
        keys: K,
        revs: R,
    ) -> impl Iterator<Item=LogEntryRef<'t>> + 't
        where
            K: RangeBounds<&'t [u8]> + 't,
            R: RangeBounds<u64>,
    {
        let start = revs.start_bound().map(|v| logkey(*v));
        let end = revs.end_bound().map(|v| logkey(*v));

        let mut it = self.hdb.range(&tx, &(start, end)).unwrap();
        Self::entry_range(keys, it)
    }

    pub fn rev_range<'t, K, R>(
        &'t self,
        tx: &'t RoTxn,
        keys: K,
        revs: R,
    ) -> impl Iterator<Item=LogEntryRef<'t>> + 't
        where
            K: RangeBounds<&'t [u8]> + 't,
            R: RangeBounds<u64>,
    {
        let start = revs.start_bound().map(|v| logkey(*v));
        let end = revs.end_bound().map(|v| logkey(*v));

        let mut it = self.hdb.rev_range(&tx, &(start, end)).unwrap();
        Self::entry_range(keys, it)
    }

    pub fn entry_range<'t, R, T>(keys: R, mut it: T) -> impl Iterator<Item=LogEntryRef<'t>> + 't
        where
            R: RangeBounds<&'t [u8]> + 't,
            T: Iterator<Item=Result<(LogKey, LogValueRef<'t>), heed::Error>> + 't,
    {
        std::iter::from_fn(move || {
            while let Some(v) = it.next() {
                match v {
                    Ok((rev, val)) => {
                        if keys.contains(&val.key.deref().deref()) {
                            return Some((rev, val));
                        }
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
            None
        })
    }

    pub fn revs(&self, tx: &RoTxn<'_>, k: &[u8]) -> Option<&[LogKey]> {
        self.idx.get(k).map(|v| v.as_slice())
    }

    pub fn last<'a>(&'a self, tx: &'a RoTxn<'_>, k: &'a [u8]) -> Option<LogEntryRef<'a>> {
        let last = self.revs(tx, k)?.last()?;
        self.range(tx, k..=k, last.0..=last.0).next()
    }

    pub fn put(&mut self, mut wtx: RwTxn<'_, '_>, k: &[u8], v: &[u8]) -> Result<LogKey> {
        self.lastrev += 1;
        let ekey = LogKey::new(self.lastrev, 0);
        // Everything is appending to underlying b-tree store, so should be pretty fast
        self.hdb.append(
            &mut wtx,
            &ekey,
            &LogValueRef {
                key: k,
                val: v,
                del: false,
            },
        )?;

        let _ = wtx.commit()?;
        match self.idx.get_mut(k) {
            Some(v) => v.push(ekey),
            None => {
                self.idx.insert(Cow::Owned(k.to_vec()), vec![ekey]);
            }
        }

        Ok(ekey)
    }

    pub fn del(&mut self, mut wtx: RwTxn<'_, '_>, k: &[u8]) -> Result<LogKey> {
        self.lastrev += 1;
        let ekey = LogKey::new(self.lastrev, 0);
        self.hdb.append(
            &mut wtx,
            &ekey,
            &LogValueRef {
                key: k,
                val: &[],
                del: true,
            },
        )?;
        let _ = wtx.commit()?;

        match self.idx.get_mut(k) {
            Some(v) => v.push(ekey),
            None => {
                self.idx.insert(Cow::Owned(k.to_vec()), vec![ekey]);
            }
        }
        Ok(ekey)
    }
}

/// Key-Value change database.
pub struct KV {
    /// heed Env object
    pub(crate) env: heed::Env,
    /// Sate of the in-memory pieces
    pub(crate) state: State,
}

/// Read-only view of the database
pub struct KVRead<'k> {
    tx: RoTxn<'k>,
    st: &'k State,
}

impl<'t> KVRead<'t> {
    pub fn get(&self, k: LogKey) -> Option<LogValueRef> {
        self.st.hdb.get(&self.tx, &k).ok()?
    }

    pub fn versions<'a, 'b, K>(&'a self, k: K) -> impl Iterator<Item=(&[u8], LogKey)> + 'a
        where
            K: RangeBounds<Cow<'b, [u8]>> + 'b,
            'b: 'a
    {
        self.st
            .idx
            .range::<_, _>(k)
            .map(|(k, v)| (k.deref(), *v.last().unwrap()))
    }

    pub fn range<K, R>(&'t self, keys: K, revs: R) -> impl Iterator<Item=LogEntryRef<'t>> + 't
        where
            K: RangeBounds<&'t [u8]> + 't,
            R: RangeBounds<u64> + 't,
    {
        self.st.range(&self.tx, keys, revs)
    }

    pub fn rev_range<K, R>(&'t self, keys: K, revs: R) -> impl Iterator<Item=LogEntryRef<'t>> + 't
        where
            K: RangeBounds<&'t [u8]> + 't,
            R: RangeBounds<u64> + 't,
    {
        self.st.rev_range(&self.tx, keys, revs)
    }
}

pub struct KVWrite<'t> {
    pub tx: RwTxn<'t, 't>,
    pub st: &'t mut State,
}

impl<'t> KVWrite<'t> {
    pub fn range<K, R>(&'t self, keys: K, revs: R) -> impl Iterator<Item=LogEntryRef<'t>> + 't
        where
            K: RangeBounds<&'t [u8]> + 't,
            R: RangeBounds<u64> + 't,
    {
        self.st.range(&self.tx, keys, revs)
    }
    pub fn rev_range<K, R>(&'t self, keys: K, revs: R) -> impl Iterator<Item=LogEntryRef<'t>> + 't
        where
            K: RangeBounds<&'t [u8]> + 't,
            R: RangeBounds<u64> + 't,
    {
        self.st.rev_range(&self.tx, keys, revs)
    }

    pub fn put(mut self, k: &[u8], v: &[u8]) -> Result<LogKey, crate::Error> {
        self.st.put(self.tx, k, v)
    }

    pub fn del(mut self, k: &[u8]) -> Result<LogKey> {
        self.st.del(self.tx, k)
    }
}

impl KV {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        fs::create_dir_all(&path)?;
        let env = heed::EnvOpenOptions::new()
            // 10 Gigs ought to be enough for everybody
            .map_size(10 * 1024 * 1024 * 1024)
            .max_dbs(10)
            .open(path)?;

        let hdb = env.create_database(Some("_data"))?;
        let mut rev = 0;
        let mut idx: Index = BTreeMap::new();

        let tx = env.read_txn()?;
        for entry in hdb.iter(&tx)? {
            let (k, v): (LogKey, LogValueRef) = entry?;
            rev = k.rev();
            match idx.get_mut(v.key) {
                Some(v) => {
                    v.push(k);
                }
                None => {
                    idx.insert(Cow::Owned(v.key.to_vec()), vec![k]);
                }
            }
        }
        drop(tx);

        Ok(Self {
            env,
            state: State {
                hdb,
                lastrev: rev,
                idx,
            },
        })
    }

    pub fn read(&self) -> Result<KVRead> {
        Ok(KVRead {
            tx: self.env.read_txn()?,
            st: &self.state,
        })
    }

    pub fn write(&mut self) -> Result<KVWrite> {
        Ok(KVWrite {
            tx: self.env.write_txn()?,
            st: &mut self.state,
        })
    }
}
