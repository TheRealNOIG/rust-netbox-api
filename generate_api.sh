#!/bin/bash
export JAVA_OPTS="-DmaxYamlCodePoints=9999999"

java $JAVA_OPTS -jar openapi-generator-cli-7.0.1.jar generate \
	-g rust \
	-p useSingleRequestParameter=true \
	--package-name netbox \
	--input-spec "${PWD}/netbox_openapi.yaml" \
	--output "${PWD}/api_output"

cp api_output/README.md README.md
