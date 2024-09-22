#!/bin/sh

cd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd ..

rm -f client/rust #link

rm -rf client/generated/protobuf
rm -rf client/generated/rust

rm -rf server/src/generated/frb

echo "finished" $(basename "$0")