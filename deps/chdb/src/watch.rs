use std::cell::Cell;
use std::cmp::Ordering;
use std::ops::Deref;
use std::ptr::NonNull;

use irbtree::{Overlaps, RBTree};

use crate::{LogEntryOwned, LogKey};


#[derive(Debug)]
pub struct Watcher {
    pub chan: tokio::sync::mpsc::Sender<LogEntryOwned>,
    pub key: (Box<[u8]>, Box<[u8]>),
    pub nextrev: Cell<LogKey>,
}

impl Watcher {
    pub async fn send(&mut self, entry: LogEntryOwned) {
        self.chan.send(entry.clone()).await.unwrap();
        self.nextrev.set(entry.0.next());
    }
}

/// SlicePtrs point to watchers Box<[u8]> buffers, and the lifetime of these pointers
/// is directly tied to watchers, so there should be no problems(We only move the watchers, never clone/copy)
/// and the keys are stored as pointers that we're not mutating.
#[derive(Clone, Copy, Debug)]
pub struct SlicePtr(NonNull<[u8]>);

impl From<&[u8]> for SlicePtr {
    fn from(s: &[u8]) -> Self {
        SlicePtr(NonNull::from(s.deref()))
    }
}

impl Deref for SlicePtr {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl PartialEq<Self> for SlicePtr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.as_ref().eq(other.0.as_ref()) }
    }
}

impl Eq for SlicePtr {}
impl PartialOrd<Self> for SlicePtr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe { self.0.as_ref().partial_cmp(other.0.as_ref()) }
    }
}
impl Ord for SlicePtr {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe { self.0.as_ref().cmp(other.0.as_ref()) }
    }
}

unsafe impl Send for SlicePtr {}

#[derive(Debug, Default)]
pub struct WatchGroup {
    pub watchers: RBTree<SlicePtr, Watcher>,
}

impl WatchGroup {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, w: Watcher) {
        self.watchers.insert(
            SlicePtr::from(w.key.0.deref()),
            SlicePtr::from(w.key.1.deref()),
            w,
        );
    }

    pub fn overlaps<'a>(&'a mut self, k: &'a SlicePtr) -> Overlaps<'a, SlicePtr, Watcher> {
        self.watchers.overlaps(k)
    }
}
