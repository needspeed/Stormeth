mod db;
mod worker;
mod json_parser;

extern crate rustc_serialize;
extern crate hyper; //Temp

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Write, BufReader, BufWriter, BufRead, Read};

use hyper::Client;  //Temp
use hyper::header::Connection; //Temp


const LISTEN_ADDR:  &'static str = "0.0.0.0:6667";
const HOST_ADDR:    &'static str = "127.0.0.1:8545";

//-----------------------------------------------------------------------------


fn handle_client(mut stream: TcpStream) {
    let mut client = Client::new();//Temp

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut line1 = String::new();
    reader.read_line(&mut line1);
    if line1.as_str().find("HTTP") != None { handle_non_json(&stream); return; }
    else {println!("Found JSON");}
    let mut wallet = String::new();
    let mut worker = "stratum".to_string();
    let mut body = String::new();
    
    loop {
        let request = json_parser::request_from_JSON(&line1);
        //worker::handle_request(&request);

        // HACK BEGINS -------
        worker = "stratum".to_string();
        let mut skip = false;
        match request.method.as_str() { 
            "eth_submitLogin" => {wallet = request.params[0].clone(); skip = true;},
            "eth_getWork" => {},
            "eth_submitHashrate" => worker = request.worker, 
            "eth_submitWork" => worker = request.worker,
            _ => {}
        }

        if skip {
            let mut buffer = "{\"id\":2,\"jsonrpc\":\"2.0\",\"result\":true}\n";
            writer.write_all(buffer.as_bytes()).unwrap();
            writer.flush();
        }

        if !wallet.is_empty() && !skip {
            let mut res = client.post(format!("http://127.0.0.1:5082/miner/{}.{}/10",wallet,worker)
                                      .as_str()).body(line1.as_str()).send().unwrap();

            res.read_to_string(&mut body).unwrap();
            let mut replies = json_parser::replies_from_JSON(&body);
            let mut first = true; 
            for mut reply in replies {
                if first {reply.id = request.id; first = false;}
                else {reply.id = 0;};
                
                let reply_str = json_parser::reply_to_string(&reply);
                println!("Send Back: {}", reply_str);
                writer.write_all(reply_str.as_bytes());
                //writer.write_all(body.as_bytes()).unwrap();
                writer.write_all("\n".as_bytes()).unwrap();
                writer.flush();
            }
        }
        // HACK ENDS ---------

        body.clear();
        line1.clear();
        //TODO: check if JSON or end or something
        reader.read_line(&mut line1);
        if line1.is_empty() { return; }
        println!("---------------------------------------");
    }
}

fn handle_non_json(stream: &TcpStream) {
    println!("Detected HTTP Stream... Unsupported");    
}

fn main() {
    let listener = TcpListener::bind(LISTEN_ADDR).unwrap();
    
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);
}
