#!/bin/bash

# Performance test against any server
SERVER_ADDR=$1
REQUESTS=1000
CONCURRENCY=2

./perf/welle  -n $REQUESTS -c $CONCURRENCY $SERVER_ADDR