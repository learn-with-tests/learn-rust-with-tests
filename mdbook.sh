#!/bin/bash

set -e

name=mdbook-$RANDOM

echo -n building docker image...
docker build -qt $name mdbook >/dev/null
echo done.

docker run -it --init -v $PWD:$PWD -w $PWD -u $(id -u) --rm -p 3000:3000 -p 3001:3001 --name $name $name "$@"