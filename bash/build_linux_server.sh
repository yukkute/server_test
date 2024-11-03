#!/bin/sh

set -e
cd "$(dirname "$(realpath "$0")")"

cd ../server
echo -e "\n🦀 \033[1;37mBuilding Rust server...\033[0m\n"
cargo build \
	--release \
	--target x86_64-unknown-linux-gnu \
|| {
	echo -e "\n😔 An error occurred while building the Rust server."
	echo -e "👉 Try running the script with \033[1m\033[31mclean-build\033[0m option.\n"
	exit 1
}

echo

# Copy the shared library to the Flutter client's bundle directory
LIB=./target/x86_64-unknown-linux-gnu/release/libmoreonigiri_server.so
DEST_DIRS=( 
        "../client/build/linux/x64/debug/bundle/lib"
        "../client/build/linux/x64/release/bundle/lib"         
)
for DEST_DIR in "${DEST_DIRS[@]}"; do
	mkdir -p "$DEST_DIR"
	cp "$LIB" "$DEST_DIR"
	echo "Added server .so to $DEST_DIR"
done