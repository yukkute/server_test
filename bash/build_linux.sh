#!/bin/sh

set -e
cd "$(dirname "$(realpath "$0")")"

# Ask the user if they want to clean the build
read -p "Do you want a clean-build? (Y/n): " clean_build
clean_build=${clean_build:-Y}

if [ "$clean_build" = "y" ] || [ "$clean_build" = "Y" ]; then
    echo "Cleaning build directories..."

    # Clean Flutter build directory
    cd ../client
    flutter clean >/dev/null

    cd ../server
    cargo clean -q

    echo "Clean complete."
fi

# Build Flutter client
cd ../client
flutter build linux --release || {
    echo -e "\n😔 An error occurred while building the Flutter client."
    echo -e "👉 Try running the script with \033[1m\033[31mclean-build\033[0m option.\n"
    exit 1
}

# Build Rust server
cd ../server
cargo build \
    --release \
    --target x86_64-unknown-linux-gnu \
    --color always \
    2>&1 | while IFS= read -r line; do
    # Output just the last line of cargo build
    printf "\r\033[K%s" "$line"
done || {
    echo -e "\n😔 An error occurred while building the Rust server."
    echo -e "👉 Try running the script with \033[1m\033[31mclean-build\033[0m option.\n"
    exit 1
}

# Copy the shared library to the Flutter client's bundle directory
echo
LIB=./target/x86_64-unknown-linux-gnu/release/libmoreonigiri_server.so
DEST_DIR=../client/build/linux/x64/release/bundle/lib
mkdir -p "$DEST_DIR"
cp -v "$LIB" "$DEST_DIR" | sed -E 's/(.*) -> (.*)/\1\n    -> \2/'

# Success message
echo -e "\n🎉 \033[1m\033[33mBuild complete!\033[0m\n"
