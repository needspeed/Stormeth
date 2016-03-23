use rustc_serialize::json::{self,Json,ToJson};
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
use std::collections::BTreeMap;

#[derive(RustcDecodable)]
pub struct Request {
    pub worker: String,
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>
}

#[derive(RustcEncodable)]
pub struct Reply {
    pub id: u64,
    pub jsonrpc: String,
    pub result: Json_Result
}

impl Encodable for Json_Result {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        match self {
            &Json_Result::Vector(ref a) => return a.encode(s),
            &Json_Result::Bool(ref a) => return a.encode(s)
        }
    }
}


pub enum Json_Result {
    Vector(Vec<String>),
    Bool(bool)
}

pub fn request_from_JSON(json_str: &String) -> Request {
    println!("Decoding Request: {}|||", json_str);
    return json::decode(json_str).unwrap();
}

pub fn replies_from_JSON(json_strs: &str) -> Vec<Reply> {
    return json_strs.replace("}{", "}___{").split_terminator("___")
        .map(reply_from_JSON).collect();
}

pub fn reply_from_JSON(json_str: &str) -> Reply {
    println!("Decoding Reply: {}", json_str);
    
    let data = Json::from_str(json_str).unwrap();
    let obj = data.as_object().unwrap();

    let id = obj.get("id").unwrap().as_u64().unwrap();
    let jsonrpc = obj.get("jsonrpc").unwrap().as_string().unwrap().to_string();
    let mut result;
    
    if obj.get("result").unwrap().is_boolean() {
        result = Json_Result::Bool(obj.get("result").unwrap().as_boolean().unwrap());
    }
    else {
        let json_vec = obj.get("result").unwrap().as_array().unwrap();
        let mut vec_str: Vec<String> = Vec::new(); 
        
        for json_elem in json_vec {
            vec_str.push(json_elem.as_string().unwrap().to_string());
        }

        result = Json_Result::Vector(vec_str);
    }
    return Reply { id: id, jsonrpc: jsonrpc, result: result };;
}

pub fn reply_to_string(reply: &Reply) -> String {
    return json::encode(reply).unwrap();
}
