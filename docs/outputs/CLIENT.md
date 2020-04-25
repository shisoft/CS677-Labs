# Outputs
Following content contains output from the client, frontend server, catalog server and order server.
Note that because the fontend interfaces have not been changed, Client output is identical with Lab 2.
## Client
```
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
sys
1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 4
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 16
Press return key to continue

1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
5. Test
6. Leave
Select by input the number: 
2
1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 4
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 16
3. Xen and the Art of Surviving Graduate School - Price: 9.9, Topic: Graduate School, Stock: 25
4. Cooking for the Impatient Graduate Student - Price: 30.9, Topic: Graduate School, Stock: 15
All books listed.
Press return key to continue
3
1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
5. Test
6. Leave
Select by input the number: 
1
What is the topic? Any keywords in the topics will be fine (try 'sys'):
sys
1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 4
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 16
Press return key to continue
    
1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
5. Test
6. Leave
Select by input the number: 
4
1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 4
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 16
3. Xen and the Art of Surviving Graduate School - Price: 9.9, Topic: Graduate School, Stock: 25
4. Cooking for the Impatient Graduate Student - Price: 30.9, Topic: Graduate School, Stock: 15
Which book would you like: 
2
amount: 
1
Bought book RPCs for Dummies, amount 1
Press return key to continue
```

## Cluster
### Startup Outputs
```
catalog-2           | 2020-04-25 21:34:39,956 INFO  [catalog_server] Running Catalog server
catalog-2           | 2020-04-25 21:34:39,956 INFO  [catalog_server] Catalog server have peers: ["catalog-1", "catalog-2", "catalog-3"]
catalog-1           | 2020-04-25 21:34:40,153 INFO  [catalog_server] Running Catalog server
catalog-1           | 2020-04-25 21:34:40,153 INFO  [catalog_server] Catalog server have peers: ["catalog-1", "catalog-2", "catalog-3"]
catalog-1           | 2020-04-25 21:34:40,154 DEBUG [bifrost::raft::disk] Recovered 0 raft logs
catalog-2           | 2020-04-25 21:34:39,957 DEBUG [bifrost::raft::disk] Recovered 0 raft logs
catalog-1           | SERVICE SHORTCUT DISABLED
catalog-3           | 2020-04-25 21:34:39,690 INFO  [catalog_server] Running Catalog server
catalog-3           | 2020-04-25 21:34:39,690 INFO  [catalog_server] Catalog server have peers: ["catalog-1", "catalog-2", "catalog-3"]
catalog-2           | SERVICE SHORTCUT DISABLED
catalog-3           | 2020-04-25 21:34:39,690 DEBUG [bifrost::raft::disk] Recovered 0 raft logs
catalog-3           | SERVICE SHORTCUT DISABLED
fontend             | 2020-04-25 21:34:40,153 INFO  [actix_server::builder] Starting 128 workers
fontend             | 2020-04-25 21:34:40,160 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34800" service on 0.0.0.0:34800
order-1             | 2020-04-25 21:34:39,697 INFO  [order_server] Running Order server
order-1             | 2020-04-25 21:34:39,697 DEBUG [bifrost::raft::disk] Recovered 0 raft logs
order-2             | 2020-04-25 21:34:39,953 INFO  [order_server] Running Order server
order-1             | SERVICE SHORTCUT DISABLED
order-2             | 2020-04-25 21:34:39,954 DEBUG [bifrost::raft::disk] Recovered 0 raft logs
order-3             | 2020-04-25 21:34:40,030 INFO  [order_server] Running Order server
order-3             | 2020-04-25 21:34:40,030 DEBUG [bifrost::raft::disk] Recovered 0 raft logs
order-2             | SERVICE SHORTCUT DISABLED
order-3             | SERVICE SHORTCUT DISABLED
catalog-3           | 2020-04-25 21:34:40,692 INFO  [bifrost::raft] Waiting for raft server to be initialized
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [catalog_server::configs] Server list: ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [catalog_server::configs] Will join server list
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::raft] Trying to join cluster with id 4820394039294786766
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::raft::client] Getting server info for ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]...10
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::raft::client] Checking server info on catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::raft::client] Connecting to node catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-3           | 2020-04-25 21:34:40,692 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Added server info on catalog-1:34803 to members
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Member catalog-1:34803 added
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Getting server info from catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Checking server client catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Checking response from catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Discovered zero leader id from catalog-1:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Checking server info on catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Connecting to node catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Added server info on catalog-2:34803 to members
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Member catalog-2:34803 added
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Getting server info from catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Checking server client catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,693 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Checking response from catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Discovered zero leader id from catalog-2:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Checking server info on catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Connecting to node catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Added server info on catalog-3:34803 to members
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Member catalog-3:34803 added
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Getting server info from catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Checking server client catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Checking response from catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] Discovered zero leader id from catalog-3:34803
catalog-3           | 2020-04-25 21:34:40,694 DEBUG [bifrost::raft::client] This fail attempt have zero leader id, retry...10
order-1             | 2020-04-25 21:34:40,700 INFO  [bifrost::raft] Waiting for raft server to be initialized
order-1             | 2020-04-25 21:34:40,700 DEBUG [bifrost::tcp::client] TCP connect to order-1:34803, server id 13433766016950577570, timeout 2000ms
order-1             | 2020-04-25 21:34:40,700 DEBUG [bifrost::tcp::client] Create socket on order-1:34803
order-1             | 2020-04-25 21:34:40,700 DEBUG [bifrost::tcp::client] Streaming messages for order-1:34803
order-1             | 2020-04-25 21:34:40,701 DEBUG [order_server::configs] Server list: ["order-1:34803", "order-2:34803", "order-3:34803"]
order-1             | 2020-04-25 21:34:40,701 DEBUG [order_server::configs] Will bootstrap
order-1             | 2020-04-25 21:34:40,701 DEBUG [bifrost::raft] Conservative bootstrap, checking storage
order-1             | 2020-04-25 21:34:40,701 DEBUG [bifrost::raft] There are storage, checking last term
order-1             | 2020-04-25 21:34:40,701 DEBUG [bifrost::raft] Log is empty, bootstrap
order-1             | 2020-04-25 21:34:40,701 DEBUG [bifrost::raft] Server 13433766016950577570 become leader, term 0
order-1             | 2020-04-25 21:34:40,701 INFO  [actix_server::builder] Starting 128 workers
order-1             | 2020-04-25 21:34:40,708 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34802" service on 0.0.0.0:34802
order-2             | 2020-04-25 21:34:40,955 INFO  [bifrost::raft] Waiting for raft server to be initialized
order-2             | 2020-04-25 21:34:40,955 DEBUG [bifrost::tcp::client] TCP connect to order-2:34803, server id 16848524676177999986, timeout 2000ms
order-2             | 2020-04-25 21:34:40,955 DEBUG [bifrost::tcp::client] Create socket on order-2:34803
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::tcp::client] Streaming messages for order-2:34803
order-2             | 2020-04-25 21:34:40,956 DEBUG [order_server::configs] Server list: ["order-1:34803", "order-2:34803", "order-3:34803"]
order-2             | 2020-04-25 21:34:40,956 DEBUG [order_server::configs] Will join server list
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft] Trying to join cluster with id 16848524676177999986
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Getting server info for ["order-1:34803", "order-2:34803", "order-3:34803"]
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["order-1:34803", "order-2:34803", "order-3:34803"]...10
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Checking server info on order-1:34803
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Connecting to node order-1:34803
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::tcp::client] TCP connect to order-1:34803, server id 13433766016950577570, timeout 2000ms
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::tcp::client] Create socket on order-1:34803
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::tcp::client] Streaming messages for order-1:34803
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Added server info on order-1:34803 to members
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Member order-1:34803 added
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Getting server info from order-1:34803
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Checking server client order-1:34803
order-2             | 2020-04-25 21:34:40,956 DEBUG [bifrost::raft::client] Invoking server_cluster_info on order-1:34803
order-2             | 2020-04-25 21:34:40,957 DEBUG [bifrost::raft::client] Checking response from order-1:34803
order-2             | 2020-04-25 21:34:40,957 DEBUG [bifrost::raft::client] Found server info with leader id 13433766016950577570
order-2             | 2020-04-25 21:34:40,957 DEBUG [bifrost::raft] Executing in SM to create new member order-2:34803, 16848524676177999986
order-2             | 2020-04-25 21:34:40,957 DEBUG [bifrost::raft::client] Leader id for client: 13433766016950577570
catalog-2           | 2020-04-25 21:34:40,959 INFO  [bifrost::raft] Waiting for raft server to be initialized
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [catalog_server::configs] Server list: ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [catalog_server::configs] Will join server list
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::raft] Trying to join cluster with id 16030156735997574805
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::raft::client] Getting server info for ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]...10
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::raft::client] Checking server info on catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::raft::client] Connecting to node catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-2           | 2020-04-25 21:34:40,959 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Added server info on catalog-1:34803 to members
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Member catalog-1:34803 added
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Getting server info from catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Checking server client catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Checking response from catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Discovered zero leader id from catalog-1:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Checking server info on catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::raft::client] Connecting to node catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-2           | 2020-04-25 21:34:40,960 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Added server info on catalog-2:34803 to members
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Member catalog-2:34803 added
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Getting server info from catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Checking server client catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Checking response from catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Discovered zero leader id from catalog-2:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Checking server info on catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Connecting to node catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Added server info on catalog-3:34803 to members
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Member catalog-3:34803 added
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Getting server info from catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Checking server client catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Checking response from catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] Discovered zero leader id from catalog-3:34803
catalog-2           | 2020-04-25 21:34:40,961 DEBUG [bifrost::raft::client] This fail attempt have zero leader id, retry...10
order-1             | 2020-04-25 21:34:40,974 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 0, appended [1]
order-1             | 2020-04-25 21:34:40,974 DEBUG [bifrost::raft] Sync config to followers
order-1             | 2020-04-25 21:34:40,974 DEBUG [bifrost::tcp::client] TCP connect to order-2:34803, server id 16848524676177999986, timeout 2000ms
order-1             | 2020-04-25 21:34:40,974 DEBUG [bifrost::tcp::client] Create socket on order-2:34803
order-1             | 2020-04-25 21:34:40,975 DEBUG [bifrost::tcp::client] Streaming messages for order-2:34803
order-1             | 2020-04-25 21:34:40,975 DEBUG [bifrost::raft] Log mismatch in follower 16848524676177999986, index 2
order-2             | 2020-04-25 21:34:40,979 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 0, appended [1]
order-1             | 2020-04-25 21:34:40,979 DEBUG [bifrost::raft] Heartbeat response from 16848524676177999986 is 1
order-1             | 2020-04-25 21:34:40,979 DEBUG [bifrost::raft] Members 1 granted 1, is majority: true
order-2             | 2020-04-25 21:34:40,979 DEBUG [bifrost::raft] Getting member address: 16848524676177999986
order-2             | 2020-04-25 21:34:40,979 DEBUG [bifrost::raft] Updating local meta by acquiring lock: 16848524676177999986
order-2             | 2020-04-25 21:34:40,979 DEBUG [bifrost::raft] Local meta lock acquired: 16848524676177999986
order-2             | 2020-04-25 21:34:40,979 DEBUG [bifrost::raft] We have following members for 16848524676177999986: ["order-1:34803", "order-2:34803"]
order-2             | 2020-04-25 21:34:40,979 DEBUG [bifrost::tcp::client] TCP connect to order-1:34803, server id 13433766016950577570, timeout 2000ms
order-2             | 2020-04-25 21:34:40,979 DEBUG [bifrost::tcp::client] Create socket on order-1:34803
order-2             | 2020-04-25 21:34:40,980 DEBUG [bifrost::tcp::client] Streaming messages for order-1:34803
order-2             | 2020-04-25 21:34:40,980 DEBUG [bifrost::raft] Become follower bacause of join: 16848524676177999986
order-2             | 2020-04-25 21:34:40,980 DEBUG [bifrost::raft] Resetting last checked for join: 16848524676177999986
order-2             | 2020-04-25 21:34:40,980 DEBUG [bifrost::raft] Completed join for 16848524676177999986, result true
order-2             | 2020-04-25 21:34:40,980 INFO  [actix_server::builder] Starting 128 workers
order-2             | 2020-04-25 21:34:40,988 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34802" service on 0.0.0.0:34802
order-3             | 2020-04-25 21:34:41,033 INFO  [bifrost::raft] Waiting for raft server to be initialized
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::tcp::client] TCP connect to order-3:34803, server id 4751096279368508625, timeout 2000ms
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::tcp::client] Create socket on order-3:34803
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::tcp::client] Streaming messages for order-3:34803
order-3             | 2020-04-25 21:34:41,033 DEBUG [order_server::configs] Server list: ["order-1:34803", "order-2:34803", "order-3:34803"]
order-3             | 2020-04-25 21:34:41,033 DEBUG [order_server::configs] Will join server list
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::raft] Trying to join cluster with id 4751096279368508625
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::raft::client] Getting server info for ["order-1:34803", "order-2:34803", "order-3:34803"]
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["order-1:34803", "order-2:34803", "order-3:34803"]...10
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::raft::client] Checking server info on order-1:34803
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::raft::client] Connecting to node order-1:34803
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::tcp::client] TCP connect to order-1:34803, server id 13433766016950577570, timeout 2000ms
order-3             | 2020-04-25 21:34:41,033 DEBUG [bifrost::tcp::client] Create socket on order-1:34803
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::tcp::client] Streaming messages for order-1:34803
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft::client] Added server info on order-1:34803 to members
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft::client] Member order-1:34803 added
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft::client] Getting server info from order-1:34803
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft::client] Checking server client order-1:34803
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft::client] Invoking server_cluster_info on order-1:34803
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft::client] Checking response from order-1:34803
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft::client] Found server info with leader id 13433766016950577570
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::tcp::client] TCP connect to order-2:34803, server id 16848524676177999986, timeout 2000ms
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::tcp::client] Create socket on order-2:34803
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::tcp::client] Streaming messages for order-2:34803
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft] Executing in SM to create new member order-3:34803, 4751096279368508625
order-3             | 2020-04-25 21:34:41,034 DEBUG [bifrost::raft::client] Leader id for client: 13433766016950577570
order-1             | 2020-04-25 21:34:41,065 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 1, appended [2]
order-1             | 2020-04-25 21:34:41,065 DEBUG [bifrost::raft] Sync config to followers
order-1             | 2020-04-25 21:34:41,065 DEBUG [bifrost::tcp::client] TCP connect to order-3:34803, server id 4751096279368508625, timeout 2000ms
order-1             | 2020-04-25 21:34:41,065 DEBUG [bifrost::tcp::client] Create socket on order-3:34803
order-1             | 2020-04-25 21:34:41,065 DEBUG [bifrost::tcp::client] Streaming messages for order-3:34803
order-1             | 2020-04-25 21:34:41,065 DEBUG [bifrost::raft] Log mismatch in follower 4751096279368508625, index 3
order-1             | 2020-04-25 21:34:41,066 DEBUG [bifrost::raft] Log mismatch in follower 4751096279368508625, index 2
order-2             | 2020-04-25 21:34:41,069 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 1, appended [2]
order-2             | 2020-04-25 21:34:41,069 DEBUG [bifrost::tcp::client] TCP connect to order-3:34803, server id 4751096279368508625, timeout 2000ms
order-2             | 2020-04-25 21:34:41,069 DEBUG [bifrost::tcp::client] Create socket on order-3:34803
order-2             | 2020-04-25 21:34:41,069 DEBUG [bifrost::tcp::client] Streaming messages for order-3:34803
order-1             | 2020-04-25 21:34:41,069 DEBUG [bifrost::raft] Heartbeat response from 16848524676177999986 is 2
order-1             | 2020-04-25 21:34:41,070 DEBUG [bifrost::raft] Members 2 granted 1, is majority: false
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::raft::disk] Appended and persisted 2 logs, was 0, appended [1, 2]
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::tcp::client] TCP connect to order-2:34803, server id 16848524676177999986, timeout 2000ms
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::tcp::client] Create socket on order-2:34803
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::tcp::client] Streaming messages for order-2:34803
order-1             | 2020-04-25 21:34:41,073 DEBUG [bifrost::raft] Heartbeat response from 4751096279368508625 is 2
order-1             | 2020-04-25 21:34:41,073 DEBUG [bifrost::raft] Members 2 granted 2, is majority: true
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::raft] Getting member address: 4751096279368508625
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::raft] Updating local meta by acquiring lock: 4751096279368508625
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::raft] Local meta lock acquired: 4751096279368508625
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::raft] We have following members for 4751096279368508625: ["order-3:34803", "order-1:34803", "order-2:34803"]
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::tcp::client] TCP connect to order-1:34803, server id 13433766016950577570, timeout 2000ms
order-3             | 2020-04-25 21:34:41,073 DEBUG [bifrost::tcp::client] Create socket on order-1:34803
order-3             | 2020-04-25 21:34:41,074 DEBUG [bifrost::tcp::client] Streaming messages for order-1:34803
order-3             | 2020-04-25 21:34:41,074 DEBUG [bifrost::raft] Become follower bacause of join: 4751096279368508625
order-3             | 2020-04-25 21:34:41,074 DEBUG [bifrost::raft] Resetting last checked for join: 4751096279368508625
order-3             | 2020-04-25 21:34:41,074 DEBUG [bifrost::raft] Completed join for 4751096279368508625, result true
order-3             | 2020-04-25 21:34:41,074 INFO  [actix_server::builder] Starting 128 workers
order-3             | 2020-04-25 21:34:41,081 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34802" service on 0.0.0.0:34802
catalog-1           | 2020-04-25 21:34:41,155 INFO  [bifrost::raft] Waiting for raft server to be initialized
catalog-1           | 2020-04-25 21:34:41,155 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-1           | 2020-04-25 21:34:41,155 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-1           | 2020-04-25 21:34:41,156 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-1           | 2020-04-25 21:34:41,156 DEBUG [catalog_server::configs] Server list: ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-1           | 2020-04-25 21:34:41,156 DEBUG [catalog_server::configs] Will bootstrap
catalog-1           | 2020-04-25 21:34:41,156 DEBUG [bifrost::raft] Conservative bootstrap, checking storage
catalog-1           | 2020-04-25 21:34:41,156 DEBUG [bifrost::raft] There are storage, checking last term
catalog-1           | 2020-04-25 21:34:41,156 DEBUG [bifrost::raft] Log is empty, bootstrap
catalog-1           | 2020-04-25 21:34:41,156 DEBUG [bifrost::raft] Server 10578407250799441171 become leader, term 0
catalog-1           | 2020-04-25 21:34:41,156 INFO  [actix_server::builder] Starting 128 workers
catalog-1           | 2020-04-25 21:34:41,164 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34801" service on 0.0.0.0:34801
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]...9
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft::client] Checking server info on catalog-1:34803
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft::client] Getting server info from catalog-1:34803
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft::client] Checking server client catalog-1:34803
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-1:34803
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft::client] Checking response from catalog-1:34803
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft::client] Found server info with leader id 10578407250799441171
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft] Executing in SM to create new member catalog-3:34803, 4820394039294786766
catalog-3           | 2020-04-25 21:34:47,695 DEBUG [bifrost::raft::client] Leader id for client: 10578407250799441171
catalog-1           | 2020-04-25 21:34:47,703 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 0, appended [1]
catalog-1           | 2020-04-25 21:34:47,703 DEBUG [bifrost::raft] Sync config to followers
catalog-1           | 2020-04-25 21:34:47,703 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-1           | 2020-04-25 21:34:47,703 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-1           | 2020-04-25 21:34:47,703 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-1           | 2020-04-25 21:34:47,703 DEBUG [bifrost::raft] Log mismatch in follower 4820394039294786766, index 2
catalog-3           | 2020-04-25 21:34:47,707 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 0, appended [1]
catalog-3           | 2020-04-25 21:34:47,707 DEBUG [bifrost::raft] Getting member address: 4820394039294786766
catalog-1           | 2020-04-25 21:34:47,707 DEBUG [bifrost::raft] Heartbeat response from 4820394039294786766 is 1
catalog-1           | 2020-04-25 21:34:47,707 DEBUG [bifrost::raft] Members 1 granted 1, is majority: true
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::raft] Updating local meta by acquiring lock: 4820394039294786766
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::raft] Local meta lock acquired: 4820394039294786766
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::raft] We have following members for 4820394039294786766: ["catalog-3:34803", "catalog-1:34803"]
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::raft] Become follower bacause of join: 4820394039294786766
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::raft] Resetting last checked for join: 4820394039294786766
catalog-3           | 2020-04-25 21:34:47,708 DEBUG [bifrost::raft] Completed join for 4820394039294786766, result true
catalog-3           | 2020-04-25 21:34:47,708 INFO  [actix_server::builder] Starting 128 workers
catalog-3           | 2020-04-25 21:34:47,716 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34801" service on 0.0.0.0:34801
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]...9
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft::client] Checking server info on catalog-1:34803
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft::client] Getting server info from catalog-1:34803
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft::client] Checking server client catalog-1:34803
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-1:34803
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft::client] Checking response from catalog-1:34803
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft::client] Found server info with leader id 10578407250799441171
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft] Executing in SM to create new member catalog-2:34803, 16030156735997574805
catalog-2           | 2020-04-25 21:34:49,963 DEBUG [bifrost::raft::client] Leader id for client: 10578407250799441171
catalog-1           | 2020-04-25 21:34:49,981 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 1, appended [2]
catalog-1           | 2020-04-25 21:34:49,981 DEBUG [bifrost::raft] Sync config to followers
catalog-1           | 2020-04-25 21:34:49,981 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-1           | 2020-04-25 21:34:49,981 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-1           | 2020-04-25 21:34:49,981 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-1           | 2020-04-25 21:34:49,981 DEBUG [bifrost::raft] Log mismatch in follower 16030156735997574805, index 3
catalog-1           | 2020-04-25 21:34:49,981 DEBUG [bifrost::raft] Log mismatch in follower 16030156735997574805, index 2
catalog-3           | 2020-04-25 21:34:49,985 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 1, appended [2]
catalog-3           | 2020-04-25 21:34:49,985 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-3           | 2020-04-25 21:34:49,985 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-3           | 2020-04-25 21:34:49,985 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-1           | 2020-04-25 21:34:49,986 DEBUG [bifrost::raft] Heartbeat response from 4820394039294786766 is 2
catalog-1           | 2020-04-25 21:34:49,986 DEBUG [bifrost::raft] Members 2 granted 1, is majority: false
catalog-2           | 2020-04-25 21:34:49,988 DEBUG [bifrost::raft::disk] Appended and persisted 2 logs, was 0, appended [1, 2]
catalog-2           | 2020-04-25 21:34:49,988 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-2           | 2020-04-25 21:34:49,988 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-1           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] Heartbeat response from 16030156735997574805 is 2
catalog-1           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] Members 2 granted 2, is majority: true
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] Getting member address: 16030156735997574805
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] Updating local meta by acquiring lock: 16030156735997574805
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] Local meta lock acquired: 16030156735997574805
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] We have following members for 16030156735997574805: ["catalog-3:34803", "catalog-2:34803", "catalog-1:34803"]
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] Become follower bacause of join: 16030156735997574805
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] Resetting last checked for join: 16030156735997574805
catalog-2           | 2020-04-25 21:34:49,989 DEBUG [bifrost::raft] Completed join for 16030156735997574805, result true
catalog-2           | 2020-04-25 21:34:49,990 INFO  [actix_server::builder] Starting 128 workers
catalog-2           | 2020-04-25 21:34:49,997 INFO  [actix_server::builder] Starting "actix-web-service-0.0.0.0:34801" service on 0.0.0.0:34801
```
### Execution Outputs
```
fontend             | 2020-04-25 21:38:10,968 DEBUG [pygmy_frontend] Using remote result for 3
fontend             | 2020-04-25 21:38:10,968 DEBUG [pygmy_frontend] Checking url (0): http://catalog-1:34801/lookup/3
fontend             | 2020-04-25 21:38:10,968 DEBUG [reqwest::connect] starting new connection: http://catalog-1:34801/
fontend             | 2020-04-25 21:38:10,968 DEBUG [hyper::client::connect::dns] resolving host="catalog-1"
fontend             | 2020-04-25 21:38:10,969 DEBUG [hyper::client::connect::http] connecting to 192.168.144.7:34801
fontend             | 2020-04-25 21:38:10,969 DEBUG [hyper::client::connect::http] connected to 192.168.144.7:34801
fontend             | 2020-04-25 21:38:10,969 DEBUG [hyper::proto::h1::io] flushed 62 bytes
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Construct cached state machine for list ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"], service id 10662549175266302366, state machine 50
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Creating state machine client instance, service 10662549175266302366, state machine id 50
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Getting server info for ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]...10
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Checking server info on catalog-1:34803
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Connecting to node catalog-1:34803
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Added server info on catalog-1:34803 to members
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Member catalog-1:34803 added
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Getting server info from catalog-1:34803
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Checking server client catalog-1:34803
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-1:34803
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Checking response from catalog-1:34803
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::raft::client] Found server info with leader id 10578407250799441171
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-1           | 2020-04-25 21:38:10,970 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-1           | 2020-04-25 21:38:10,971 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-1           | 2020-04-25 21:38:10,971 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-1           | 2020-04-25 21:38:10,971 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-1           | 2020-04-25 21:38:10,971 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-1           | 2020-04-25 21:38:10,971 INFO  [catalog_server] Lookup 3
catalog-1           | 2020-04-25 21:38:10,976 INFO  [actix_web::middleware::logger] 192.168.144.8:55690 "GET /lookup/3 HTTP/1.1" 200 161 "-" "-" 0.006844
fontend             | 2020-04-25 21:38:10,976 DEBUG [hyper::proto::h1::io] read 270 bytes
fontend             | 2020-04-25 21:38:10,976 DEBUG [hyper::proto::h1::io] parsed 3 headers
fontend             | 2020-04-25 21:38:10,976 DEBUG [hyper::proto::h1::conn] incoming body is content-length (161 bytes)
fontend             | 2020-04-25 21:38:10,976 DEBUG [hyper::proto::h1::conn] incoming body completed
fontend             | 2020-04-25 21:38:10,976 DEBUG [hyper::client::pool] pooling idle connection for ("http", catalog-1:34801)
fontend             | 2020-04-25 21:38:10,977 DEBUG [reqwest::async_impl::client] response '200 OK' for http://catalog-1:34801/lookup/3
fontend             | 2020-04-25 21:38:10,977 INFO  [actix_web::middleware::logger] 192.168.144.1:38504 "GET /lookup/3 HTTP/1.1" 200 161 "-" "PostmanRuntime/7.24.1" 0.008651
fontend             | 2020-04-25 21:40:55,254 DEBUG [pygmy_frontend] Using remote result for sys
fontend             | 2020-04-25 21:40:55,254 DEBUG [pygmy_frontend] Checking url (0): http://catalog-2:34801/search/sys
fontend             | 2020-04-25 21:40:55,254 DEBUG [reqwest::connect] starting new connection: http://catalog-2:34801/
fontend             | 2020-04-25 21:40:55,255 DEBUG [hyper::client::connect::dns] resolving host="catalog-2"
fontend             | 2020-04-25 21:40:55,255 DEBUG [hyper::client::connect::http] connecting to 192.168.144.5:34801
fontend             | 2020-04-25 21:40:55,255 DEBUG [hyper::client::connect::http] connected to 192.168.144.5:34801
fontend             | 2020-04-25 21:40:55,255 DEBUG [hyper::proto::h1::io] flushed 64 bytes
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Construct cached state machine for list ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"], service id 10662549175266302366, state machine 50
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Creating state machine client instance, service 10662549175266302366, state machine id 50
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Getting server info for ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]...10
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Checking server info on catalog-1:34803
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Connecting to node catalog-1:34803
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Added server info on catalog-1:34803 to members
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Member catalog-1:34803 added
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Getting server info from catalog-1:34803
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Checking server client catalog-1:34803
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-1:34803
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Checking response from catalog-1:34803
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::raft::client] Found server info with leader id 10578407250799441171
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-2           | 2020-04-25 21:40:55,256 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-2           | 2020-04-25 21:40:55,257 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-2           | 2020-04-25 21:40:55,257 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-2           | 2020-04-25 21:40:55,257 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-2           | 2020-04-25 21:40:55,257 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-1           | 2020-04-25 21:40:55,257 INFO  [catalog_server] Searching sys
catalog-2           | 2020-04-25 21:40:55,258 INFO  [actix_web::middleware::logger] 192.168.144.8:37942 "GET /search/sys HTTP/1.1" 200 242 "-" "-" 0.002388
fontend             | 2020-04-25 21:40:55,258 DEBUG [hyper::proto::h1::io] read 351 bytes
fontend             | 2020-04-25 21:40:55,258 DEBUG [hyper::proto::h1::io] parsed 3 headers
fontend             | 2020-04-25 21:40:55,258 DEBUG [hyper::proto::h1::conn] incoming body is content-length (242 bytes)
fontend             | 2020-04-25 21:40:55,258 DEBUG [hyper::proto::h1::conn] incoming body completed
fontend             | 2020-04-25 21:40:55,258 DEBUG [hyper::client::pool] pooling idle connection for ("http", catalog-2:34801)
fontend             | 2020-04-25 21:40:55,258 DEBUG [reqwest::async_impl::client] response '200 OK' for http://catalog-2:34801/search/sys
fontend             | 2020-04-25 21:40:55,258 INFO  [actix_web::middleware::logger] 192.168.144.1:40166 "GET /search/sys HTTP/1.1" 200 242 "-" "-" 0.003890
fontend             | 2020-04-25 21:49:06,559 DEBUG [pygmy_frontend] Getting data from down stream catalog server for list all
fontend             | 2020-04-25 21:49:06,559 DEBUG [pygmy_frontend] Checking url (0): http://catalog-3:34801/lookup
fontend             | 2020-04-25 21:49:06,559 DEBUG [reqwest::connect] starting new connection: http://catalog-3:34801/
fontend             | 2020-04-25 21:49:06,559 DEBUG [hyper::client::connect::dns] resolving host="catalog-3"
fontend             | 2020-04-25 21:49:06,560 DEBUG [hyper::client::connect::http] connecting to 192.168.144.2:34801
fontend             | 2020-04-25 21:49:06,560 DEBUG [hyper::client::connect::http] connected to 192.168.144.2:34801
fontend             | 2020-04-25 21:49:06,560 DEBUG [hyper::proto::h1::io] flushed 60 bytes
catalog-3           | 2020-04-25 21:49:06,560 DEBUG [bifrost::raft::client] Construct cached state machine for list ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"], service id 10662549175266302366, state machine 50
catalog-3           | 2020-04-25 21:49:06,560 DEBUG [bifrost::raft::client] Creating state machine client instance, service 10662549175266302366, state machine id 50
catalog-3           | 2020-04-25 21:49:06,560 DEBUG [bifrost::raft::client] Getting server info for ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]
catalog-3           | 2020-04-25 21:49:06,560 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["catalog-1:34803", "catalog-2:34803", "catalog-3:34803"]...10
catalog-3           | 2020-04-25 21:49:06,560 DEBUG [bifrost::raft::client] Checking server info on catalog-1:34803
catalog-3           | 2020-04-25 21:49:06,560 DEBUG [bifrost::raft::client] Connecting to node catalog-1:34803
catalog-3           | 2020-04-25 21:49:06,560 DEBUG [bifrost::tcp::client] TCP connect to catalog-1:34803, server id 10578407250799441171, timeout 2000ms
catalog-3           | 2020-04-25 21:49:06,560 DEBUG [bifrost::tcp::client] Create socket on catalog-1:34803
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::tcp::client] Streaming messages for catalog-1:34803
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::raft::client] Added server info on catalog-1:34803 to members
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::raft::client] Member catalog-1:34803 added
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::raft::client] Getting server info from catalog-1:34803
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::raft::client] Checking server client catalog-1:34803
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::raft::client] Invoking server_cluster_info on catalog-1:34803
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::raft::client] Checking response from catalog-1:34803
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::raft::client] Found server info with leader id 10578407250799441171
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::tcp::client] TCP connect to catalog-3:34803, server id 4820394039294786766, timeout 2000ms
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::tcp::client] Create socket on catalog-3:34803
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::tcp::client] Streaming messages for catalog-3:34803
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::tcp::client] TCP connect to catalog-2:34803, server id 16030156735997574805, timeout 2000ms
catalog-3           | 2020-04-25 21:49:06,561 DEBUG [bifrost::tcp::client] Create socket on catalog-2:34803
catalog-3           | 2020-04-25 21:49:06,562 DEBUG [bifrost::tcp::client] Streaming messages for catalog-2:34803
catalog-1           | 2020-04-25 21:49:06,562 INFO  [catalog_server] List all
catalog-3           | 2020-04-25 21:49:06,562 INFO  [actix_web::middleware::logger] 192.168.144.8:33008 "GET /lookup HTTP/1.1" 200 467 "-" "-" 0.002201
fontend             | 2020-04-25 21:49:06,562 DEBUG [hyper::proto::h1::io] read 576 bytes
fontend             | 2020-04-25 21:49:06,563 DEBUG [hyper::proto::h1::io] parsed 3 headers
fontend             | 2020-04-25 21:49:06,563 DEBUG [hyper::proto::h1::conn] incoming body is content-length (467 bytes)
fontend             | 2020-04-25 21:49:06,563 DEBUG [hyper::proto::h1::conn] incoming body completed
fontend             | 2020-04-25 21:49:06,563 DEBUG [hyper::client::pool] pooling idle connection for ("http", catalog-3:34801)
fontend             | 2020-04-25 21:49:06,563 DEBUG [reqwest::async_impl::client] response '200 OK' for http://catalog-3:34801/lookup
fontend             | 2020-04-25 21:49:06,563 INFO  [actix_web::middleware::logger] 192.168.144.1:45174 "GET /lookup HTTP/1.1" 200 467 "-" "-" 0.003574
fontend             | 2020-04-25 21:49:30,690 DEBUG [pygmy_frontend] Using cached result for sys
fontend             | 2020-04-25 21:49:30,690 INFO  [actix_web::middleware::logger] 192.168.144.1:45426 "GET /search/sys HTTP/1.1" 200 242 "-" "-" 0.000098
fontend             | 2020-04-25 21:49:40,959 DEBUG [pygmy_frontend] Use cache for list all items
fontend             | 2020-04-25 21:49:40,959 INFO  [actix_web::middleware::logger] 192.168.144.1:45530 "GET /lookup HTTP/1.1" 200 467 "-" "-" 0.000085
fontend             | 2020-04-25 21:49:45,590 DEBUG [pygmy_frontend] Using remote result for 2
fontend             | 2020-04-25 21:49:45,590 DEBUG [pygmy_frontend] Checking url (0): http://catalog-1:34801/lookup/2
fontend             | 2020-04-25 21:49:45,590 DEBUG [reqwest::connect] starting new connection: http://catalog-1:34801/
fontend             | 2020-04-25 21:49:45,590 DEBUG [hyper::client::connect::dns] resolving host="catalog-1"
fontend             | 2020-04-25 21:49:45,591 DEBUG [hyper::client::connect::http] connecting to 192.168.144.7:34801
fontend             | 2020-04-25 21:49:45,591 DEBUG [hyper::client::connect::http] connected to 192.168.144.7:34801
fontend             | 2020-04-25 21:49:45,591 DEBUG [hyper::proto::h1::io] flushed 62 bytes
catalog-2           | 2020-04-25 21:49:45,591 INFO  [catalog_server] Lookup 2
catalog-1           | 2020-04-25 21:49:45,610 INFO  [actix_web::middleware::logger] 192.168.144.8:34528 "GET /lookup/2 HTTP/1.1" 200 138 "-" "-" 0.019030
fontend             | 2020-04-25 21:49:45,610 DEBUG [hyper::proto::h1::io] read 247 bytes
fontend             | 2020-04-25 21:49:45,610 DEBUG [hyper::proto::h1::io] parsed 3 headers
fontend             | 2020-04-25 21:49:45,610 DEBUG [hyper::proto::h1::conn] incoming body is content-length (138 bytes)
fontend             | 2020-04-25 21:49:45,610 DEBUG [hyper::proto::h1::conn] incoming body completed
fontend             | 2020-04-25 21:49:45,610 DEBUG [hyper::client::pool] pooling idle connection for ("http", catalog-1:34801)
fontend             | 2020-04-25 21:49:45,610 DEBUG [reqwest::async_impl::client] response '200 OK' for http://catalog-1:34801/lookup/2
fontend             | 2020-04-25 21:49:45,610 INFO  [actix_web::middleware::logger] 192.168.144.1:45574 "GET /lookup/2 HTTP/1.1" 200 138 "-" "-" 0.020309
fontend             | 2020-04-25 21:49:45,612 DEBUG [pygmy_frontend] Checking url (0): http://order-1:34802/order/2?amount=1
fontend             | 2020-04-25 21:49:45,612 DEBUG [reqwest::connect] starting new connection: http://order-1:34802/
fontend             | 2020-04-25 21:49:45,612 DEBUG [hyper::client::connect::dns] resolving host="order-1"
fontend             | 2020-04-25 21:49:45,612 DEBUG [hyper::client::connect::http] connecting to 192.168.144.3:34802
fontend             | 2020-04-25 21:49:45,613 DEBUG [hyper::client::connect::http] connected to 192.168.144.3:34802
fontend             | 2020-04-25 21:49:45,613 DEBUG [hyper::proto::h1::io] flushed 69 bytes
order-1             | 2020-04-25 21:49:45,613 INFO  [order_server] Ordering 2, amount 1. Checking stock on catalog server.
order-1             | 2020-04-25 21:49:45,613 DEBUG [order_server] GET URL - http://catalog-1:34801/lookup/2
order-1             | 2020-04-25 21:49:45,613 DEBUG [reqwest::connect] starting new connection: http://catalog-1:34801/
order-1             | 2020-04-25 21:49:45,613 DEBUG [hyper::client::connect::dns] resolving host="catalog-1"
order-1             | 2020-04-25 21:49:45,613 DEBUG [hyper::client::connect::http] connecting to 192.168.144.7:34801
order-1             | 2020-04-25 21:49:45,614 DEBUG [hyper::client::connect::http] connected to 192.168.144.7:34801
order-1             | 2020-04-25 21:49:45,614 DEBUG [hyper::proto::h1::io] flushed 62 bytes
catalog-3           | 2020-04-25 21:49:45,614 INFO  [catalog_server] Lookup 2
catalog-1           | 2020-04-25 21:49:45,619 INFO  [actix_web::middleware::logger] 192.168.144.3:46066 "GET /lookup/2 HTTP/1.1" 200 138 "-" "-" 0.005320
order-1             | 2020-04-25 21:49:45,619 DEBUG [hyper::proto::h1::io] read 247 bytes
order-1             | 2020-04-25 21:49:45,619 DEBUG [hyper::proto::h1::io] parsed 3 headers
order-1             | 2020-04-25 21:49:45,619 DEBUG [hyper::proto::h1::conn] incoming body is content-length (138 bytes)
order-1             | 2020-04-25 21:49:45,619 DEBUG [hyper::proto::h1::conn] incoming body completed
order-1             | 2020-04-25 21:49:45,619 DEBUG [hyper::client::pool] pooling idle connection for ("http", catalog-1:34801)
order-1             | 2020-04-25 21:49:45,619 DEBUG [reqwest::async_impl::client] response '200 OK' for http://catalog-1:34801/lookup/2
order-1             | 2020-04-25 21:49:45,619 INFO  [order_server] Found item Item { id: 2, name: "RPCs for Dummies", stock: 16, price: 29.9, topic: 1 }, start transaction
order-1             | 2020-04-25 21:49:45,619 DEBUG [order_server] POST URL - http://catalog-2:34801/update/2/stock/deduct/1
order-1             | 2020-04-25 21:49:45,619 DEBUG [reqwest::connect] starting new connection: http://catalog-2:34801/
order-1             | 2020-04-25 21:49:45,619 DEBUG [hyper::client::connect::dns] resolving host="catalog-2"
order-1             | 2020-04-25 21:49:45,620 DEBUG [hyper::client::connect::http] connecting to 192.168.144.5:34801
order-1             | 2020-04-25 21:49:45,620 DEBUG [hyper::client::connect::http] connected to 192.168.144.5:34801
order-1             | 2020-04-25 21:49:45,620 DEBUG [hyper::proto::h1::io] flushed 78 bytes
catalog-2           | 2020-04-25 21:49:45,620 DEBUG [catalog_server] Trying to update stock by sending command to state machine
catalog-2           | 2020-04-25 21:49:45,620 DEBUG [bifrost::raft::client] Leader id for client: 10578407250799441171
catalog-1           | 2020-04-25 21:49:45,624 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 2, appended [3]
catalog-1           | 2020-04-25 21:49:45,624 DEBUG [bifrost::raft] Sync logs to followers
catalog-3           | 2020-04-25 21:49:45,628 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 2, appended [3]
catalog-2           | 2020-04-25 21:49:45,628 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 2, appended [3]
catalog-1           | 2020-04-25 21:49:45,628 DEBUG [bifrost::raft] Heartbeat response from 16030156735997574805 is 3
catalog-1           | 2020-04-25 21:49:45,628 DEBUG [bifrost::raft] Members 2 granted 1, is majority: false
catalog-1           | 2020-04-25 21:49:45,628 DEBUG [bifrost::raft] Heartbeat response from 4820394039294786766 is 3
catalog-1           | 2020-04-25 21:49:45,628 DEBUG [bifrost::raft] Members 2 granted 2, is majority: true
catalog-1           | 2020-04-25 21:49:45,628 INFO  [catalog_server] Deduct item 2 amount 1
catalog-2           | 2020-04-25 21:49:45,635 DEBUG [reqwest::connect] starting new connection: http://fontend:34800/
catalog-2           | 2020-04-25 21:49:45,635 DEBUG [hyper::client::connect::dns] resolving host="fontend"
catalog-2           | 2020-04-25 21:49:45,635 DEBUG [hyper::client::connect::http] connecting to 192.168.144.8:34800
catalog-2           | 2020-04-25 21:49:45,635 DEBUG [hyper::client::connect::http] connected to 192.168.144.8:34800
catalog-2           | 2020-04-25 21:49:45,635 DEBUG [hyper::proto::h1::io] flushed 70 bytes
fontend             | 2020-04-25 21:49:45,635 INFO  [pygmy_frontend] Invalidate item 2
fontend             | 2020-04-25 21:49:45,636 INFO  [actix_web::middleware::logger] 192.168.144.5:35864 "POST /invalidate/item/2 HTTP/1.1" 200 4 "-" "-" 0.000081
catalog-2           | 2020-04-25 21:49:45,636 DEBUG [hyper::proto::h1::io] read 111 bytes
catalog-2           | 2020-04-25 21:49:45,636 DEBUG [hyper::proto::h1::io] parsed 3 headers
catalog-2           | 2020-04-25 21:49:45,636 DEBUG [hyper::proto::h1::conn] incoming body is content-length (4 bytes)
order-1             | 2020-04-25 21:49:45,636 DEBUG [hyper::proto::h1::io] read 111 bytes
order-1             | 2020-04-25 21:49:45,636 DEBUG [hyper::proto::h1::io] parsed 3 headers
order-1             | 2020-04-25 21:49:45,636 DEBUG [hyper::proto::h1::conn] incoming body is content-length (4 bytes)
order-1             | 2020-04-25 21:49:45,636 DEBUG [hyper::proto::h1::conn] incoming body completed
order-1             | 2020-04-25 21:49:45,636 DEBUG [hyper::client::pool] pooling idle connection for ("http", catalog-2:34801)
catalog-2           | 2020-04-25 21:49:45,636 DEBUG [hyper::proto::h1::conn] incoming body completed
order-1             | 2020-04-25 21:49:45,636 DEBUG [reqwest::async_impl::client] response '200 OK' for http://catalog-2:34801/update/2/stock/deduct/1
catalog-2           | 2020-04-25 21:49:45,636 DEBUG [hyper::client::pool] pooling idle connection for ("http", fontend:34800)
catalog-2           | 2020-04-25 21:49:45,636 DEBUG [reqwest::async_impl::client] response '200 OK' for http://fontend:34800/invalidate/item/2
order-1             | 2020-04-25 21:49:45,636 INFO  [order_server] Order transaction for 2 successful, log transaction
catalog-2           | 2020-04-25 21:49:45,636 INFO  [actix_web::middleware::logger] 192.168.144.3:55874 "POST /update/2/stock/deduct/1 HTTP/1.1" 200 4 "-" "-" 0.015623
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Construct cached state machine for list ["order-1:34803", "order-2:34803", "order-3:34803"], service id 10662549175266302366, state machine 100
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Creating state machine client instance, service 10662549175266302366, state machine id 100
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Getting server info for ["order-1:34803", "order-2:34803", "order-3:34803"]
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Trying to get cluster info, attempt from ["order-1:34803", "order-2:34803", "order-3:34803"]...10
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Checking server info on order-1:34803
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Connecting to node order-1:34803
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::tcp::client] TCP connect to order-1:34803, server id 13433766016950577570, timeout 2000ms
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::tcp::client] Create socket on order-1:34803
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::tcp::client] Streaming messages for order-1:34803
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Added server info on order-1:34803 to members
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Member order-1:34803 added
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Getting server info from order-1:34803
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Checking server client order-1:34803
order-1             | 2020-04-25 21:49:45,636 DEBUG [bifrost::raft::client] Invoking server_cluster_info on order-1:34803
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::raft::client] Checking response from order-1:34803
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::raft::client] Found server info with leader id 13433766016950577570
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::tcp::client] TCP connect to order-2:34803, server id 16848524676177999986, timeout 2000ms
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::tcp::client] Create socket on order-2:34803
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::tcp::client] Streaming messages for order-2:34803
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::tcp::client] TCP connect to order-3:34803, server id 4751096279368508625, timeout 2000ms
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::tcp::client] Create socket on order-3:34803
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::tcp::client] Streaming messages for order-3:34803
order-1             | 2020-04-25 21:49:45,637 DEBUG [bifrost::raft::client] Leader id for client: 13433766016950577570
order-1             | 2020-04-25 21:49:45,640 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 2, appended [3]
order-1             | 2020-04-25 21:49:45,640 DEBUG [bifrost::raft] Sync logs to followers
order-2             | 2020-04-25 21:49:45,644 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 2, appended [3]
order-3             | 2020-04-25 21:49:45,644 DEBUG [bifrost::raft::disk] Appended and persisted 1 logs, was 2, appended [3]
order-1             | 2020-04-25 21:49:45,644 DEBUG [bifrost::raft] Heartbeat response from 4751096279368508625 is 3
order-1             | 2020-04-25 21:49:45,644 DEBUG [bifrost::raft] Members 2 granted 1, is majority: false
order-1             | 2020-04-25 21:49:45,644 DEBUG [bifrost::raft] Heartbeat response from 16848524676177999986 is 3
order-1             | 2020-04-25 21:49:45,644 DEBUG [bifrost::raft] Members 2 granted 2, is majority: true
order-1             | 2020-04-25 21:49:45,644 INFO  [order_server] RSM replicating the order log, item 2, amount 1, total 29.9
order-1             | 2020-04-25 21:49:45,656 INFO  [actix_web::middleware::logger] 192.168.144.8:35364 "POST /order/2?amount=1 HTTP/1.1" 200 4 "-" "-" 0.043122
fontend             | 2020-04-25 21:49:45,656 DEBUG [hyper::proto::h1::io] read 111 bytes
fontend             | 2020-04-25 21:49:45,656 DEBUG [hyper::proto::h1::io] parsed 3 headers
fontend             | 2020-04-25 21:49:45,656 DEBUG [hyper::proto::h1::conn] incoming body is content-length (4 bytes)
fontend             | 2020-04-25 21:49:45,656 DEBUG [hyper::proto::h1::conn] incoming body completed
fontend             | 2020-04-25 21:49:45,656 DEBUG [hyper::client::pool] pooling idle connection for ("http", order-1:34802)
fontend             | 2020-04-25 21:49:45,656 DEBUG [reqwest::async_impl::client] response '200 OK' for http://order-1:34802/order/2?amount=1
fontend             | 2020-04-25 21:49:45,656 INFO  [actix_web::middleware::logger] 192.168.144.1:45580 "POST /order/2?amount=1 HTTP/1.1" 200 4 "-" "-" 0.044538
catalog-3           | 2020-04-25 21:49:45,659 INFO  [catalog_server] Deduct item 2 amount 1
catalog-2           | 2020-04-25 21:49:45,659 INFO  [catalog_server] Deduct item 2 amount 1
order-3             | 2020-04-25 21:49:45,674 INFO  [order_server] RSM replicating the order log, item 2, amount 1, total 29.9
order-2             | 2020-04-25 21:49:45,674 INFO  [order_server] RSM replicating the order log, item 2, amount 1, total 29.9
```