#!/bin/bash
set -e

if [ -z "$1" ]; then
  echo "Required generator file is missing."
  exit 1
fi

file="$1"

rm -rf ./openapi

docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli:latest generate \
  -i "/local/$file" \
  -g rust \
  -o /local/openapi \
  --additional-properties=packageVersion=0.1.0
