# Outputs
Following content contains output from the client, frontend server, catalog server and order server.
Note that because the fontend interfaces have not been changed, Client output is identical with Lab 2.
## Client
```
/Users/shisoft/Dropbox/Homeworks/CS677/lab-2-huang-shi/tests/pygmy-client/target/debug/pygmy-client http://localhost:34800
Welcome to Pygmy.com - the World’s smallest book store
Server URL is: http://localhost:34800
You can
1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
5. Test
6. Leave
Select by input the number: 
1
What is the topic? Any keywords in the topics will be fine (try 'sys'):
dist
1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 5
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 17
Press return key to continue

1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
5. Test
6. Leave
Select by input the number: 
2
1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 5
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 17
3. Xen and the Art of Surviving Graduate School - Price: 9.9, Topic: Graduate School, Stock: 25
4. Cooking for the Impatient Graduate Student - Price: 30.9, Topic: Graduate School, Stock: 15
All books listed.
Press return key to continue
2
1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
5. Test
6. Leave
Select by input the number: 
3
Which book do you want to look at? Input the number: 
3
3. Xen and the Art of Surviving Graduate School - Price: 9.9, Topic: Graduate School, Stock: 25
Press return key to continue

1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
5. Test
6. Leave
Select by input the number: 
4
1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 5
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 17
3. Xen and the Art of Surviving Graduate School - Price: 9.9, Topic: Graduate School, Stock: 25
4. Cooking for the Impatient Graduate Student - Price: 30.9, Topic: Graduate School, Stock: 15
Which book would you like: 
1
amount: 
1
Bought book How to get a good grade in 677 in 20 minutes a day, amount 1
Press return key to continue
```

## Frontend
```
2020-04-07 00:11:14,375 INFO  [actix_server::builder] Starting 12 workers
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,376 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,376 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,376 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,376 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,377 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34800" service on 0.0.0.0:34800
2020-04-07 00:11:14,376 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,377 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,377 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,377 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,376 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::sys::unix::kqueue] registering; token=Token(100); interests=Readable
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::sys::unix::kqueue] registering; token=Token(2); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:14,378 TRACE [mio::poll] registering with poller
2020-04-07 00:11:14,378 TRACE [mio::sys::unix::kqueue] registering; token=Token(3); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:14,380 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,381 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,382 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,383 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,384 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,385 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,386 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,387 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,388 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,389 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,390 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,392 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,392 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,394 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,394 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,396 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,397 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,400 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,400 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,401 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,402 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:14,404 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:11:14,406 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34800" is available
2020-04-07 00:13:13,586 TRACE [mio::poll] registering with poller
2020-04-07 00:13:13,586 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:13,587 TRACE [hyper::client::pool] checkout waiting for idle connection: ("http", 127.0.0.1:34801)
2020-04-07 00:13:13,588 DEBUG [reqwest::connect] starting new connection: http://127.0.0.1:34801/
2020-04-07 00:13:13,588 TRACE [hyper::client::connect::http] Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(34801))
2020-04-07 00:13:13,588 DEBUG [hyper::client::connect::http] connecting to 127.0.0.1:34801
2020-04-07 00:13:13,588 TRACE [mio::poll] registering with poller
2020-04-07 00:13:13,588 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:13,588 DEBUG [hyper::client::connect::http] connected to 127.0.0.1:34801
2020-04-07 00:13:13,588 TRACE [hyper::client::conn] client handshake HTTP/1
2020-04-07 00:13:13,588 TRACE [hyper::client] handshake complete, spawning background dispatcher task
2020-04-07 00:13:13,588 TRACE [want] signal: Want
2020-04-07 00:13:13,588 TRACE [want] signal found waiting giver, notifying
2020-04-07 00:13:13,588 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2020-04-07 00:13:13,589 TRACE [want] poll_want: taker wants!
2020-04-07 00:13:13,589 TRACE [hyper::client::pool] checkout dropped for ("http", 127.0.0.1:34801)
2020-04-07 00:13:13,589 TRACE [hyper::proto::h1::role] Client::encode method=GET, body=None
2020-04-07 00:13:13,589 TRACE [hyper::proto::h1::io] detected no usage of vectored write, flattening
2020-04-07 00:13:13,589 DEBUG [hyper::proto::h1::io] flushed 65 bytes
2020-04-07 00:13:13,590 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2020-04-07 00:13:13,600 TRACE [hyper::proto::h1::conn] Conn::read_head
2020-04-07 00:13:13,600 DEBUG [hyper::proto::h1::io] read 351 bytes
2020-04-07 00:13:13,600 TRACE [hyper::proto::h1::role] Response.parse([Header; 100], [u8; 351])
2020-04-07 00:13:13,600 TRACE [hyper::proto::h1::role] Response.parse Complete(109)
2020-04-07 00:13:13,600 DEBUG [hyper::proto::h1::io] parsed 3 headers
2020-04-07 00:13:13,600 DEBUG [hyper::proto::h1::conn] incoming body is content-length (242 bytes)
2020-04-07 00:13:13,600 TRACE [hyper::proto::h1::decode] decode; state=Length(242)
2020-04-07 00:13:13,600 DEBUG [hyper::proto::h1::conn] incoming body completed
2020-04-07 00:13:13,600 TRACE [hyper::proto::h1::conn] maybe_notify; read_from_io blocked
2020-04-07 00:13:13,601 TRACE [want] signal: Want
2020-04-07 00:13:13,601 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:13,601 TRACE [want] signal: Want
2020-04-07 00:13:13,601 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:13,601 TRACE [hyper::client::pool] put; add idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:13,601 DEBUG [hyper::client::pool] pooling idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:13,601 DEBUG [reqwest::async_impl::client] response '200 OK' for http://127.0.0.1:34801/search/dist
2020-04-07 00:13:13,601 INFO  [actix_web::middleware::logger] 127.0.0.1:61982 "GET /search/dist HTTP/1.1" 200 242 "-" "-" 0.014648
2020-04-07 00:13:13,601 TRACE [hyper::proto::h1::dispatch] client tx closed
2020-04-07 00:13:13,601 TRACE [hyper::proto::h1::conn] State::close_read()
2020-04-07 00:13:13,601 TRACE [hyper::proto::h1::conn] State::close_write()
2020-04-07 00:13:13,601 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2020-04-07 00:13:13,601 TRACE [hyper::proto::h1::conn] shut down IO complete
2020-04-07 00:13:13,601 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:13,601 TRACE [want] signal: Closed
2020-04-07 00:13:13,602 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:18,034 TRACE [mio::poll] registering with poller
2020-04-07 00:13:18,034 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:18,035 TRACE [hyper::client::pool] checkout waiting for idle connection: ("http", 127.0.0.1:34801)
2020-04-07 00:13:18,035 DEBUG [reqwest::connect] starting new connection: http://127.0.0.1:34801/
2020-04-07 00:13:18,035 TRACE [hyper::client::connect::http] Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(34801))
2020-04-07 00:13:18,035 DEBUG [hyper::client::connect::http] connecting to 127.0.0.1:34801
2020-04-07 00:13:18,035 TRACE [mio::poll] registering with poller
2020-04-07 00:13:18,035 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:18,036 DEBUG [hyper::client::connect::http] connected to 127.0.0.1:34801
2020-04-07 00:13:18,036 TRACE [hyper::client::conn] client handshake HTTP/1
2020-04-07 00:13:18,036 TRACE [hyper::client] handshake complete, spawning background dispatcher task
2020-04-07 00:13:18,036 TRACE [want] signal: Want
2020-04-07 00:13:18,036 TRACE [want] signal found waiting giver, notifying
2020-04-07 00:13:18,036 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2020-04-07 00:13:18,036 TRACE [want] poll_want: taker wants!
2020-04-07 00:13:18,036 TRACE [hyper::client::pool] checkout dropped for ("http", 127.0.0.1:34801)
2020-04-07 00:13:18,036 TRACE [hyper::proto::h1::role] Client::encode method=GET, body=None
2020-04-07 00:13:18,036 TRACE [hyper::proto::h1::io] detected no usage of vectored write, flattening
2020-04-07 00:13:18,036 DEBUG [hyper::proto::h1::io] flushed 60 bytes
2020-04-07 00:13:18,036 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2020-04-07 00:13:18,038 TRACE [hyper::proto::h1::conn] Conn::read_head
2020-04-07 00:13:18,038 DEBUG [hyper::proto::h1::io] read 576 bytes
2020-04-07 00:13:18,038 TRACE [hyper::proto::h1::role] Response.parse([Header; 100], [u8; 576])
2020-04-07 00:13:18,038 TRACE [hyper::proto::h1::role] Response.parse Complete(109)
2020-04-07 00:13:18,038 DEBUG [hyper::proto::h1::io] parsed 3 headers
2020-04-07 00:13:18,038 DEBUG [hyper::proto::h1::conn] incoming body is content-length (467 bytes)
2020-04-07 00:13:18,038 TRACE [hyper::proto::h1::decode] decode; state=Length(467)
2020-04-07 00:13:18,038 DEBUG [hyper::proto::h1::conn] incoming body completed
2020-04-07 00:13:18,038 TRACE [hyper::proto::h1::conn] maybe_notify; read_from_io blocked
2020-04-07 00:13:18,038 TRACE [want] signal: Want
2020-04-07 00:13:18,039 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:18,039 TRACE [want] signal: Want
2020-04-07 00:13:18,039 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:18,039 TRACE [hyper::client::pool] put; add idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:18,039 DEBUG [hyper::client::pool] pooling idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:18,039 DEBUG [reqwest::async_impl::client] response '200 OK' for http://127.0.0.1:34801/lookup
2020-04-07 00:13:18,039 INFO  [actix_web::middleware::logger] 127.0.0.1:61985 "GET /lookup HTTP/1.1" 200 467 "-" "-" 0.004626
2020-04-07 00:13:18,041 TRACE [hyper::proto::h1::dispatch] client tx closed
2020-04-07 00:13:18,041 TRACE [hyper::proto::h1::conn] State::close_read()
2020-04-07 00:13:18,041 TRACE [hyper::proto::h1::conn] State::close_write()
2020-04-07 00:13:18,041 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2020-04-07 00:13:18,041 TRACE [hyper::proto::h1::conn] shut down IO complete
2020-04-07 00:13:18,041 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:18,041 TRACE [want] signal: Closed
2020-04-07 00:13:18,041 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:34,223 TRACE [mio::poll] registering with poller
2020-04-07 00:13:34,223 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:34,224 TRACE [hyper::client::pool] checkout waiting for idle connection: ("http", 127.0.0.1:34801)
2020-04-07 00:13:34,224 DEBUG [reqwest::connect] starting new connection: http://127.0.0.1:34801/
2020-04-07 00:13:34,224 TRACE [hyper::client::connect::http] Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(34801))
2020-04-07 00:13:34,225 DEBUG [hyper::client::connect::http] connecting to 127.0.0.1:34801
2020-04-07 00:13:34,225 TRACE [mio::poll] registering with poller
2020-04-07 00:13:34,225 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:34,225 DEBUG [hyper::client::connect::http] connected to 127.0.0.1:34801
2020-04-07 00:13:34,225 TRACE [hyper::client::conn] client handshake HTTP/1
2020-04-07 00:13:34,225 TRACE [hyper::client] handshake complete, spawning background dispatcher task
2020-04-07 00:13:34,226 TRACE [want] signal: Want
2020-04-07 00:13:34,226 TRACE [want] signal found waiting giver, notifying
2020-04-07 00:13:34,226 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2020-04-07 00:13:34,226 TRACE [want] poll_want: taker wants!
2020-04-07 00:13:34,226 TRACE [hyper::client::pool] checkout dropped for ("http", 127.0.0.1:34801)
2020-04-07 00:13:34,226 TRACE [hyper::proto::h1::role] Client::encode method=GET, body=None
2020-04-07 00:13:34,226 TRACE [hyper::proto::h1::io] detected no usage of vectored write, flattening
2020-04-07 00:13:34,227 DEBUG [hyper::proto::h1::io] flushed 62 bytes
2020-04-07 00:13:34,227 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2020-04-07 00:13:34,229 TRACE [hyper::proto::h1::conn] Conn::read_head
2020-04-07 00:13:34,229 DEBUG [hyper::proto::h1::io] read 270 bytes
2020-04-07 00:13:34,229 TRACE [hyper::proto::h1::role] Response.parse([Header; 100], [u8; 270])
2020-04-07 00:13:34,229 TRACE [hyper::proto::h1::role] Response.parse Complete(109)
2020-04-07 00:13:34,229 DEBUG [hyper::proto::h1::io] parsed 3 headers
2020-04-07 00:13:34,229 DEBUG [hyper::proto::h1::conn] incoming body is content-length (161 bytes)
2020-04-07 00:13:34,229 TRACE [hyper::proto::h1::decode] decode; state=Length(161)
2020-04-07 00:13:34,229 DEBUG [hyper::proto::h1::conn] incoming body completed
2020-04-07 00:13:34,229 TRACE [hyper::proto::h1::conn] maybe_notify; read_from_io blocked
2020-04-07 00:13:34,229 TRACE [want] signal: Want
2020-04-07 00:13:34,229 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:34,229 TRACE [want] signal: Want
2020-04-07 00:13:34,229 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:34,229 TRACE [hyper::client::pool] put; add idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:34,229 DEBUG [hyper::client::pool] pooling idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:34,229 DEBUG [reqwest::async_impl::client] response '200 OK' for http://127.0.0.1:34801/lookup/3
2020-04-07 00:13:34,230 INFO  [actix_web::middleware::logger] 127.0.0.1:61988 "GET /lookup/3 HTTP/1.1" 200 161 "-" "-" 0.005913
2020-04-07 00:13:34,230 TRACE [hyper::proto::h1::dispatch] client tx closed
2020-04-07 00:13:34,230 TRACE [hyper::proto::h1::conn] State::close_read()
2020-04-07 00:13:34,230 TRACE [hyper::proto::h1::conn] State::close_write()
2020-04-07 00:13:34,230 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2020-04-07 00:13:34,230 TRACE [hyper::proto::h1::conn] shut down IO complete
2020-04-07 00:13:34,230 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:34,230 TRACE [want] signal: Closed
2020-04-07 00:13:34,231 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:38,462 TRACE [mio::poll] registering with poller
2020-04-07 00:13:38,462 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:38,462 TRACE [hyper::client::pool] checkout waiting for idle connection: ("http", 127.0.0.1:34801)
2020-04-07 00:13:38,463 DEBUG [reqwest::connect] starting new connection: http://127.0.0.1:34801/
2020-04-07 00:13:38,463 TRACE [hyper::client::connect::http] Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(34801))
2020-04-07 00:13:38,463 DEBUG [hyper::client::connect::http] connecting to 127.0.0.1:34801
2020-04-07 00:13:38,463 TRACE [mio::poll] registering with poller
2020-04-07 00:13:38,463 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:38,463 DEBUG [hyper::client::connect::http] connected to 127.0.0.1:34801
2020-04-07 00:13:38,463 TRACE [hyper::client::conn] client handshake HTTP/1
2020-04-07 00:13:38,463 TRACE [hyper::client] handshake complete, spawning background dispatcher task
2020-04-07 00:13:38,463 TRACE [want] signal: Want
2020-04-07 00:13:38,463 TRACE [want] signal found waiting giver, notifying
2020-04-07 00:13:38,463 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2020-04-07 00:13:38,464 TRACE [want] poll_want: taker wants!
2020-04-07 00:13:38,464 TRACE [hyper::client::pool] checkout dropped for ("http", 127.0.0.1:34801)
2020-04-07 00:13:38,464 TRACE [hyper::proto::h1::role] Client::encode method=GET, body=None
2020-04-07 00:13:38,464 TRACE [hyper::proto::h1::io] detected no usage of vectored write, flattening
2020-04-07 00:13:38,464 DEBUG [hyper::proto::h1::io] flushed 60 bytes
2020-04-07 00:13:38,464 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2020-04-07 00:13:38,466 TRACE [hyper::proto::h1::conn] Conn::read_head
2020-04-07 00:13:38,466 DEBUG [hyper::proto::h1::io] read 576 bytes
2020-04-07 00:13:38,466 TRACE [hyper::proto::h1::role] Response.parse([Header; 100], [u8; 576])
2020-04-07 00:13:38,466 TRACE [hyper::proto::h1::role] Response.parse Complete(109)
2020-04-07 00:13:38,466 DEBUG [hyper::proto::h1::io] parsed 3 headers
2020-04-07 00:13:38,466 DEBUG [hyper::proto::h1::conn] incoming body is content-length (467 bytes)
2020-04-07 00:13:38,466 TRACE [hyper::proto::h1::decode] decode; state=Length(467)
2020-04-07 00:13:38,466 DEBUG [hyper::proto::h1::conn] incoming body completed
2020-04-07 00:13:38,466 TRACE [hyper::proto::h1::conn] maybe_notify; read_from_io blocked
2020-04-07 00:13:38,466 TRACE [want] signal: Want
2020-04-07 00:13:38,466 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:38,467 TRACE [want] signal: Want
2020-04-07 00:13:38,467 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:38,467 TRACE [hyper::client::pool] put; add idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:38,467 DEBUG [hyper::client::pool] pooling idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:38,467 DEBUG [reqwest::async_impl::client] response '200 OK' for http://127.0.0.1:34801/lookup
2020-04-07 00:13:38,467 INFO  [actix_web::middleware::logger] 127.0.0.1:61991 "GET /lookup HTTP/1.1" 200 467 "-" "-" 0.005168
2020-04-07 00:13:38,467 TRACE [hyper::proto::h1::dispatch] client tx closed
2020-04-07 00:13:38,467 TRACE [hyper::proto::h1::conn] State::close_read()
2020-04-07 00:13:38,467 TRACE [hyper::proto::h1::conn] State::close_write()
2020-04-07 00:13:38,467 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2020-04-07 00:13:38,467 TRACE [hyper::proto::h1::conn] shut down IO complete
2020-04-07 00:13:38,467 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:38,467 TRACE [want] signal: Closed
2020-04-07 00:13:38,468 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,500 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,500 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,501 TRACE [hyper::client::pool] checkout waiting for idle connection: ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,501 DEBUG [reqwest::connect] starting new connection: http://127.0.0.1:34801/
2020-04-07 00:13:44,501 TRACE [hyper::client::connect::http] Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(34801))
2020-04-07 00:13:44,501 DEBUG [hyper::client::connect::http] connecting to 127.0.0.1:34801
2020-04-07 00:13:44,502 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,502 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,502 DEBUG [hyper::client::connect::http] connected to 127.0.0.1:34801
2020-04-07 00:13:44,502 TRACE [hyper::client::conn] client handshake HTTP/1
2020-04-07 00:13:44,502 TRACE [hyper::client] handshake complete, spawning background dispatcher task
2020-04-07 00:13:44,502 TRACE [want] signal: Want
2020-04-07 00:13:44,502 TRACE [want] signal found waiting giver, notifying
2020-04-07 00:13:44,502 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2020-04-07 00:13:44,502 TRACE [want] poll_want: taker wants!
2020-04-07 00:13:44,502 TRACE [hyper::client::pool] checkout dropped for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,502 TRACE [hyper::proto::h1::role] Client::encode method=GET, body=None
2020-04-07 00:13:44,503 TRACE [hyper::proto::h1::io] detected no usage of vectored write, flattening
2020-04-07 00:13:44,503 DEBUG [hyper::proto::h1::io] flushed 62 bytes
2020-04-07 00:13:44,503 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2020-04-07 00:13:44,504 TRACE [hyper::proto::h1::conn] Conn::read_head
2020-04-07 00:13:44,504 DEBUG [hyper::proto::h1::io] read 280 bytes
2020-04-07 00:13:44,504 TRACE [hyper::proto::h1::role] Response.parse([Header; 100], [u8; 280])
2020-04-07 00:13:44,504 TRACE [hyper::proto::h1::role] Response.parse Complete(109)
2020-04-07 00:13:44,504 DEBUG [hyper::proto::h1::io] parsed 3 headers
2020-04-07 00:13:44,504 DEBUG [hyper::proto::h1::conn] incoming body is content-length (171 bytes)
2020-04-07 00:13:44,504 TRACE [hyper::proto::h1::decode] decode; state=Length(171)
2020-04-07 00:13:44,504 DEBUG [hyper::proto::h1::conn] incoming body completed
2020-04-07 00:13:44,504 TRACE [hyper::proto::h1::conn] maybe_notify; read_from_io blocked
2020-04-07 00:13:44,504 TRACE [want] signal: Want
2020-04-07 00:13:44,504 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:44,504 TRACE [want] signal: Want
2020-04-07 00:13:44,504 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:44,507 TRACE [hyper::client::pool] put; add idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,507 DEBUG [hyper::client::pool] pooling idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,507 DEBUG [reqwest::async_impl::client] response '200 OK' for http://127.0.0.1:34801/lookup/1
2020-04-07 00:13:44,508 INFO  [actix_web::middleware::logger] 127.0.0.1:61993 "GET /lookup/1 HTTP/1.1" 200 171 "-" "-" 0.007347
2020-04-07 00:13:44,508 TRACE [hyper::proto::h1::dispatch] client tx closed
2020-04-07 00:13:44,508 TRACE [hyper::proto::h1::conn] State::close_read()
2020-04-07 00:13:44,508 TRACE [hyper::proto::h1::conn] State::close_write()
2020-04-07 00:13:44,508 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2020-04-07 00:13:44,508 TRACE [hyper::proto::h1::conn] shut down IO complete
2020-04-07 00:13:44,508 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,508 TRACE [want] signal: Closed
2020-04-07 00:13:44,509 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,510 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,510 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,511 TRACE [hyper::client::pool] checkout waiting for idle connection: ("http", 127.0.0.1:34802)
2020-04-07 00:13:44,512 DEBUG [reqwest::connect] starting new connection: http://127.0.0.1:34802/
2020-04-07 00:13:44,512 TRACE [hyper::client::connect::http] Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(34802))
2020-04-07 00:13:44,512 DEBUG [hyper::client::connect::http] connecting to 127.0.0.1:34802
2020-04-07 00:13:44,512 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,513 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,513 DEBUG [hyper::client::connect::http] connected to 127.0.0.1:34802
2020-04-07 00:13:44,513 TRACE [hyper::client::conn] client handshake HTTP/1
2020-04-07 00:13:44,513 TRACE [hyper::client] handshake complete, spawning background dispatcher task
2020-04-07 00:13:44,514 TRACE [want] signal: Want
2020-04-07 00:13:44,514 TRACE [want] signal found waiting giver, notifying
2020-04-07 00:13:44,514 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2020-04-07 00:13:44,514 TRACE [want] poll_want: taker wants!
2020-04-07 00:13:44,514 TRACE [hyper::client::pool] checkout dropped for ("http", 127.0.0.1:34802)
2020-04-07 00:13:44,514 TRACE [hyper::proto::h1::role] Client::encode method=POST, body=None
2020-04-07 00:13:44,515 TRACE [hyper::proto::h1::io] detected no usage of vectored write, flattening
2020-04-07 00:13:44,515 DEBUG [hyper::proto::h1::io] flushed 71 bytes
2020-04-07 00:13:44,515 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2020-04-07 00:13:44,534 TRACE [hyper::proto::h1::conn] Conn::read_head
2020-04-07 00:13:44,534 DEBUG [hyper::proto::h1::io] read 111 bytes
2020-04-07 00:13:44,534 TRACE [hyper::proto::h1::role] Response.parse([Header; 100], [u8; 111])
2020-04-07 00:13:44,534 TRACE [hyper::proto::h1::role] Response.parse Complete(107)
2020-04-07 00:13:44,535 DEBUG [hyper::proto::h1::io] parsed 3 headers
2020-04-07 00:13:44,535 DEBUG [hyper::proto::h1::conn] incoming body is content-length (4 bytes)
2020-04-07 00:13:44,535 TRACE [hyper::proto::h1::decode] decode; state=Length(4)
2020-04-07 00:13:44,535 DEBUG [hyper::proto::h1::conn] incoming body completed
2020-04-07 00:13:44,535 TRACE [hyper::proto::h1::conn] maybe_notify; read_from_io blocked
2020-04-07 00:13:44,535 TRACE [want] signal: Want
2020-04-07 00:13:44,535 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:44,535 TRACE [want] signal: Want
2020-04-07 00:13:44,535 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:44,535 TRACE [hyper::client::pool] put; add idle connection for ("http", 127.0.0.1:34802)
2020-04-07 00:13:44,535 DEBUG [hyper::client::pool] pooling idle connection for ("http", 127.0.0.1:34802)
2020-04-07 00:13:44,535 DEBUG [reqwest::async_impl::client] response '200 OK' for http://127.0.0.1:34802/order/1?amount=1
2020-04-07 00:13:44,535 INFO  [actix_web::middleware::logger] 127.0.0.1:61995 "POST /order/1?amount=1 HTTP/1.1" 200 4 "-" "-" 0.024697
2020-04-07 00:13:44,536 TRACE [hyper::proto::h1::dispatch] client tx closed
2020-04-07 00:13:44,536 TRACE [hyper::proto::h1::conn] State::close_read()
2020-04-07 00:13:44,536 TRACE [hyper::proto::h1::conn] State::close_write()
2020-04-07 00:13:44,536 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2020-04-07 00:13:44,536 TRACE [hyper::proto::h1::conn] shut down IO complete
2020-04-07 00:13:44,536 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,536 TRACE [want] signal: Closed
2020-04-07 00:13:44,536 TRACE [mio::poll] deregistering handle with poller
```

## Catalog Server
```
2020-04-07 00:11:32,471 INFO  [actix_server::builder] Starting 12 workers
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,473 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,473 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,473 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,473 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34801" service on 0.0.0.0:34801
2020-04-07 00:11:32,474 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,474 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,474 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,474 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,474 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,474 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,474 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,474 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,474 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:32,475 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,475 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:32,475 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,475 TRACE [mio::sys::unix::kqueue] registering; token=Token(100); interests=Readable
2020-04-07 00:11:32,475 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,475 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,475 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,475 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,475 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:32,475 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,475 TRACE [mio::sys::unix::kqueue] registering; token=Token(2); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:32,475 TRACE [mio::poll] registering with poller
2020-04-07 00:11:32,475 TRACE [mio::sys::unix::kqueue] registering; token=Token(3); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:32,478 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,480 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,481 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,482 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,483 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,485 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,486 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,488 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,489 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,491 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,492 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,495 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,496 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,498 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,498 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,500 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,501 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,503 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,503 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,505 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,506 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:32,507 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:11:32,510 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34801" is available
2020-04-07 00:13:13,588 TRACE [mio::poll] registering with poller
2020-04-07 00:13:13,588 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:13,600 INFO  [actix_web::middleware::logger] 127.0.0.1:61983 "GET /search/dist HTTP/1.1" 200 242 "-" "-" 0.010026
2020-04-07 00:13:13,602 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:18,036 TRACE [mio::poll] registering with poller
2020-04-07 00:13:18,036 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:18,038 INFO  [actix_web::middleware::logger] 127.0.0.1:61986 "GET /lookup HTTP/1.1" 200 467 "-" "-" 0.001274
2020-04-07 00:13:18,041 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:34,225 TRACE [mio::poll] registering with poller
2020-04-07 00:13:34,225 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:34,228 INFO  [actix_web::middleware::logger] 127.0.0.1:61989 "GET /lookup/3 HTTP/1.1" 200 161 "-" "-" 0.001436
2020-04-07 00:13:34,230 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:38,463 TRACE [mio::poll] registering with poller
2020-04-07 00:13:38,463 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:38,466 INFO  [actix_web::middleware::logger] 127.0.0.1:61992 "GET /lookup HTTP/1.1" 200 467 "-" "-" 0.001316
2020-04-07 00:13:38,468 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,502 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,502 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,504 INFO  [actix_web::middleware::logger] 127.0.0.1:61994 "GET /lookup/1 HTTP/1.1" 200 171 "-" "-" 0.001006
2020-04-07 00:13:44,509 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,517 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,517 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,519 INFO  [actix_web::middleware::logger] 127.0.0.1:61997 "GET /lookup/1 HTTP/1.1" 200 171 "-" "-" 0.001090
2020-04-07 00:13:44,522 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,522 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,522 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,526 INFO  [actix_web::middleware::logger] 127.0.0.1:61998 "POST /update/1/stock/deduct/1 HTTP/1.1" 200 4 "-" "-" 0.002422
2020-04-07 00:13:44,535 TRACE [mio::poll] deregistering handle with poller
```

## Order Server
```
2020-04-07 00:11:37,883 INFO  [actix_server::builder] Starting 12 workers
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,884 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,884 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,884 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,885 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,884 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34802" service on 0.0.0.0:34802
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,884 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,885 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,884 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,885 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,885 TRACE [mio::sys::unix::kqueue] registering; token=Token(18446744073709551615); interests=Readable
2020-04-07 00:11:37,885 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,885 TRACE [mio::sys::unix::kqueue] registering; token=Token(100); interests=Readable
2020-04-07 00:11:37,885 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,885 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,885 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,885 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,885 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,885 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:37,886 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,886 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,886 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,886 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:37,886 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,886 TRACE [mio::sys::unix::kqueue] registering; token=Token(2); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:37,886 TRACE [mio::poll] registering with poller
2020-04-07 00:11:37,886 TRACE [mio::sys::unix::kqueue] registering; token=Token(3); interests=Readable | Writable | Error | Hup
2020-04-07 00:11:37,888 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,890 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,893 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,894 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,896 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,897 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,899 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,899 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,901 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,903 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,905 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,906 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,908 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,909 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,912 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,913 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,915 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,916 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,918 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,918 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,920 TRACE [actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
2020-04-07 00:11:37,920 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:11:37,923 TRACE [actix_server::worker] Service "actix-web-service-0.0.0.0:34802" is available
2020-04-07 00:13:44,513 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,513 TRACE [mio::sys::unix::kqueue] registering; token=Token(0); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,515 INFO  [order_server] Ordering 1, amount 1. Checking stock on catalog server.
2020-04-07 00:13:44,516 TRACE [hyper::client::pool] checkout waiting for idle connection: ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,516 DEBUG [reqwest::connect] starting new connection: http://127.0.0.1:34801/
2020-04-07 00:13:44,516 TRACE [hyper::client::connect::http] Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(34801))
2020-04-07 00:13:44,516 DEBUG [hyper::client::connect::http] connecting to 127.0.0.1:34801
2020-04-07 00:13:44,516 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,516 TRACE [mio::sys::unix::kqueue] registering; token=Token(1); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,517 DEBUG [hyper::client::connect::http] connected to 127.0.0.1:34801
2020-04-07 00:13:44,517 TRACE [hyper::client::conn] client handshake HTTP/1
2020-04-07 00:13:44,517 TRACE [hyper::client] handshake complete, spawning background dispatcher task
2020-04-07 00:13:44,517 TRACE [want] signal: Want
2020-04-07 00:13:44,517 TRACE [want] signal found waiting giver, notifying
2020-04-07 00:13:44,517 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2020-04-07 00:13:44,517 TRACE [want] poll_want: taker wants!
2020-04-07 00:13:44,517 TRACE [hyper::client::pool] checkout dropped for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,517 TRACE [hyper::proto::h1::role] Client::encode method=GET, body=None
2020-04-07 00:13:44,517 TRACE [hyper::proto::h1::io] detected no usage of vectored write, flattening
2020-04-07 00:13:44,518 DEBUG [hyper::proto::h1::io] flushed 62 bytes
2020-04-07 00:13:44,518 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2020-04-07 00:13:44,519 TRACE [hyper::proto::h1::conn] Conn::read_head
2020-04-07 00:13:44,519 DEBUG [hyper::proto::h1::io] read 280 bytes
2020-04-07 00:13:44,519 TRACE [hyper::proto::h1::role] Response.parse([Header; 100], [u8; 280])
2020-04-07 00:13:44,519 TRACE [hyper::proto::h1::role] Response.parse Complete(109)
2020-04-07 00:13:44,519 DEBUG [hyper::proto::h1::io] parsed 3 headers
2020-04-07 00:13:44,520 DEBUG [hyper::proto::h1::conn] incoming body is content-length (171 bytes)
2020-04-07 00:13:44,520 TRACE [hyper::proto::h1::decode] decode; state=Length(171)
2020-04-07 00:13:44,520 DEBUG [hyper::proto::h1::conn] incoming body completed
2020-04-07 00:13:44,520 TRACE [hyper::proto::h1::conn] maybe_notify; read_from_io blocked
2020-04-07 00:13:44,520 TRACE [want] signal: Want
2020-04-07 00:13:44,520 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:44,520 TRACE [want] signal: Want
2020-04-07 00:13:44,520 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:44,520 TRACE [hyper::client::pool] put; add idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,520 DEBUG [hyper::client::pool] pooling idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,521 DEBUG [reqwest::async_impl::client] response '200 OK' for http://127.0.0.1:34801/lookup/1
2020-04-07 00:13:44,521 INFO  [order_server] Found item Item { id: 1, name: "How to get a good grade in 677 in 20 minutes a day", stock: 5, price: 19.9, topic: 1 }, start transaction
2020-04-07 00:13:44,521 TRACE [hyper::client::pool] checkout waiting for idle connection: ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,521 DEBUG [reqwest::connect] starting new connection: http://127.0.0.1:34801/
2020-04-07 00:13:44,521 TRACE [hyper::client::connect::http] Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(34801))
2020-04-07 00:13:44,522 DEBUG [hyper::client::connect::http] connecting to 127.0.0.1:34801
2020-04-07 00:13:44,522 TRACE [mio::poll] registering with poller
2020-04-07 00:13:44,522 TRACE [mio::sys::unix::kqueue] registering; token=Token(2); interests=Readable | Writable | Error | Hup
2020-04-07 00:13:44,522 TRACE [hyper::proto::h1::dispatch] client tx closed
2020-04-07 00:13:44,522 TRACE [hyper::proto::h1::conn] State::close_read()
2020-04-07 00:13:44,522 TRACE [hyper::proto::h1::conn] State::close_write()
2020-04-07 00:13:44,522 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2020-04-07 00:13:44,522 TRACE [hyper::proto::h1::conn] shut down IO complete
2020-04-07 00:13:44,522 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,522 TRACE [want] signal: Closed
2020-04-07 00:13:44,523 DEBUG [hyper::client::connect::http] connected to 127.0.0.1:34801
2020-04-07 00:13:44,523 TRACE [hyper::client::conn] client handshake HTTP/1
2020-04-07 00:13:44,523 TRACE [hyper::client] handshake complete, spawning background dispatcher task
2020-04-07 00:13:44,523 TRACE [want] signal: Want
2020-04-07 00:13:44,523 TRACE [want] signal found waiting giver, notifying
2020-04-07 00:13:44,523 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2020-04-07 00:13:44,523 TRACE [want] poll_want: taker wants!
2020-04-07 00:13:44,523 TRACE [hyper::client::pool] checkout dropped for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,523 TRACE [hyper::proto::h1::role] Client::encode method=POST, body=None
2020-04-07 00:13:44,523 TRACE [hyper::proto::h1::io] detected no usage of vectored write, flattening
2020-04-07 00:13:44,523 DEBUG [hyper::proto::h1::io] flushed 78 bytes
2020-04-07 00:13:44,523 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2020-04-07 00:13:44,526 TRACE [hyper::proto::h1::conn] Conn::read_head
2020-04-07 00:13:44,526 DEBUG [hyper::proto::h1::io] read 111 bytes
2020-04-07 00:13:44,526 TRACE [hyper::proto::h1::role] Response.parse([Header; 100], [u8; 111])
2020-04-07 00:13:44,526 TRACE [hyper::proto::h1::role] Response.parse Complete(107)
2020-04-07 00:13:44,527 DEBUG [hyper::proto::h1::io] parsed 3 headers
2020-04-07 00:13:44,527 DEBUG [hyper::proto::h1::conn] incoming body is content-length (4 bytes)
2020-04-07 00:13:44,527 TRACE [hyper::proto::h1::decode] decode; state=Length(4)
2020-04-07 00:13:44,527 DEBUG [hyper::proto::h1::conn] incoming body completed
2020-04-07 00:13:44,527 TRACE [hyper::proto::h1::conn] maybe_notify; read_from_io blocked
2020-04-07 00:13:44,527 TRACE [want] signal: Want
2020-04-07 00:13:44,527 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:44,527 TRACE [want] signal: Want
2020-04-07 00:13:44,527 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Idle }
2020-04-07 00:13:44,527 TRACE [hyper::client::pool] put; add idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,527 DEBUG [hyper::client::pool] pooling idle connection for ("http", 127.0.0.1:34801)
2020-04-07 00:13:44,527 DEBUG [reqwest::async_impl::client] response '200 OK' for http://127.0.0.1:34801/update/1/stock/deduct/1
2020-04-07 00:13:44,527 INFO  [order_server] Order transaction for 1 successful, log transaction
2020-04-07 00:13:44,534 INFO  [actix_web::middleware::logger] 127.0.0.1:61996 "POST /order/1?amount=1 HTTP/1.1" 200 4 "-" "-" 0.019140
2020-04-07 00:13:44,534 TRACE [hyper::proto::h1::dispatch] client tx closed
2020-04-07 00:13:44,534 TRACE [hyper::proto::h1::conn] State::close_read()
2020-04-07 00:13:44,534 TRACE [hyper::proto::h1::conn] State::close_write()
2020-04-07 00:13:44,534 TRACE [hyper::proto::h1::conn] flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2020-04-07 00:13:44,534 TRACE [hyper::proto::h1::conn] shut down IO complete
2020-04-07 00:13:44,534 TRACE [mio::poll] deregistering handle with poller
2020-04-07 00:13:44,534 TRACE [want] signal: Closed
2020-04-07 00:13:44,536 TRACE [mio::poll] deregistering handle with poller
```