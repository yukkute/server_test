#!/bin/sh

set -e
cd "$(dirname "$(realpath "$0")")"

echo -e -n "\n\033[37m🧹 Cleaning build directories... "

rm -rf ../client/lib/generated/mobx
rm -rf ../client/lib/generated/protobuf

cd ../client
flutter clean &>/dev/null

cd ../server
cargo clean -q
echo -e "cleaned\033[0m"
