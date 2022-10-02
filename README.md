# Rube

Generalized control-loop architecture for whatever you need.

Ideas:

1. Protobufs for data definition, storage, and transfer format
2. Textprotos for user-modifiable configuration. If you need templating, use a DSL. Protos
   specify everything that the client expects and will accept.
3. gRPC for communication between individual nodes. TODO: zstd compression and tls encryption.
4. Sqlite/etcd/rocksdb for storage. Configurable, not all variants are yet implemented.
5. Rust for writing custom controllers. Controllers in form of `fn(state) -> internal change + cluster api calls`
6. Federated approach to nodes. There is a centralized configuration, a master source of truth.
   Apart from that, each node should have a read-only copy of resources it manages. The definition
   of that resource is read-only, but its status is not. TODO: implement state storage.

## Architecture

Largely inspired by kubernetes. A single/cluster of apiservers, that provide access to state 
store implemented as:
1. Built-in sqlite/rocksdb database - Single node
2. External etcd
3. Raft over rocksdb for cluster ? (TODO: investigate/implement)
4. Maybe TiDB ?

Per-node process (rubelet), that hosts local state store and runs local controllers.
It connects to the apiserver using client defined in `rubeapi` crate, watches for resources
TODO: Filters, maybe using metadata annotations? something like:

```prototext
metadata {
  name: "node-service"
  labels: [
    { key: "node" value: "worker-1" }
  ]
}
```

Actual resources managed by the cluster 
### Example:

Controller that manages systemd services would accept following definition.

```prototext
# proto-file: rube/api.proto
# proto-message: Object

metadata {
  namespace: "system"
  name: "indexer"
}

spec {
  [rube/api.systemd.Unit] {
    description: "hey"
    doc_url: "file://"
    wants: "graphical.target"
    service {
      title: "Don't run this without consulting me firtst"
      type: FORKING
      pid_file: "/blablabla/pid"
      exec {
        start: "echo \"Hey You!\""
      }
      time {
        restart: 60
        timeout: 30
        limit: 3600
        watchdog: 60
      }
      restart: ON_FAILURE
      status {
        success: [0, 1, 2]
        prevent_restart: [66]
      }
      oom_policy: KILL
    }
    enabled: false
  }
}
```

User should be able to create the service by running:

```shell
ructl put < put.textproto 
```

Delete by using

```shell
ructl delete 'type = "api.systemd.Unit", metadata { namespace: "system", name: "indexer" }'
```

or by shorthand:

```shell
ructl delete -t api.systemd.Unit -n system indexer
```

Watch for changes, global
```shell
ructl watch -t api.systemd.Unit
```
Per-namespace
```shell
ructl watch -t api.systemd.Unit -n system
```
Single object:
```shell
ructl watch -t api.systemd.Unit -n system indexer
```
Results in following events:
```prototext
update {
  metadata {
    
  }
  spec {
    
  }
}

...

update {
  metadata {

  }
  spec {

  }
}

...

delete {
  metadata {

  }
  spec {

  }
}

```


## HA
Current design has no guarantees about HA. In order to acheive this, we will need 
to implement data store that allows us to replicate state using RAFT.

#### Component level
One interesting possibility is implementing in-memory raft for different controllers.
