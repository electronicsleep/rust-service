#!/bin/bash
set -ex
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/health
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/events
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/event/rust-service
echo "-"
curl --fail -X GET http://0.0.0.0:8080/home
echo "-"
curl --fail -X GET http://0.0.0.0:8080/about
echo "-"
curl --fail -X POST http://0.0.0.0:8080/add -H 'Content-Type: application/json' -d '{"service":"rust-service", "event":"deploy-rust-service-v0.0.1", "event_type":"deploy-qa"}'
echo "-"
curl --fail -X POST http://0.0.0.0:8080/add -H 'Content-Type: application/json' -d '{"service":"rust-service", "event":"deploy-rust-service-v0.0.2", "event_type":"deploy-staging", "datetime": "2022-11-01 00:00:00"}'
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/events
echo -e "\nTests Pass"
