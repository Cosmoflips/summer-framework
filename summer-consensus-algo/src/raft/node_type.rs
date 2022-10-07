use tokio::time;
use std::sync::Arc;
use tokio::sync::Mutex;
//声明节点状态机
#[derive(Debug,Default,Clone)]
enum State{
    #[default] Follower,
    Candidate(u32),
    Leader(u32,u32)
}
#[derive(Debug)]
pub struct Node{
    id:String,
    state:State,
    current_term:u32,
    vote_for:Option<String>,
    entries:Vec<String>,
    commit_index:u32,
    last_applied:u32,
    next_index:Option<u32>,
    match_index:Option<u32>,
    timer:time::Instant,
}

impl Node{
    pub fn new(id:String)->Self{
        Self{
            id,
            state:State::Follower,
            current_term:1,
            vote_for:None,
            entries:vec!["0::Autofill".to_string()],
            commit_index:0,
            last_applied:0,
            next_index:None,
            match_index:None,
            timer:time::Instant::now(),
        }
    }

    pub fn upgrade(&mut self){
        match self.state{
            State::Follower=>{
                self.state=State::Candidate(0);
            },
            State::Candidate(_)=>{
                let len=self.entries.len();
                self.state=State::Leader(len as u32,len as u32 -1);
                self.vote_for = Some(self.id.clone());
            }
            State::Leader(_,_)=>{
                ()
            }
        }
    }

    pub fn downgrade(&mut self){
        match self.state{
            State::Follower=>{
                ()
            },
            State::Candidate(_)=>{
                self.state=State::Follower;
                self.vote_for = None;
            },
            State::Leader(_,_)=>{
                self.state=State::Follower;
                self.vote_for = None;
            }
        }
    }

    pub fn id(&self)->String{
        self.id.clone()
    }

    pub fn set_id(&mut self,id:String){
        self.id=id;
    }

    pub fn term(&self)->u32{
        self.current_term
    }

    pub fn set_term(&mut self,term:u32){
        self.current_term=term;
    }

    pub fn vote_for(&self) -> Option<String>{
        self.vote_for.clone()
    }

    pub fn set_vote(&mut self,leader:String){
        self.vote_for = Some(leader);
    }

    pub fn next_index(&self)->u32{
        self.next_index.unwrap_or(0)
    }

    pub fn prev_log_index(&self)->u32{
        self.entries.len() as u32 - 1
    }

    pub fn prev_log_term(&self)->u32{
        self.entries.last().unwrap().split("::").next().unwrap_or("0").parse::<u32>().unwrap()
    }

    pub fn commit_index(&self)->u32{
        self.commit_index
    }

    pub fn set_commit_index(&mut self,index:u32){
        if index > self.commit_index{
            self.commit_index = index.min(self.prev_log_index());
        }
    }

    pub fn last_applied(&self)->u32{
        self.last_applied
    }

    fn status(&self)->State{
        self.state.clone()
    }

    pub fn reset_status(&mut self){
        self.state = State::Follower;
    }

    fn timer(&self)->time::Duration{
        self.timer.elapsed()
    }

    fn timer_reset(&mut self){
        self.timer = time::Instant::now();
    }

    pub fn new_log(&mut self,log:Vec<String>,prev_log_index:u32){
        let mut p = 0;
        let prev_log_index = prev_log_index as usize + 1;
        while p + prev_log_index < self.entries.len(){
            if self.entries[p + prev_log_index] == log[p]{
                p += 1;
                continue;
            }else{
                self.entries.drain(p + prev_log_index..);
            }
        }
        if p < log.len(){
            self.entries.append(&mut log[p..].to_vec());
        }
    }

    pub async fn auto_check(node:Arc<Mutex<Node>>){
        let mut interval = time::interval(time::Duration::from_millis(500));
        loop{
            interval.tick().await;
            let mut node = node.lock().await;
            let status = node.status();
            println!("status check:{:?}",status);
            if node.timer() > time::Duration::from_millis(800){
                match status{
                    State::Follower=>node.upgrade(),
                    _=>()
                };
                node.timer_reset();
            }
        }
    }
}
