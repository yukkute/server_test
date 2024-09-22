#!/bin/sh

cd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd ../client

PROTOBUF="lib/generated/protobuf"

if ! dart pub global list | grep -q 'protoc_plugin'; then
 dart pub global activate protoc_plugin
fi

rm -rf "$PROTOBUF"
mkdir -p "$PROTOBUF"

protoc -I/usr/include/ \
--dart_out="grpc:$PROTOBUF"  \
--proto_path=".." data.proto

echo "finished" $(basename "$0")