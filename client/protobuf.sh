#!/bin/sh

dart pub global activate protoc_plugin
mkdir ./lib/generated
protoc --dart_out=grpc:./lib/generated  --proto_path=.. data.proto

echo finished