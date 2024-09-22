#!/bin/sh

cd "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd ..

# Create the symbolic link
source_dir="server"
target_dir="client/rust"
ln -sr "$(readlink -e "$source_dir")" "$target_dir"

cd client

echo "finished" $(basename "$0")