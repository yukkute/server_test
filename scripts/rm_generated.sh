#!/bin/sh

cd ..

rm -f client/rust
rm -rf client/generated/protobuf
rm -rf client/generated/rust

rm -rf server/generated/frb

echo "finished" $(basename "$0")