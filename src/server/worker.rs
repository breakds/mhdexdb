use std::io;

use mio::{EventLoop, EventSet, PollOpt, Token, TryRead, TryWrite};
use mio::tcp::TcpStream;

use http_muncher::{Parser, ParserHandler};

struct HttpParser;
impl ParserHandler for HttpParser {}

use server::DexDataServer;

pub struct DexDataWorker {
    pub token: Token,
    pub stream: TcpStream,
    // http_parser keeps the state.
    http_parser: Parser<HttpParser>,
}

impl DexDataWorker {
    pub fn new(token: Token, stream: TcpStream) -> DexDataWorker {
        DexDataWorker {
            token: token,
            stream: stream,
            http_parser: Parser::request(HttpParser),
        }
    }
    
    pub fn handle_hup(&mut self, event_loop: &mut EventLoop<DexDataServer>) {
        println!("Get Signal Hup!");
    }
    
    pub fn handle_readable(&mut self, event_loop: &mut EventLoop<DexDataServer>) {
        let mut buffer = [0; 2048];
        loop {
            match self.stream.try_read(&mut buffer) {
                Err(e) => {
                    println!("Error while reading socket stream: {:?}", e);
                },
                Ok(None) => break, // no more byte from the socket
                Ok(Some(len)) => {
                    self.http_parser.parse(&buffer);
                    println!("HTTP Version: {:?}", self.http_parser.http_version());
                    let text = String::from_utf8_lossy(&buffer[0..len]);
                    println!("Read: {}", text);
                }
                
            }
        }

        event_loop.reregister(&self.stream, self.token, EventSet::writable(),
                              PollOpt::edge() | PollOpt::oneshot()).unwrap();

        println!("Handled client with token {:?}.", self.token);
    }
    
    pub fn handle_writable(&mut self, event_loop: &mut EventLoop<DexDataServer>) {
        println!("Prepare to write with token {:?}", self.token);

        self.stream.try_write("abcdfeasdfasdfasdf".as_bytes()).unwrap();

        event_loop.reregister(&self.stream, self.token, EventSet::readable(),
                              PollOpt::edge() | PollOpt::oneshot()).unwrap();
    }
}




