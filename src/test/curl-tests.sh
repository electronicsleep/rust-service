#!/bin/bash
URL=http://0.0.0.0:8081
set -ex
curl --fail -I -X GET $URL
curl --fail -I -X GET $URL/health
curl --fail -I -X GET $URL/events
curl --fail -i -X GET $URL/home
curl --fail -i -X GET $URL/about
curl --fail -i -X POST $URL/add -H 'Content-Type: application/json' -d '{"service":"rust-service", "event":"deploy-rust-service-v0.0.1", "event_type":"deploy-qa"}'
curl --fail -i -X POST $URL/add -H 'Content-Type: application/json' -d '{"service":"rust-service", "event":"deploy-rust-service-v0.0.2", "event_type":"deploy-staging", "datetime": "2022-11-01 00:00:00"}'
echo -e "\nTests Pass"
