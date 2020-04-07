# Performance Test
Here we will test performance in term of response time. In this test, servers are on the same node to avoid influences from hardware. Every endpoint will run 10k times and 2 concurrent threads.
For microservices, response time really impact performance because a lower level component can really slow down the whole system. From the result we have so far, response time is low enough to complete 10k requests in less than 30 seconds. Note that in average, lower level component have less response time than end-to-end.

## End-to-end
### /lookup
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 23.152702378s
Avg Time Taken: 2.31527ms
Total Time In Flight: 45.557307057s
Avg Time In Flight: 4.55573ms

Percentage of the requests served within a certain time:
50%: 5.105345ms
66%: 6.127282ms
75%: 7.302952ms
80%: 8.811533ms
90%: 12.563581ms
95%: 14.619875ms
99%: 19.503072ms
100%: 738.596808ms
```

### /lookup/1
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 20.962293936s
Avg Time Taken: 2.096229ms
Total Time In Flight: 41.16806842s
Avg Time In Flight: 4.116806ms

Percentage of the requests served within a certain time:
50%: 4.536901ms
66%: 5.517902ms
75%: 6.614303ms
80%: 8.254468ms
90%: 12.00563ms
95%: 13.6712ms
99%: 19.844407ms
100%: 73.076806ms
```

### /search/sys
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 21.800157177s
Avg Time Taken: 2.180015ms
Total Time In Flight: 42.840328194s
Avg Time In Flight: 4.284032ms

Percentage of the requests served within a certain time:
50%: 4.863097ms
66%: 5.834392ms
75%: 6.89146ms
80%: 8.328634ms
90%: 12.300946ms
95%: 14.35856ms
99%: 19.226818ms
100%: 241.189939ms
```

## Catalog Server
### /lookup
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 3.951041545s
Avg Time Taken: 395.104µs
Total Time In Flight: 7.370389127s
Avg Time In Flight: 737.038µs

Percentage of the requests served within a certain time:
50%: 870.545µs
66%: 970.788µs
75%: 1.047408ms
80%: 1.115115ms
90%: 1.541698ms
95%: 2.7268ms
99%: 4.626301ms
100%: 20.149187ms
```

### /lookup/1
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 3.239684208s
Avg Time Taken: 323.968µs
Total Time In Flight: 6.027733874s
Avg Time In Flight: 602.773µs

Percentage of the requests served within a certain time:
50%: 823.419µs
66%: 902.062µs
75%: 961.686µs
80%: 1.002449ms
90%: 1.170469ms
95%: 1.413566ms
99%: 2.348526ms
100%: 3.735492ms
```

### /search/sys
```
Total Requests: 10000
Concurrency Count: 2
Total Completed Requests: 10000
Total Errored Requests: 0
Total 5XX Requests: 0

Total Time Taken: 3.338709528s
Avg Time Taken: 333.87µs
Total Time In Flight: 6.217127696s
Avg Time In Flight: 621.712µs

Percentage of the requests served within a certain time:
50%: 841.223µs
66%: 916.932µs
75%: 985.33µs
80%: 1.036777ms
90%: 1.231422ms
95%: 1.493891ms
99%: 2.467146ms
100%: 3.184704ms
```