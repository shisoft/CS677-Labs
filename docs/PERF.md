# Performance Test
Here we will test performance in term of response time. In this test, servers are on the same node with the help of `docker-compose` to avoid influences from hardware. Every endpoint will run 10k times and 2 concurrent threads.

We can see that end-to-end total time and average time is shorter than invoking the backend server directly. This is because these request results are cached in the frontend server, but backend server need to confirm the data it got from the Raft state machine is new, to prevent stale. On cache hit, frontend only need to get the data it last seen from the cache and send to end users.

## End-to-end

With command `./tests/perf/welle -n 10000 -c 2 http://localhost:34800/...`

### /lookup
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 928.910773ms
Avg Time Taken: 92.891µs
Total Time In Flight: 1.526589204s
Avg Time In Flight: 152.658µs

Percentage of the requests served within a certain time:
50%: 221.342µs
66%: 236.235µs
75%: 245.833µs
80%: 251.799µs
90%: 268.09µs
95%: 281.406µs
99%: 312.514µs
100%: 13.374458ms
```

### /lookup/1
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 915.385169ms
Avg Time Taken: 91.538µs
Total Time In Flight: 1.498144494s
Avg Time In Flight: 149.814µs

Percentage of the requests served within a certain time:
50%: 215.838µs
66%: 229.617µs
75%: 239.993µs
80%: 245.998µs
90%: 264.369µs
95%: 282.112µs
99%: 319.177µs
100%: 12.023352ms
```

### /search/sys
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 905.60481ms
Avg Time Taken: 90.56µs
Total Time In Flight: 1.45751483s
Avg Time In Flight: 145.751µs

Percentage of the requests served within a certain time:
50%: 212.702µs
66%: 224.423µs
75%: 233.155µs
80%: 239.236µs
90%: 257.085µs
95%: 272.028µs
99%: 305.832µs
100%: 3.577809ms
```

## Catalog Server

With `./tests/perf/welle -n 10000 -c 2 http://localhost:34801/`

### /lookup
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 2.643955332s
Avg Time Taken: 264.395µs
Total Time In Flight: 4.958533787s
Avg Time In Flight: 495.853µs

Percentage of the requests served within a certain time:
50%: 736.944µs
66%: 755.054µs
75%: 767.648µs
80%: 776.785µs
90%: 805.023µs
95%: 838.662µs
99%: 940.243µs
100%: 1.966229ms
```

### /lookup/1
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 2.631342098s
Avg Time Taken: 263.134µs
Total Time In Flight: 4.918096042s
Avg Time In Flight: 491.809µs

Percentage of the requests served within a certain time:
50%: 727.647µs
66%: 747.59µs
75%: 762.358µs
80%: 772.086µs
90%: 802.99µs
95%: 857.502µs
99%: 948.413µs
100%: 2.035809ms
```

### /search/sys
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 2.639603328s
Avg Time Taken: 263.96µs
Total Time In Flight: 4.934475313s
Avg Time In Flight: 493.447µs

Percentage of the requests served within a certain time:
50%: 732.226µs
66%: 750.791µs
75%: 762.303µs
80%: 770.283µs
90%: 793.351µs
95%: 815.939µs
99%: 955.838µs
100%: 2.101583ms
```