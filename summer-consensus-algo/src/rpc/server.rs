use std::sync::Arc;

use tonic::{transport::Server, Request, Response, Status};

use crate::raft::node_type::Node;
use raft_rpc::append_entries_server::{AppendEntries, AppendEntriesServer};
use raft_rpc::request_vote_server::{RequestVote,RequestVoteServer};
use raft_rpc::log_replicate_server::{LogReplicate, LogReplicateServer};
use raft_rpc::{AppendEntriesRequest, AppendEntriesReply,RequestVoteRequest,RequestVoteReply,LogReplicateRequest,LogReplicateReply};

use tokio::sync::Mutex;

//从.proto文件解析协议
pub mod raft_rpc {
    tonic::include_proto!("rpcservice");
}

//处理RPC信息和绑定所属节点
#[derive(Clone,Debug)]
pub struct ServerBuffer {
    node:Arc<Mutex<Node>>,
}

impl ServerBuffer{
    fn bind(node:Arc<Mutex<Node>>)->ServerBuffer{
        Self{
            node,
        }
    }
}

//定义AppendEntries服务中的rpc操作
#[tonic::async_trait]
impl AppendEntries for ServerBuffer {
    async fn request(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesReply>, Status> {
        let message = request.into_inner();
        println!("{:?}",message);
        let reply = self.node.lock().await.server_read(message);

        Ok(Response::new(reply))
    }
}

//定义RequestVote服务中的rpc操作
#[tonic::async_trait]
impl RequestVote for ServerBuffer {
    async fn vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteReply>, Status> {
        let message = request.into_inner();
        println!("{:?}",message);
        let reply = self.node.lock().await.server_vote(message);

        Ok(Response::new(reply))
    }
}

//启动服务
pub async fn server_run(node:Arc<Mutex<Node>>,url:String) -> Result<(), Box<dyn std::error::Error>> {
    //从基本库中解析监听的IP地址
    let addr = format!("{}:50051",url).as_str().parse()?;
    let buffer = ServerBuffer::bind(node);

    //启动服务
    Server::builder()
        .add_service(AppendEntriesServer::new(buffer.clone()))
        .add_service(RequestVoteServer::new(buffer.clone()))
        .serve(addr)
        .await?;

    Ok(())
}