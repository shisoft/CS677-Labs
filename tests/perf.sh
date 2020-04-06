#!/bin/bash

# Performance test against frontend and catalog server
SERVER_ADDR=$1
REQUESTS=1000
CONCURRENCY=2

./perf/welle  -n $REQUESTS -c $CONCURRENCY $SERVER_ADDR/lookup/1