#!/usr/bin/env bash

fields=$(bw get item tenable.com | jq '.fields')
secretKey=$(echo $fields | jq '.[] | select(.name == "Secret Key") | .value' -r)
accessKey=$(echo $fields | jq '.[] | select(.name == "Access Key") | .value' -r)

export TIO_SECRET_KEY=$secretKey
export TIO_ACCESS_KEY=$accessKey

python3 src/app.py