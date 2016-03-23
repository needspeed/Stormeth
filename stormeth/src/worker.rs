use json_parser::Request;

pub fn handle_request(request: &Request) {
    println!("id: {}, jsonrpc: {}, method {}, params: {}", request.id, request.jsonrpc, request.method, 
             request.params.len());

    
}


//submitLogin, getWork, submitHashrate, submitWork
