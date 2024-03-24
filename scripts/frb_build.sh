#!/bin/sh

SERVER_GEN="server/src/generated/frb"
CLIENT_GEN="client/lib/generated/rust"


# Clean previous generated code
rm -rf "$SERVER_GEN"
rm -rf "$CLIENT_GEN"
mkdir -p "$SERVER_GEN"
mkdir -p "$CLIENT_GEN"

echo -e "pub mod frb_generated;\n" > "$SERVER_GEN/mod.rs"

# Run
flutter_rust_bridge_codegen generate \
--no-add-mod-to-lib \
--rust-root "server" \
--rust-input "server/src/mo_server/local_server.rs" \
--rust-output "$SERVER_GEN/frb_generated.rs" \
--dart-output "$CLIENT_GEN"

echo "finished" $(basename "$0")