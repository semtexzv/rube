use std::collections::Bound;
use std::fmt::Debug;
use std::ops::{Range, RangeBounds, RangeTo};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use heed::CompactionOption;
use heed::types::CowSlice;
use openraft::{AppData, AppDataResponse, EffectiveMembership, Entry, EntryPayload, HardState, LogId, LogState, Membership, RaftStorage, Snapshot, SnapshotMeta, StateMachineChanges, StorageError};
use openraft::testing::StoreBuilder;
use serde::{Deserialize, Serialize};
use tokio::fs::File;

use crate::bin::SerdeBin;

pub struct HeedStore {
    env: heed::Env,
    mdb: heed::Database<SerdeBin<MetaKey>, SerdeBin<MetaValue>>,
    ldb: heed::Database<SerdeBin<u64>, SerdeBin<Entry<DistReq>>>,
    stm: crate::DB,
}

impl HeedStore {
    pub fn new(db: crate::DB) -> Self {
        let env = db.kv.lock().unwrap().env.clone();
        Self {
            mdb: env.create_database(Some("_meta")).unwrap(),
            ldb: env.create_database(Some("_raft")).unwrap(),
            env,
            stm: db,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistReq {
    key: Box<[u8]>,
    val: Box<[u8]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistResp {
    Applied,
}

impl AppData for DistReq {}

impl AppDataResponse for DistResp {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MetaKey {
    HardState,
    LastApplied,
    LastPurged,
    Membership,
    Log(u64),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MetaValue {
    HardState(HardState),
    LastApplied(LogId),
    LastPurged(LogId),
    Members(EffectiveMembership),
    Log(Entry<DistReq>),
}

#[async_trait::async_trait]
impl RaftStorage<DistReq, DistResp> for HeedStore {
    type SnapshotData = File;

    async fn save_hard_state(&self, hs: &HardState) -> Result<(), StorageError> {
        println!("Write hard state: {:?}", hs);
        let mut wtx = self.env.write_txn().unwrap();
        self.mdb.put(&mut wtx, &MetaKey::HardState, &MetaValue::HardState(hs.clone())).unwrap();
        wtx.commit().unwrap();
        Ok(())
    }

    async fn read_hard_state(&self) -> Result<Option<HardState>, StorageError> {
        let mut rtx = self.env.read_txn().unwrap();
        let hs = self.mdb.get(&rtx, &MetaKey::HardState).unwrap();
        if let Some(MetaValue::HardState(hs)) = hs {
            return Ok(Some(hs));
        } else {
            return Ok(None);
        }
    }

    async fn get_log_state(&self) -> Result<LogState, StorageError> {
        let mut rtx = self.env.read_txn().unwrap();
        let last_purged = if let Some(MetaValue::LastPurged(p)) = self.mdb.get(&rtx, &MetaKey::LastPurged).unwrap() {
            Some(p)
        } else {
            None
        };

        let last_entry = if let Some((l, v)) = self.ldb.last(&rtx).unwrap() {
            Some(v.log_id)
        } else {
            last_purged
        };

        println!("LogState: {:?}. {:?}", last_purged, last_entry);
        Ok(LogState {
            last_purged_log_id: last_purged,
            last_log_id: last_entry,
        })
    }

    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + Send + Sync>(&self, range: RB) -> Result<Vec<Entry<DistReq>>, StorageError> {
        let mut rtx = self.env.read_txn().unwrap();

        let iter = self.ldb.range(&rtx, &range).unwrap();
        let mut result = vec![];
        for it in iter {
            let (k, v) = it.unwrap();
            println!("Read log: {:?}:{:?}", k, v);
            result.push(v);
        }
        eprintln!("Read log {:?}", result);
        Ok(result)
    }

    async fn append_to_log(&self, entries: &[&Entry<DistReq>]) -> Result<(), StorageError> {
        let mut wtx = self.env.write_txn().unwrap();
        for e in entries {
            println!("Append: {:?}", e);
            self.ldb.put(&mut wtx, &e.log_id.index, e).unwrap();
        }

        wtx.commit().unwrap();
        Ok(())
    }

    async fn delete_conflict_logs_since(&self, log_id: LogId) -> Result<(), StorageError> {
        let mut wtx = self.env.write_txn().unwrap();
        self.ldb.delete_range(&mut wtx, &(log_id.index..)).unwrap();
        wtx.commit().unwrap();
        Ok(())
    }

    async fn purge_logs_upto(&self, log_id: LogId) -> Result<(), StorageError> {
        let mut wtx = self.env.write_txn().unwrap();
        self.ldb.delete_range(&mut wtx, &(0..=log_id.index)).unwrap();
        self.mdb.put(&mut wtx, &MetaKey::LastPurged, &MetaValue::LastPurged(log_id)).unwrap();
        wtx.commit().unwrap();
        Ok(())
    }

    async fn last_applied_state(&self) -> Result<(Option<LogId>, Option<EffectiveMembership>), StorageError> {
        let mut tx = self.env.read_txn().unwrap();
        let applied = if let Some(MetaValue::LastApplied(log)) = self.mdb.get(&tx, &MetaKey::LastApplied).unwrap() {
            Some(log)
        } else { None };
        let membership = if let Some(MetaValue::Members(em)) = self.mdb.get(&tx, &MetaKey::Membership).unwrap() {
            Some(em)
        } else { None };

        Ok((applied, membership))
    }

    async fn apply_to_state_machine(&self, entries: &[&Entry<DistReq>]) -> Result<Vec<DistResp>, StorageError> {
        let mut res = vec![];
        for e in entries {
            println!("Apply: {:?}", e);
            match &e.payload {
                EntryPayload::Blank => {
                    let mut wtx = self.env.write_txn().unwrap();
                    self.mdb.put(&mut wtx, &MetaKey::LastApplied, &MetaValue::LastApplied(e.log_id)).unwrap();
                    wtx.commit().unwrap();
                }
                EntryPayload::Normal(d) => {
                    self.stm.put(&d.key, &d.val).await.unwrap();
                    let mut wtx = self.env.write_txn().unwrap();
                    self.mdb.put(&mut wtx, &MetaKey::LastApplied, &MetaValue::LastApplied(e.log_id)).unwrap();
                    wtx.commit().unwrap();
                }
                EntryPayload::Membership(mem) => {
                    let mut wtx = self.env.write_txn().unwrap();
                    self.mdb.put(&mut wtx, &MetaKey::Membership, &MetaValue::Members(EffectiveMembership {
                        log_id: e.log_id,
                        membership: mem.clone(),
                    })).unwrap();
                    self.mdb.put(&mut wtx, &MetaKey::LastApplied, &MetaValue::LastApplied(e.log_id)).unwrap();
                    wtx.commit().unwrap();
                }
            }
            res.push(DistResp::Applied);
        }
        Ok(res)
    }

    async fn build_snapshot(&self) -> Result<Snapshot<Self::SnapshotData>, StorageError> {
        Ok(Snapshot {
            meta: SnapshotMeta {
                last_log_id: None,
                snapshot_id: "".to_string(),
            },
            snapshot: Box::new(File::from_std(self.env.copy_to_path("/tmp/snapshot", CompactionOption::Enabled).unwrap())),
        })
    }

    async fn begin_receiving_snapshot(&self) -> Result<Box<Self::SnapshotData>, StorageError> {
        Ok(Box::new(tokio::fs::File::create("/tmp/snapshot").await.unwrap()))
    }

    async fn install_snapshot(&self, meta: &SnapshotMeta, snapshot: Box<Self::SnapshotData>) -> Result<StateMachineChanges, StorageError> {
        println!("Install snapshot: ");
        let senv = heed::EnvOpenOptions::new().open("/tmp/snapshot").unwrap();
        let sdb = senv.create_database::<CowSlice<u8>, CowSlice<u8>>(None).unwrap();
        let rtx = senv.read_txn().unwrap();

        let mut wtx = self.env.write_txn().unwrap();
        self.mdb.clear(&mut wtx).unwrap();

        for k in sdb.iter(&rtx).unwrap() {
            let (k, v) = k.unwrap();
            self.mdb.remap_types::<CowSlice<u8>, CowSlice<u8>>().put(&mut wtx, &k, &v).unwrap();
        };
        if let Some(MetaValue::LastApplied(l)) = self.mdb.get(&wtx, &MetaKey::LastApplied).unwrap() {
            Ok(StateMachineChanges {
                last_applied: Some(l),
                is_snapshot: true,
            })
        } else {
            Ok(StateMachineChanges {
                last_applied: None,
                is_snapshot: false,
            })
        }
    }

    async fn get_current_snapshot(&self) -> Result<Option<Snapshot<Self::SnapshotData>>, StorageError> {
        Ok(None)
    }
}



impl From<openraft::LogId> for raft::raft::LogId {
    fn from(v: openraft::LogId) -> Self {
        Self {
            term: v.term,
            index: v.index,
            ..Default::default()
        }
    }
}

impl Into<openraft::LogId> for raft::raft::LogId {
    fn into(self) -> openraft::LogId {
        openraft::LogId {
            term: self.term,
            index: self.index,
        }
    }
}

impl<S: Serialize + AppData> From<openraft::Entry<S>> for raft::raft::Entry {
    fn from(s: Entry<S>) -> Self {
        Self {
            log_id: Some(Box::new(s.log_id.into())),
            payload: storekey::serialize(&s.payload).unwrap(),
            ..Default::default()
        }
    }
}

impl<S: DeserializeOwned + AppData> TryInto<openraft::Entry<S>> for raft::raft::Entry {
    type Error = storekey::decode::Error;

    fn try_into(self) -> std::result::Result<Entry<S>, Self::Error> {
        Ok(Entry {
            log_id: (*self.log_id.unwrap()).into(),
            payload: storekey::deserialize(&self.payload)?,
        })
    }
}

impl From<openraft::SnapshotMeta> for raft::raft::SnapshotMeta {
    fn from(m: SnapshotMeta) -> Self {
        Self {
            last_log_id: m.last_log_id.map(|l| Box::new(l.into())),
            snapshot_id: m.snapshot_id,
            ..Default::default()
        }
    }
}

impl Into<openraft::SnapshotMeta> for raft::raft::SnapshotMeta {
    fn into(self) -> SnapshotMeta {
        SnapshotMeta {
            last_log_id: self.last_log_id.map(|v| (*v).into()),
            snapshot_id: self.snapshot_id,
        }
    }
}

type RaftClient = raft::raft::RaftClient<protokit::grpc::tonic::transport::Channel>;

pub struct Net {
    peers: RwLock<HashMap<NodeId, RaftClient>>,
}

#[async_trait::async_trait]
impl RaftNetwork<DistReq> for Net {
    async fn send_append_entries(&self, target: NodeId, rpc: AppendEntriesRequest<DistReq>) -> protokit::Result<AppendEntriesResponse> {
        let mut client = self.peers.read().unwrap()[&target].clone();

        let resp = client
            .append(Request::new(AppendRequest {
                term: rpc.term,
                leader_id: rpc.leader_id,
                prev_log_id: rpc.prev_log_id.map(Into::into).map(Box::new),
                entries: rpc.entries.into_iter().map(|v| {
                    v.into()
                }).collect(),
                leadr_commit: rpc.leader_commit.map(Into::into).map(Box::new),
                ..Default::default()
            }))
            .await?
            .into_inner();

        Ok(AppendEntriesResponse {
            term: resp.term,
            success: resp.success,
            conflict: resp.conflict,
        })
    }

    async fn send_install_snapshot(&self, target: NodeId, rpc: InstallSnapshotRequest) -> protokit::Result<InstallSnapshotResponse> {
        let mut client = self.peers.read().unwrap()[&target].clone();
        let resp = client
            .install(Request::new(InstallRequest {
                term: rpc.term,
                leader_id: rpc.leader_id,
                meta: Some(Box::new(rpc.meta.into())),
                offset: rpc.offset,
                data: rpc.data,
                done: rpc.done,
                ..Default::default()
            }))
            .await?
            .into_inner();

        Ok(InstallSnapshotResponse {
            term: resp.term
        })
    }

    async fn send_vote(&self, target: NodeId, rpc: VoteRequest) -> protokit::Result<VoteResponse> {
        let mut client = self.peers.read().unwrap()[&target].clone();

        let resp = client
            .send_vote(Request::new(raft::raft::VoteRequest {
                term: rpc.term,
                candidate_id: rpc.candidate_id,
                log_id: rpc.last_log_id.map(|v| Box::new(v.into())),
                ..Default::default()
            }))
            .await?
            .into_inner();

        Ok(VoteResponse {
            term: resp.term,
            vote_granted: resp.vote_granted,
            last_log_id: resp.last_log_id.map(|v| (*v).into()),
        })
    }
}

pub struct DistStore {
    raft: Raft<DistReq, DistResp, Net, HeedStore>,
    net: Arc<Net>,
    store: Arc<HeedStore>,
}



#[test]
pub fn test_mem_store() {
    let mut i = Arc::new(AtomicUsize::new(0));
    let bldr = || async {
        let ii = i.fetch_add(1, Ordering::SeqCst);
        std::fs::remove_dir_all(format!("/tmp/db-{}", ii));
        let mut db = crate::DB::new(format!("/tmp/db-{}", ii)).unwrap();
        HeedStore::new(db)
    };
    openraft::testing::Suite::test_all(bldr).unwrap()
}
