#!/bin/sh

set -e
cd "$(dirname "$(realpath "$0")")"

./clean.py
./protoc.py
./mobx.py
./flutter.py
./rust.py


echo -e "\n🌈 \033[1;97mBuild complete!\033[0m\n"
