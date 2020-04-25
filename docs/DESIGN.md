# Design
## Componnents
This project have 3 server components, one client with CLI and tests.
1. Catalog server - It is in `src/pygmy-backend`
2. Order server - It is in `src/pygmy-backend`
3. Frontend server - It is in `src/pygmy-frontend`
4. Client and tests - It is in `tests/pygmy-client`

Server address and ports, as well as endpoints for their services, can be configured via `.env` file.

Replication, recovery are built on top of one of the early work from Hao Shi (https://github.com/ShisoftResearch/bifrost/tree/cs677-lab-3), the **Biforst**. It have been rapidly improved to accomdate asynchronized Rust programming pattern and fit the needs from this assignment, shipped with Raft replicated state machine framework for consensus.

## Raft Framework

This assignment demands replication and fault tolerance. Raft consensus can achieve first one and partialy second one. It provides following features.

1. Replicate state change logs on majority of followers before return to the client
2. Follower and leaders can fail
3. When follower fails, the system won't be affected until the cluster loose majority of its membrers
4. When leader fails, it will trigger heartbeat time out at follower side, then follower will broadcast to request votes to become a leader
5. If a node is back online, leader will transfer all logs that is missing to the node for recovery and back to the same state as other member in the cluster

Raft can offers a replicated state machine (RSM) for this project, we want to make sure that the algorithm can be offered to both order server (for order logs) and catalog server (for stock number). Thus, it will be perferred if we can have a framework can provide consensus without concern of the the logic behind. 

Bifrost offered a DSL like most of the RPC frameworks, to translate function calls to logs. For example, in our order server, looging the order can define as following

```
raft_state_machine! {
    // Define log order as a command, with 3 parameters
    def cmd log_order(item: i32, amount: i32, total: f32);
}
``` 
Each of the function call `log_order` will produce a raft log, been replicated across replicas and then been executed. The code to be executed in the state machine, can be defined as following

```
// Define the implementation of the RSM, which is to write the log to database
impl StateMachineCmds for ReplicatedOrderLog {
    fn log_order(&mut self, item_id: i32, num_amount: i32, total_sum: f32) -> BoxFuture<()> {
        info!(
            "RSM replicating the order log, item {}, amount {}, total {}",
            item_id, num_amount, total_sum
        );
        use schema::order::dsl::*;
        let conn = establish_connection();
        diesel::insert_into(order).values(&NewOrder {item: item_id, amount: num_amount, total: total_sum}).execute(&conn).unwrap();
        future::ready(()).boxed()
    }
}
```
Most of them are business logic. It can be writing to a local file, local databases, but not a shared resource among other members in the cluster.  

If we have data replicated, it is vital to ensure sequential consistency in any case when the underlying data have been changed. To ensure this, clients for the cluster should responsible for maintaining the total order for reading data from the server. Say you have 3 nodes in the cluster. The client may read from different node than last time due to load-balancing or covering bad nodes. It is possible for the cluster to read stale data from a follower that have not been updated yet. This can leads to let the user or other part of the system to see a older data instead of new or unchanged one. In **bifrost**, read operations are also been handled by the state machine, but with no logs to be produced. For example, we define catalog server RSM as follows

```
raft_state_machine! {
    // Define search query
    def qry search(topic: String) -> LookupRes<Vec<Item>>;
    // Define lookup query
    def qry lookup(item_id: i32) -> LookupRes<Item>;
    // Define list a;; query
    def qry list_all() -> LookupRes<Vec<Item>>;
    // Define update command
    def cmd update_stock_deduct(item_id: i32, stock_deduct: i32) -> bool;
}
```
Prefix `qry` are differnt from `cmd` because they will be executed under the context of the state machine but no logs will be produced. It can be very fast by using round-robin to rotate nodes for each of the the read operations. To easure never read data backwards, in each of the read operation the serve will return the `committed log index`. The client will compare the log index with the one it seen before, to see if the result can be older or not. If the log index is lower than before, it will issue another query to other followers.

Implementation of the Raft consensus algorithm can be tedious and consume dozen of pages to describe. In short, each node have a timer running in the background, checking if the the leader should send heartbeat to its followers, or followers can see if one leader have been 'missing' for a while and it may be dead to become a candidate and initiate a round of `vote request` to try to become a new leader. Each nodes will only cast vote to other only when it is not candidate and it have not give to others in this term. It will also check if the candidate have been up to dated by comparing last log id. Once a candidate received vote from majority of nodes in the cluster, it will become the leader and then start to send heartbeat to its followers. Commands with new logs will be sent to the leader and replicated to followers with the heartbeat, which means only leader can make progress. But followers can always redirect the client to the leader if the client send commands to the follower. Each of the logs will be presisted on disk if the have been processed, after a crash, RSM can be restored from the presisted logs.

## Frontend server

Frontend server is the entry point to the service. It can be used to load-balance user requests to backend servers, and also cover failed backends by trying next server when an error have been discovered. It have cache for relatively static data to boost read operations and caches can be invalidated when the cached data have been changed.

Load-balance are achieved in round-robin fashion. Each node will have uniform loads under heavy access. For each of the requests to the backend server, it will check if it have been succeed. If the request to backend return a failure, it will try the next one. If the frontend server tried every node in the server list and still failed, it will give up.

This approach can take a lot of time if there are too many nodes are unavailable. The best way is to actively monitor the health of backend nodes and choose among the healthy. 

Frontend server also provide a RESTful interface to enable invalidate the cache. Right now, frontend caches item information, search results and the list of all items. Catalog server can choose to invalidate specific item, the frontend server will trash the cache for the item, together with all search result and all items cache. It can be time consuming to scan over all searchs to determinate which one to invalidate, we just clear them, A better way can be building an reverse index for the search cache to remember which search result contains which items and remove the search results contains the item only.

## Backend servers

Frontend server will pick one backend server for each user request at its well, so did the order server when it want to update item stock. All backend servers in the same type (catalog and order) are no difference to the upstream servers. In this circumstances, the server called by upstream servers, become the coordinator to this operation. 

For example, when a user made an order by sending a request to the frontend server, the fontend server is free to pick any of the available order servers in the list. But actually, this operation should be handled by a cluster to replicate the order logs, and the updating on catalog server task should be done only once, not on multiple nodes in the cluster. The coordinator selected by upstream servers will be responsible for sending command to the Raft state machine and talk to downstream servers.

In backend, downstream servers are also be called in round-robin and fail-over way. If upstream coordinator find the server it call to is not available or return error for some reason, it will try another one until every downserver is not available.

Each of the backend server are one of the member in the Raft state machine. Upon start, backend servers will initialize the Raft state machine server and the HTTP server. Bacause all of the backend data are gone through the Raft state machine, and data will persistent to SQLite data bases, we can count on the these two infrstructure for recovery. Because the data will either in the database or in the state machine for execution. Pair with round-robin and fail-over, backend servers are also fault-tolerant, because upstreams can always find a machine to sent their requests even though part of the nodes in the cluster is not available. 

To ensure cache consistency, catalog coordinator will send invalidate messages to frontend server, tp ensure end users can see most recent results. 


## HTTP server and business logics
This project is built on Rust, with `actix` framework for servers and `reqwest` for clients. For data persistency, it use `SQLite` with `diesel` as ORM. The reason to choose those tools is because they are fast and easy to development in the same time. Data are modeled and items are imported first with diesel migration scripts. 'diesel' will generate database schemas for us to build models upon it.In most cases, we don't need to write SQL except some statements cannot been generated by `diesel`. To manage route and for HTTP server, we use `actix` server setup. Server logs are also setup there to see how is the server working and incomming requests.

All applications are build in asynchronization pattern. Which is the server functions are mostly state machine composer, especially for remote microservice server requests. Transacrion is used on updating stock because we need to check if there is enough on stock.

For simplicity this server have weak concurrency control. Which is it totally relying on the embedded SQLite database engine itself for consistency. `diesel` itself is not asynchronized at all. For every database operations, it will block the state machine. This can be improved by using a thread pool or using asynchronized version of ORM libraries.

#### Frontend improvement
It is easy to say that the frontend server is merly an proxy to catalog and order server. In this case, frontend server does not need to have any knowledge on data format for each of the backend servers. In our implementation, responses JSON from backend servers are bypassed without deserialization and serialization. This can enable better performance and simplier to maintain.