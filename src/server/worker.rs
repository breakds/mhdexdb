use std::io;
use std::io::Write;
use std::fmt;

use mio::{EventLoop, EventSet, PollOpt, Token, TryRead, TryWrite};
use mio::tcp::{TcpStream, Shutdown};

use http_muncher::{Parser, ParserHandler};

use server::{create_json_response, TestPayload, DexDataServer};

struct HttpParser;
impl ParserHandler for HttpParser {
    fn on_body(&mut self, body: &[u8]) -> bool {
        // Here we can get the payload of the http request.
        println!("Received Body: {:?}", body);
        true
    }
}

enum DexDataWorkerState {
    Running,
    Closed,
}

pub struct DexDataWorker {
    pub token: Token,
    pub stream: TcpStream,
    state: DexDataWorkerState,
    // http_parser keeps the state.
    http_parser: Parser<HttpParser>,
}

impl DexDataWorker {
    pub fn new(token: Token, stream: TcpStream) -> DexDataWorker {
        DexDataWorker {
            token: token,
            stream: stream,
            http_parser: Parser::request(HttpParser),
            state: DexDataWorkerState::Running,
        }
    }
    
    pub fn handle_readable(&mut self, event_loop: &mut EventLoop<DexDataServer>) {
        let mut buffer = [0; 2048];
        loop {
            match self.stream.try_read(&mut buffer) {
                Err(e) => {
                    println!("Error while reading socket stream: {:?}", e);
                },
                Ok(None) => break, // no more byte from the socket
                Ok(Some(0)) => {
                    self.state = DexDataWorkerState::Closed;
                    break;
                },
                Ok(Some(len)) => {
                    self.http_parser.parse(&buffer);
                    println!("HTTP Version: {:?}", self.http_parser.http_version());
                    let text = String::from_utf8_lossy(&buffer[0..len]);
                    println!("Read length {} : {}", len, text);
                }
                
            }
        }

        println!("Handled client with token {:?}.", self.token);

        match self.state {
            DexDataWorkerState::Closed => (),
            _ => event_loop.reregister(&self.stream, self.token, EventSet::writable(),
                                       PollOpt::edge() | PollOpt::oneshot()).unwrap(),
        }
    }
    
    pub fn handle_writable(&mut self, event_loop: &mut EventLoop<DexDataServer>) {
        println!("Prepare to write with token {:?}", self.token);

        let tmp = TestPayload {
            a: 31,
            b: 37,
        };

        let response = create_json_response(&tmp);

        self.stream.try_write(response.as_bytes()).unwrap();
        
        event_loop.reregister(&self.stream, self.token, EventSet::readable(),
                              PollOpt::edge() | PollOpt::oneshot()).unwrap();
    }
    
    pub fn is_closed(&self) -> bool {
        match self.state {
            DexDataWorkerState::Closed => true,
            _ => false,
        }
    }
}




