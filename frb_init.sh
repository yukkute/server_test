#!/bin/sh

# Create the symbolic link
source_dir="./server"
target_dir="./client/rust"
ln -s "$(readlink -e "$source_dir")" "$target_dir"

cd client

REPO_URL="https://github.com/fzyzcjy/flutter_rust_bridge.git"
FOLDER_PATH="frb_utils"

if [ ! -d "$FOLDER_PATH" ]; then
    git clone --filter=blob:none $REPO_URL temp_repo
    cd temp_repo
    git config core.sparseCheckout true
    echo "$FOLDER_PATH/*" > .git/info/sparse-checkout
    git pull origin master
    mv $FOLDER_PATH ..
    cd ..
    rm -rf temp_repo
fi

SHADY_PUBSPEC="frb_utils/pubspec.yaml"
sed -i -e ':a;N;$!ba;s|flutter_rust_bridge:\n    path: ../frb_dart|flutter_rust_bridge: ^2.0.0-dev.28|g' $SHADY_PUBSPEC
sed -i 's/test_core: \^0.5.9/^0.6.0/' $SHADY_PUBSPEC

echo "finished" $(basename "$0")