#!/bin/sh

cd ..

cd client

PROTOBUF="lib/generated/protobuf"

dart pub global activate protoc_plugin

rm -rf "$PROTOBUF"
mkdir -p "$PROTOBUF"

protoc -I/usr/include/ \
--dart_out="grpc:$PROTOBUF"  \
--proto_path=.. data.proto

echo "finished" $(basename "$0")