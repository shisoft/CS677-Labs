# Experiments
Following chapters are for proving that the fault-tolerent indeed works and cached data in frontend are consistent with backend.

## Fault-tolerent
This experiment shows how backend server can be covered by caching and fail-over.
First start the cluster at `src/` by

```
sudo docker-compose up
```

Server will start and connect with each other in less than 30 seconds, frontend server cache is empty right now. If you read through the log, we can see the `catalog-1` and `order-1` is the leader of the cluster or their service.

```
catalog-1           | 2020-04-26 03:02:32,158 DEBUG [catalog_server::configs] Will bootstrap
catalog-1           | 2020-04-26 03:02:32,158 DEBUG [bifrost::raft] Conservative bootstrap, checking storage
catalog-1           | 2020-04-26 03:02:32,158 DEBUG [bifrost::raft] There are storage, checking last term
catalog-1           | 2020-04-26 03:02:32,158 DEBUG [bifrost::raft] Log is empty, bootstrap
catalog-1           | 2020-04-26 03:02:32,158 DEBUG [bifrost::raft] Server 10578407250799441171 become leader, term 2
```

```
order-1             | 2020-04-26 03:02:32,157 DEBUG [order_server::configs] Will bootstrap
order-1             | 2020-04-26 03:02:32,157 DEBUG [bifrost::raft] Conservative bootstrap, checking storage
order-1             | 2020-04-26 03:02:32,157 DEBUG [bifrost::raft] There are storage, checking last term
order-1             | 2020-04-26 03:02:32,157 DEBUG [bifrost::raft] Log is empty, bootstrap
order-1             | 2020-04-26 03:02:32,157 DEBUG [bifrost::raft] Server 13433766016950577570 become leader, term 2
```


Query the frontens server, using the client, wget or web browser

```
curl -v http://localhost:34800/lookup/1
```
We can get

```
*   Trying 127.0.0.1:34800...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 34800 (#0)
> GET /lookup/1 HTTP/1.1
> Host: localhost:34800
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 171
< content-type: application/json
< date: Sun, 26 Apr 2020 02:57:24 GMT
< 
* Connection #0 to host localhost left intact
{"ok":true,"result":{"id":1,"name":"How to get a good grade in 677 in 20 minutes a day","stock":4,"price":19.9,"topic":1},"topics":[{"id":1,"name":"Distributed systems"}]}% 
```
Docker compose windows shows that the data was fetched from one of the backend server `catalog-1`

```
fontend             | 2020-04-26 03:02:43,516 DEBUG [pygmy_frontend] Using remote result for 1
fontend             | 2020-04-26 03:02:43,516 DEBUG [pygmy_frontend] Checking url (0): http://catalog-1:34801/lookup/1
fontend             | 2020-04-26 03:02:43,516 DEBUG [reqwest::connect] starting new connection: http://catalog-1:34801/
fontend             | 2020-04-26 03:02:43,516 DEBUG [hyper::client::connect::dns] resolving host="catalog-1"
fontend             | 2020-04-26 03:02:43,517 DEBUG [hyper::client::connect::http] connecting to 172.18.0.6:34801
fontend             | 2020-04-26 03:02:43,517 DEBUG [hyper::client::connect::http] connected to 172.18.0.6:34801
fontend             | 2020-04-26 03:02:43,517 DEBUG [hyper::proto::h1::io] flushed 62 bytes
catalog-1           | 2020-04-26 03:02:43,517 DEBUG [bifrost::raft::client] Construct cached state machine for list ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"], service id 10662549175266302366, state machine 50
catalog-1           | 2020-04-26 03:02:43,517 DEBUG [bifrost::raft::client] Creating state machine client instance, service 10662549175266302366, state machine id 50
catalog-1           | 2020-04-26 03:02:43,517 DEBUG [bifrost::raft::client] Getting server info for ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-1           | 2020-04-26 03:02:43,517 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]...10
catalog-1           | 2020-04-26 03:02:43,517 DEBUG [bifrost::raft::client] Checking server info on catalog-1:34803
catalog-1           | 2020-04-26 03:02:43,517 DEBUG [bifrost::raft::client] Connecting to node catalog-1:34803
catalog-1           | 2020-04-26 03:02:43,517 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-1           | 2020-04-26 03:02:43,517 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::raft::client] Added server info on catalog-1:34803 to members
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::raft::client] Member catalog-1:34803 added
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::raft::client] Getting server info from catalog-1:34803
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::raft::client] Checking server client catalog-1:34803
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-1:34803
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::raft::client] Checking response from catalog-1:34803
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::raft::client] Found server info with leader id 10578407250799441171
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-1           | 2020-04-26 03:02:43,518 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-1           | 2020-04-26 03:02:43,519 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-1           | 2020-04-26 03:02:43,519 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-1           | 2020-04-26 03:02:43,519 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-1           | 2020-04-26 03:02:43,519 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-2           | 2020-04-26 03:02:43,519 INFO  [catalog_server] Lookup 1
catalog-1           | 2020-04-26 03:02:43,538 INFO  [actix_web::middleware::logger] 172.18.0.2:52970 "GET /lookup/1 HTTP/1.1" 200 171 "-" "-" 0.020217
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::proto::h1::io] read 280 bytes
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::proto::h1::io] parsed 3 headers
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::proto::h1::conn] incoming body is content-length (171 bytes)
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::proto::h1::conn] incoming body completed
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::client::pool] pooling idle connection for ("http", catalog-1:34801)
fontend             | 2020-04-26 03:02:43,538 DEBUG [reqwest::async_impl::client] response '200 OK' for http://catalog-1:34801/lookup/1
fontend             | 2020-04-26 03:02:43,538 INFO  [actix_web::middleware::logger] 172.18.0.1:46042 "GET /lookup/1 HTTP/1.1" 200 171 "-" "curl/7.68.0" 0.022164
```

Backend server `catalog-1` will go to the state machines for data. Now, issue the command again

```
curl -v http://localhost:34800/lookup/1
```

Now the frontend server will poll the result from cache and won't go through backend server again.

```
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::proto::h1::io] read 280 bytes
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::proto::h1::io] parsed 3 headers
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::proto::h1::conn] incoming body is content-length (171 bytes)
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::proto::h1::conn] incoming body completed
fontend             | 2020-04-26 03:02:43,538 DEBUG [hyper::client::pool] pooling idle connection for ("http", catalog-1:34801)
fontend             | 2020-04-26 03:02:43,538 DEBUG [reqwest::async_impl::client] response '200 OK' for http://catalog-1:34801/lookup/1
fontend             | 2020-04-26 03:02:43,538 INFO  [actix_web::middleware::logger] 172.18.0.1:46042 "GET /lookup/1 HTTP/1.1" 200 171 "-" "curl/7.68.0" 0.022164
fontend             | 2020-04-26 03:03:34,524 DEBUG [pygmy_frontend] Using cached result for 1
fontend             | 2020-04-26 03:03:34,524 INFO  [actix_web::middleware::logger] 172.18.0.1:46572 "GET /lookup/1 HTTP/1.1" 200 171 "-" "curl/7.68.0" 0.000095
```

Stop one server, say, `catalog-1`, the leader of catalog servers.

```
sudo docker stop catalog-1
```

After a little one of the alive catalog server will detect their leader is down, and become a candidate to request votes from other nodes. Remaining 2 nodes will grant their node to make the candidate a new leader.

```
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] LEADER 10578407250799441171 TIMEOUT!!! GOING TO CANDIDATE!!! 4820394039294786766, time remains -23ms
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] 4820394039294786766 become candidate
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] Member 4820394039294786766 vote for itself
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] Member 4820394039294786766 received 1 votes in for now
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] Members 3 granted 1, is majority: false
catalog-2           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] 16030156735997574805 VOTE FOR: 4820394039294786766, valid: true
catalog-2           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] 16030156735997574805 VOTE FOR: 4820394039294786766, granted: true
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] Member 4820394039294786766 received one vote from 16030156735997574805
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] Member 4820394039294786766 received 2 votes in for now
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] Members 3 granted 2, is majority: true
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] Member 4820394039294786766 become leader for received majority votes
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] Server 4820394039294786766 become leader, term 3
catalog-3           | 2020-04-26 03:06:51,991 DEBUG [bifrost::raft] GRANTED 4820394039294786766: 2/3
catalog-3           | 2020-04-26 03:06:53,993 DEBUG [bifrost::raft] Member 4820394039294786766 request vote failed from 10578407250799441171
```

We call this process `leader transfer`. After leader transfer, we still want to know if the service is uneffected. If we try send request to for `lookup/1`, the frontend can still provice the data because of caches. Now, let us send a request that have not been cached yet, say `lookup/2`

```
curl -X POST -v http://localhost:34800/order/3\?amount\=6
```

We can still get the data if it is not cached

```
*   Trying 127.0.0.1:34800...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 34800 (#0)
> GET /lookup/2 HTTP/1.1
> Host: localhost:34800
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 138
< content-type: application/json
< date: Sun, 26 Apr 2020 03:18:55 GMT
< 
* Connection #0 to host localhost left intact
{"ok":true,"result":{"id":2,"name":"RPCs for Dummies","stock":16,"price":29.9,"topic":1},"topics":[{"id":1,"name":"Distributed systems"}]}%  
```

Now, lets send more uncached requests `lookup/2`, `lookup/3`, `lookup/4` so the frontend can rotate back to unavailable `catalog-1`. When requesting for `lookup/4`, the frontend server can discover that `catalog-1` is unreachable, and send to next server which is `catalog-2` instead.

```
fontend             | 2020-04-26 03:22:05,493 DEBUG [pygmy_frontend] Using remote result for 4
fontend             | 2020-04-26 03:22:05,493 DEBUG [pygmy_frontend] Checking url (0): http://catalog-1:34801/lookup/4
fontend             | 2020-04-26 03:22:05,493 DEBUG [reqwest::connect] starting new connection: http://catalog-1:34801/
fontend             | 2020-04-26 03:22:05,493 DEBUG [hyper::client::connect::dns] resolving host="catalog-1"
fontend             | 2020-04-26 03:22:05,512 DEBUG [pygmy_frontend] Checking url (1): http://catalog-2:34801/lookup/4
fontend             | 2020-04-26 03:22:05,512 DEBUG [reqwest::connect] starting new connection: http://catalog-2:34801/
fontend             | 2020-04-26 03:22:05,512 DEBUG [hyper::client::connect::dns] resolving host="catalog-2"
fontend             | 2020-04-26 03:22:05,512 DEBUG [hyper::client::connect::http] connecting to 172.19.0.6:34801
fontend             | 2020-04-26 03:22:05,512 DEBUG [hyper::client::connect::http] connected to 172.19.0.6:34801
fontend             | 2020-04-26 03:22:05,512 DEBUG [hyper::proto::h1::io] flushed 62 bytes
catalog-3           | 2020-04-26 03:22:05,513 INFO  [catalog_server] Lookup 4
catalog-2           | 2020-04-26 03:22:05,513 INFO  [actix_web::middleware::logger] 172.19.0.3:44732 "GET /lookup/4 HTTP/1.1" 200 160 "-" "-" 0.000846
```

We can see `catalog-3` is part of this, because it is the server that called by `catalog-2` to synchronize the Raft state machine. This experiment is over, use Ctrl + C or to quit.

## Cache consistency

Now, lets order something. This will introduce commands that will produce logs in the state machine. The log need to be replicated accross cluster member.

```
sudo docker-compose up
```
First let's see how much books on stock

```
curl -v http://localhost:34800/lookup/3
```

It gives

```
*   Trying 127.0.0.1:34800...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 34800 (#0)
> GET /lookup/3 HTTP/1.1
> Host: localhost:34800
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 161
< content-type: application/json
< date: Sun, 26 Apr 2020 03:47:55 GMT
< 
* Connection #0 to host localhost left intact
{"ok":true,"result":{"id":3,"name":"Xen and the Art of Surviving Graduate School","stock":25,"price":9.9,"topic":2},"topics":[{"id":2,"name":"Graduate School"}]}%    
```

We can see there are 25 books on stock. To make the data is cached, we can send more of the same request and cinfirm with server logs. 

Next send request to buy books, for example, 6 of them

```
curl -X POST -v http://localhost:34800/order/3\?amount\=6
```

It should return true.

```
➜ curl -X POST -v http://localhost:34800/order/3\?amount\=6                                    
*   Trying 127.0.0.1:34800...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 34800 (#0)
> POST /order/3?amount=6 HTTP/1.1
> Host: localhost:34800
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 4
< content-type: application/json
< date: Sun, 26 Apr 2020 03:49:43 GMT
< 
* Connection #0 to host localhost left intact
true% 
```

At this point, frontend server should receive message from catalog server to invalidate the cache

```
fontend             | 2020-04-26 03:49:43,820 INFO  [pygmy_frontend] Invalidate item 3
fontend             | 2020-04-26 03:49:43,820 INFO  [actix_web::middleware::logger] 172.19.0.8:59878 "POST /invalidate/item/3 HTTP/1.1" 200 4 "-" "-" 0.000090
```

If we send the same lookup request again, the frontend server should now ask for backend and return the new one

```
*   Trying 127.0.0.1:34800...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 34800 (#0)
> GET /lookup/3 HTTP/1.1
> Host: localhost:34800
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 161
< content-type: application/json
< date: Sun, 26 Apr 2020 03:50:19 GMT
< 
* Connection #0 to host localhost left intact
{"ok":true,"result":{"id":3,"name":"Xen and the Art of Surviving Graduate School","stock":19,"price":9.9,"topic":2},"topics":[{"id":2,"name":"Graduate School"}]}%   
```

It shows this book have 19 in stock.