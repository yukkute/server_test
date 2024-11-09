#!/bin/sh

set -e
cd "$(dirname "$(realpath "$0")")"
cd ..

IMAGE_NAME="moreonigiri_image"
CONTAINER_NAME="moreonigiri_container"

docker build -t $IMAGE_NAME .

docker run -it --rm \
	--name $CONTAINER_NAME \
	--mount type=bind,source="$(pwd)",target=/home/developer/moreonigiri \
	$IMAGE_NAME

echo "Finished $(basename "$0")"
