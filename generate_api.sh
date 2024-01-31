#!/bin/bash
export JAVA_OPTS="-DmaxYamlCodePoints=9999999"

java $JAVA_OPTS -jar openapi-generator-cli-7.0.1.jar generate \
	--config "${PWD}/config.yaml" \
	--input-spec "${PWD}/netbox_openapi.yaml" \
	--output "${PWD}/api_output"

cp api_output/README.md README.md
