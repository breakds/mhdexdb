use std::fmt;
use rustc_serialize::{Encodable, json};

#[derive(RustcEncodable)]
pub struct TestPayload {
    pub a: i32,
    pub b: i32
}

pub fn create_json_response<DataType: Encodable>(data: &DataType) -> String {
    let payload: String = json::encode(data).unwrap();
    fmt::format(format_args!("HTTP/1.1 200 OK\r\n\
                              Access-Control-Allow-Origin: *\r\n\
                              Content-Type: application/json\r\n\
                              Content-Length: {}\r\n\
                              \r\n\
                              {}\r\n\r\n",
                             payload.len(),
                             payload))
}
                                    
