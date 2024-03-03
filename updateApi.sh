#!/usr/bin/env bash

docker run --rm -v ${PWD}:/local --user $UID:$GID openapitools/openapi-generator-cli generate -i "https://developer.tenable.com/openapi/653bf52677b3bd00411bda79" -g rust -o /local/openapi_generator/rust