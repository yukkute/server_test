#!/bin/sh

dart pub global activate protoc_plugin
mkdir ./lib/generated
protoc -I/usr/include --dart_out=grpc:./lib/generated  --proto_path=.. data.proto

echo finished