#!/bin/sh

set -e
cd "$(dirname "$(realpath "$0")")"

cd ../client
flutter build linux --release

cd ../server
cargo build --release --target x86_64-unknown-linux-gnu

echo
DEST_DIR=../client/build/linux/x64/release/bundle/lib
mkdir -p "$DEST_DIR"
cp -v ./target/x86_64-unknown-linux-gnu/release/libmoreonigiri_server.so "$DEST_DIR"

echo "Finished $(basename "$0")"
