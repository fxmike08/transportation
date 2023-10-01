#!/bin/bash
# For debug only
#set -o xtrace

filename="openapi-generator-cli-7.0.0.jar"
openapi_gen_link="https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/7.0.0/${filename}"
openapi_bin_path="./target/openapi_jar/"

if ! [ -f "${openapi_bin_path}${filename}" ]; then
    # File does not exist
    mkdir -p target/openapi_jar
    cd ${openapi_bin_path}
    wget "${openapi_gen_link}"
fi

java -jar "${openapi_bin_path}/${filename}" generate --package-name transportation_api -i /opt/work_rust/bus_monitor/api.yaml -g rust-server -o ./transportation_api