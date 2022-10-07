use crate::raft::node_type::{Node};
use crate::rpc::server::raft_rpc::{AppendEntriesRequest, AppendEntriesReply,RequestVoteRequest,RequestVoteReply};

impl Node{
    pub fn server_read(&mut self,message:AppendEntriesRequest)->AppendEntriesReply{
        if self.term() < message.term {
            self.downgrade();
            self.set_term(message.term);
        }
        if self.term() > message.term{
            return AppendEntriesReply{
                term:self.term(),
                success:false
            }
        }
        if message.term == message.prev_log_term && self.prev_log_index() < message.prev_log_index {
            return AppendEntriesReply{
                term:self.term(),
                success:false
            }
        }
        self.set_commit_index(message.leader_commit);
        return AppendEntriesReply{
            term:self.term(),
            success:true
        }
    }

    pub fn server_vote(&mut self,message:RequestVoteRequest) -> RequestVoteReply{
        if self.term() < message.term {
            self.downgrade();
            self.set_term(message.term);
        }
        if self.term() > message.term{
            return RequestVoteReply{
                term:self.term(),
                vote_granted:false
            }
        }else{
            if let Some(id) = self.vote_for(){
                if id == message.candidate_id{
                    if message.last_log_index >= self.prev_log_index(){
                        return RequestVoteReply{
                            term:self.term(),
                            vote_granted:true
                        }
                    }
                }
            }else{
                if message.last_log_index >= self.prev_log_index(){
                    self.set_vote(message.candidate_id);
                    return RequestVoteReply{
                        term:self.term(),
                        vote_granted:true
                    }
                }
            }
        }
        return RequestVoteReply{
            term:self.term(),
            vote_granted:false
        }
    }

    pub fn client_read(&mut self,message:AppendEntriesReply){
        if self.term() < message.term {
            self.downgrade();
            self.set_term(message.term);
        }
        if message.success{
            
        }
    }

    pub fn client_vote(&mut self,message:RequestVoteReply)->u32{
        if self.term() < message.term {
            self.downgrade();
            self.set_term(message.term);
        }
        if message.vote_granted{
            return 1;
        }else{
            return 0;
        }
    }
}