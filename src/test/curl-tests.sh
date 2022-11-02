#!/bin/bash
set -ex
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/health
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/events
echo "-"
curl --fail -X GET http://0.0.0.0:8080/home
echo "-"
curl --fail -X GET http://0.0.0.0:8080/about
echo "-"
curl --fail -X POST http://0.0.0.0:8080/add -H 'Content-Type: application/json' -d '{"service":"infrasvc","event":"deploy-infrasvc-v0.0.3", "event_type":"deploy-qa"}'
echo "-"
curl --fail -X POST http://0.0.0.0:8080/add -H 'Content-Type: application/json' -d '{"service":"infrasvc","event":"deploy-infrasvc-v0.0.3", "event_type":"deploy-qa", "datetime": "2022-11-01 00:00:00"}'
echo "-"
curl --fail -I -X GET http://0.0.0.0:8080/events
echo -e "\nTests Pass"
