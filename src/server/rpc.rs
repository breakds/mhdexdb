use std::io;
use std::io::Read;
use mio::tcp::TcpStream;
use http_muncher::{Parser, ParserHandler};

pub struct RPCRequest {
    pub name: String,
}

impl RPCRequest {
    pub fn new() -> RPCRequest {
        RPCRequest {
            name: "".to_string()
        }
    }
    
    pub fn consume(stream: &mut TcpStream) -> io::Result<RPCRequest> {

        let mut request = RPCRequest {
            name: "".to_string()
        };

        let mut close_connection: bool = false;

        {
            // TODO(breakds): Use the deicated buffer in Rust.
            let mut buffer = [0; 2048];

            let mut parser = Parser::request(RPCHandler {
                request: &mut request
            });

            loop {
                match stream.read(&mut buffer) {
                    Err(e) => {
                        // DEBUG
                        println!("Read: {}", String::from_utf8_lossy(&buffer));

                        println!("Is_final_chunck: {:?}", parser.is_final_chunk());
                        println!("Error: {:?}", e.kind());
                        return Err(e)
                    },
                    Ok(0) => {
                        close_connection = true;
                        break;
                    },
                    Ok(len) => {
                        // DEBUG
                        println!("Read {} bytes.", len);
                        parser.parse(&buffer);
                        ()
                    }
                }
            }
        }  // Scope ends and releases request.

        if close_connection {
            Ok(RPCRequest {
                name: "CloseConnection".to_string()
            })
        } else {
            Ok(request)
        }        
    }
}

/// A shell struct that hosts several mutable references. It
/// implements ParserHandler to fill those mutable references while
/// parsing the HTTP request when the information becomes available.
///
/// 1. The url in the request is parsed to fetch the target RPC name,
///    as well as the arguments.
pub struct RPCHandler<'a> {
    pub request: &'a mut RPCRequest
}

impl<'a> ParserHandler for RPCHandler<'a> {
    fn on_url(&mut self, url_bytes: &[u8]) -> bool {
        self.request.name.push_str(&String::from_utf8_lossy(url_bytes));
        true
    }
}

