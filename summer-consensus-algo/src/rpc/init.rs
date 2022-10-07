use std::sync::Arc;

use crate::rpc::{server,client};
use crate::raft::node_type::Node;

use tokio::sync::Mutex;

pub async fn server_init(node:Arc<Mutex<Node>>,url:String){
    let node1 = node.clone();
    let url1 = url.clone();
    let mut handles = Vec::new();

    handles.push(
        tokio::spawn(async move{
            println!("[startup]Server Launched");
            let request_vote_handle = server::server_run(node1,url1).await;
            println!("{:?}",request_vote_handle);
            }
        )
    );
    /*handles.push(
        tokio::spawn(async move{
            println!("[startup]Timer Launched");
            Node::auto_check(node.clone()).await;
            }
        )
    );*/

    for handle in handles{
        handle.await;
    }
    
    
    
}

pub async fn client_init(node:Arc<Mutex<Node>>){
    let node1 = node.clone();
    let node2 = node.clone();
    let mut handles = Vec::new();

    /*handles.push(
        tokio::spawn(async move{
            println!("[startup]RequestVoteClient Launched");
            let request_vote_handle = client::request_vote_request(node2).await;
            println!("{:?}",request_vote_handle);
            }
        )
    );
    handles.push(
        tokio::spawn(async move{
            println!("[startup]AppendEntriesClient Launched");
            let append_entries_handle = client::append_entries_request(node1,Vec::new()).await;
            println!("{:?}",append_entries_handle);
            }
        )
    );*/
    handles.push(
        tokio::spawn(async move{
            println!("mock launched");
            let mock_handle = client::mock(node1,Vec::new()).await;
            println!("{:?}",mock_handle);
        })
    );

    for handle in handles{
        handle.await;
    }
}