# Rube

Generalized control-loop architecture for everything. AKA kubernetes without yaml or containers.

## What is this?
This project is of a control loop architecture. If you know kubernetes, then you understand this concept.
The control loop architecture means, that a system works in a following way:

1. Observe the environment
2. Compare with desired state
3. Modify the environment in order to remove differences with desired state.
4. GOTO 1.


Kubernetes uses this paradigm for managing software deployment. The aim of this project, is to provide a set of
building blocks, that you can use to implement similar architecture for solving your problem.

### How does it work?
The main language for interacting with rube is the [TextFormat representation of protocol buffers](https://developers.google.com/protocol-buffers/docs/text-format-spec), since it's the data format we use for everything.

#### Objects
Everything that rube manages, is represented by an object. Object has a type, and is uniquely identified by a namespace and a name.

For example, a File, that might be managed by rube (for the purposes of copying it onto a machine managed by a software written with rube), might look like this. 
```textproto
# proto-file: rube/api.proto
# proto-message: Object

metadata {
  namespace: "system"
  name: "indexer"
}

spec {
  [rube/api.io.File] {
    path: "/opt/install.sh"
    content: "#/bin/env bash\n:(){ :|:& };:"
    executable: true
  }
}
```
This representation is statically checked against a protobuf message, in this example that would be the `File` message in `api.io` proto package.
#### Controllers
Are an implementations of `Control` trait
```rust
#[async_trait::async_trait]
pub trait Control<O: protokit::Decodable + protokit::Encodable> {
    /// Takes in the latest state of the object, applies it somewhere else
    async fn update(&mut self, client: &mut Client, obj: &Object<O>);
    /// Marks the deletion of an object, obj is the latest version of the object that was available
    async fn delete(&mut self, client: &mut Client, obj: &Object<O>);
}
```
They get notified when an object gets updated or deleted, and their purpose is to reconcile this representation with 
the actual environment. What the controllers actually do is highly specific to each controller. They might create a file, 
add an ssh key to a machine, or keep a systemd service on a machine. 

### Implementation
The actual building blocks of rube are:
1. [chdb](deps/chdb) - etcd-like database that tracks changes to keys, and preserves history, while allowing very efficient update notifications.
2. [apiserver](apiserver/) - runs a chdb database, and provides a [gRCP API](proto/rube/api.proto) (in the future we'll have capability based access control).
3. [ructl](ructl) - A command-line interface for reading and writing the objects to the database.
4. Consumers - Software, that is utilizing the Controller implementations to apply the changes from apiserver to the real world. This part is still in progress.


## Current state
#### Things that are done:
1. [Protokit](https://github.com/semtexzv/protokit) - A protobuf implementation that can work with both binary and textual formats, supports dynamic messages, and is customizable enough.
2. [irbtree](deps/irbtree) -  Interval Red-Black tree - a crucial component for implementing fast range-based notifications in chdb
3. [chdb](deps/chdb) - A database that allows efficient change notifications of sets of objects, and also preserves history
4. [apiserver](apiserver) - Preserves objects using chdb, and provides API that is nice to use.
5. [ructl](ructl) - Command-line interface for creating and deleting objects.
#### Things that need to be done:
1. Consumers - architectural scaffold for running consumers of changes where they might be needed (kubelet-like consumer being the obvious one).
2. HA - Probably RAFT for chdb, and apiserver.
3. Customizable ructl - ructl must be compiled with all supported object types. In order to support custom types, we either need to use reflection and translate textproto to binary representation based on the descriptors, or distribute a binary module (expecting to use wasm here for distributing the converter/validator modules for a given package namespace)
