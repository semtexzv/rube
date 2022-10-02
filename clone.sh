#!/usr/bin/env bash

git clone https://github.com/etcd-io/etcd --depth 1 vendor/etcd
git clone https://github.com/googleapis/googleapis --depth 1 vendor/google
mkdir -p vendor/gogoproto
wget https://raw.githubusercontent.com/gogo/protobuf/master/gogoproto/gogo.proto -O vendor/gogoproto/gogo.proto