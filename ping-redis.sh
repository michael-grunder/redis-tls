#!/bin/bash

PORT=${PORT:-56443}

redis-cli --tls -p ${PORT} --cacert keys/ca.crt --cert keys/redis.crt --key keys/redis.key PING
