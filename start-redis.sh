#!/bin/bash

PORT=${PORT:-56443}

redis-server \
    --port 0 \
    --unixsocket redis-tls.sock \
    --tls-port ${PORT} \
    --tls-ca-cert-file keys/ca.crt \
    --tls-cert-file keys/redis.crt \
    --tls-key-file keys/redis.key
