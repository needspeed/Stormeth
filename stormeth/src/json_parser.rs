use rustc_serialize::json::{self,Json};

#[derive(RustcDecodable)]
pub struct Request {
    pub id: u32,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>
}

pub fn fromJSON(json_str: &String) -> Request {
    println!("Decoding: {}", json_str);
    return json::decode(json_str).unwrap();
}
