#!/bin/sh

set -e
cd "$(dirname "$(realpath "$0")")"

# Check for required commands
check_command() {
	command -v "$1" >/dev/null 2>&1 || { echo >&2 "Error: $1 is not installed."; exit 1; }
}
check_command flutter
check_command cargo
check_command protoc

# Ask the user if they want a clean build
while true; do
	read -p "Do you want a clean build? (Y/n): " clean_build
	clean_build=${clean_build:-Y}
	case "$clean_build" in
		[Yy]* ) 
			./clean.sh  # Call clean.sh as an executable
			break;;
		[Nn]* ) 
			clean_build="N"; 
			break;;
		* ) 
	esac
done

# Build Flutter client
cd ..
mkdir -p client/lib/generated/protobuf/
protoc --proto_path=proto --dart_out=grpc:client/lib/generated/protobuf proto/*.proto

cd client
echo -e "\n🐦 \033[1;37mBuilding Flutter client...\033[0m"

flutter pub run build_runner build
flutter build linux --release || {
	echo -e "\n😔 An error occurred while building the Flutter client."
	echo -e "👉 Try running the script with \033[1m\033[31mclean-build\033[0m option.\n"
	exit 1
}

# Build Rust server
cd ../bash
./build_linux_server.sh


echo -e "\n🎉 \033[1m\033[33mBuild complete!\033[0m\n"
