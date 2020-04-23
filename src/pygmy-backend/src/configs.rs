use std::env;
use bifrost::rpc::Server;
use bifrost::raft::*;
use bifrost::raft::state_machine::master::SubStateMachine;

// Preload all configurations from environment variable
lazy_static! {
    pub static ref CAT_SERVER_PORT: String =
        env::var("CAT_SERVER_PORT").unwrap_or("34801".to_string());
    pub static ref ORDER_SERVER_PORT: String =
        env::var("ORDER_SERVER_PORT").unwrap_or("34802".to_string());
    pub static ref RAFT_SERVER_PORT: String =
        env::var("RAFT_SERVER_PORT").unwrap_or("34803".to_string());
    pub static ref SERVER_ADDR: String = env::var("SERVER_ADDR").unwrap_or("127.0.0.1".to_string());
    pub static ref FRONTEND_SERVER_ADDR: String = env::var("FRONTEND_SERVER_ADDR")
        .unwrap_or("127.0.0.1:34800".to_string());
    pub static ref CATALOG_SERVER_LIST: Vec<String> = env::var("CATALOG_SERVER_LIST")
        .unwrap_or("127.0.0.1".to_string())
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    pub static ref ORDER_SERVER_LIST: Vec<String> = env::var("ORDER_SERVER_LIST")
        .unwrap_or("127.0.0.1".to_string())
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    pub static ref BOOTSTRAP_RAFT: bool =
        env::var("BOOTSTRAP_RAFT").unwrap_or("".to_string()).to_lowercase() == "true";
    pub static ref CATALOG_RAFT_SERVER_LIST: Vec<String> =
        CATALOG_SERVER_LIST.iter().map(|addr| {
            format!("{}:{}", addr, *RAFT_SERVER_PORT)
        }).collect();
    pub static ref ORDER_RAFT_SERVER_LIST: Vec<String> =
        ORDER_SERVER_LIST.iter().map(|addr| {
            format!("{}:{}", addr, *RAFT_SERVER_PORT)
        }).collect();
    pub static ref CATALOG_HTTP_SERVER_LIST: Vec<String> =
        CATALOG_SERVER_LIST.iter().map(|addr| {
            format!("http://{}:{}", addr, *CAT_SERVER_PORT)
        }).collect();
}

pub async fn start_raft_state_machine(state_machine: SubStateMachine, server_list: &Vec<String>) {
    let raft_addr = format!("{}:{}", *SERVER_ADDR, *RAFT_SERVER_PORT);
    let raft_service = RaftService::new(Options {
        storage: Storage::default(),
        address: raft_addr.clone(),
        service_id: DEFAULT_SERVICE_ID,
    });
    // Initialize the RPC server for Raft
    let server = Server::new(&raft_addr);
    // Register the Raft service to the RPC server
    server
        .register_service(DEFAULT_SERVICE_ID, &raft_service)
        .await;
    // Start the RPC server
    Server::listen_and_resume(&server).await;
    // Start the raft service
    RaftService::start(&raft_service).await;
    // Register the state machine to the raft service
    raft_service.register_state_machine(state_machine).await;
    // Probe and join the cluster. If no live node, it will bootstrap
    debug!("Server list: {:?}", server_list);
    if *BOOTSTRAP_RAFT {
        debug!("Will bootstrap");
        raft_service.bootstrap().await;
    } else {
        debug!("Will join server list");
        raft_service.join(server_list).await.unwrap();
    }
}