mod db;
mod worker;
mod json_parser;

extern crate rustc_serialize;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Write, BufReader, BufRead};


const LISTEN_ADDR:  &'static str = "0.0.0.0:6667";
const HOST_ADDR:    &'static str = "127.0.0.1:8545";

//-----------------------------------------------------------------------------


fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut line1 = String::new();
    reader.read_line(&mut line1);
    if line1.as_str().find("HTTP") != None { handle_non_json(&stream); return; }
    else {println!("Found JSON");}
    
    loop {
        let request = json_parser::fromJSON(&line1);
        worker::handle_request(&request);
        line1.clear();
        //TODO: check if JSON or end or something
        reader.read_line(&mut line1);
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
