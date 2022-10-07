use raft_rpc::append_entries_client::AppendEntriesClient;
use raft_rpc::request_vote_client::RequestVoteClient;
use raft_rpc::{AppendEntriesRequest,RequestVoteRequest};
use tonic::codegen::http::request;
use crate::raft::node_type::Node;
use std::sync::Arc;
use tokio::sync::Mutex;


pub mod raft_rpc {
    tonic::include_proto!("rpcservice");
}


pub async fn append_entries(node:Arc<Mutex<Node>>,pending:Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AppendEntriesClient::connect("http://127.0.0.1:50051").await?;
    let node = node.lock().await;

    let request = tonic::Request::new(AppendEntriesRequest {
        term : node.term(),
        leader_id : node.id(),
        prev_log_index : node.prev_log_index(),
        prev_log_term : node.prev_log_term(),
        entries : pending,
        leader_commit : node.commit_index()
    });
    println!("{:?}",request);
    
    let response = client.request(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}

pub async fn request_vote(node:Arc<Mutex<Node>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RequestVoteClient::connect("http://127.0.0.1:50051").await?;
    let node = node.lock().await;

    let request = tonic::Request::new(RequestVoteRequest {
        term : node.term().into(),
        candidate_id : node.id().into(),
        last_log_index : node.prev_log_index().into(),
        last_log_term : node.prev_log_term().into(),
    });
    println!("{:?}",request);

    let response = client.vote(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}

pub async fn append_entries_request(request:AppendEntriesRequest,target:String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AppendEntriesClient::connect(format!("http://{}:50051",target)).await?;
    let request = tonic::Request::new(request);
    println!("{:?}",request);
    
    let response = client.request(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}

pub async fn request_vote_request(request:RequestVoteRequest,target:String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RequestVoteClient::connect(format!("http://{}:50051",target)).await?;
    let request = tonic::Request::new(request);
    println!("{:?}",request);

    let response = client.vote(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}

pub async fn mock(node:Arc<Mutex<Node>>,pending:Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AppendEntriesClient::connect("http://127.0.0.1:50051").await?;
    let node = node.lock().await;

    let request = tonic::Request::new(AppendEntriesRequest {
        term : 10,
        leader_id : "0xab".to_string(),
        prev_log_index : 20,
        prev_log_term : 20,
        entries : pending,
        leader_commit : 20
    });
    println!("{:?}",request);
    
    let response = client.request(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}