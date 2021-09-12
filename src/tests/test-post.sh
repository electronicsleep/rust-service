#!/bin/bash
set -ex
curl --fail -I -X GET http://0.0.0.0:8080/
curl --fail -I -X GET http://0.0.0.0:8080/health
curl --fail -I -X GET http://0.0.0.0:8080/events
curl --fail -i -X POST http://0.0.0.0:8080/add -H 'Content-Type: application/json' -d '{"service":"infrasvc","event":"deploy-infrasvc-v0.0.1", "event_type":"deploy-qa"}'
curl --fail -I -X GET http://0.0.0.0:8080/events
echo -e "\ntests pass"
