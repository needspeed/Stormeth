use json_parser::Request;

pub struct client {
    miner: Vec<miner>
}

pub struct miner {
    id: String,
    hashrate: u32
}

pub fn handle_request(request: &Request) {
    println!("id: {}, jsonrpc: {}, method {}, params: {}", request.id, request.jsonrpc, request.method, 
             request.params.len());
}


//submitLogin, getWork, submitHashrate, submitWork
