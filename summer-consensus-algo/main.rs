use summer_consensus_algo::raft::node_type::Node;
use summer_consensus_algo::rpc::init;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::env;

#[tokio::main]
async fn main(){
    let args:Vec<String> = env::args().collect();
    let node = Arc::new(Mutex::new(Node::new("0xff".to_string())));

    match args[1].as_str(){
        "Client"=>{
            println!("Launching Client");
            init::client_init(node).await;
        },
        "Server"=>{
            let url = args[2].clone();
            println!("Launching Server on {}",url);
            init::server_init(node,url).await;
        },
        _=>{
            println!("Unkown args:{}",args[1]);
        },
    }
}