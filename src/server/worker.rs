use std::io;
use std::io::Write;
use std::fmt;

use mio::{EventLoop, EventSet, PollOpt, Token, TryRead, TryWrite};
use mio::tcp::{TcpStream, Shutdown};

use http_muncher::{Parser, ParserHandler};

use server::{create_json_response, TestPayload, DexDataServer};
use server::rpc::RPCRequest;

enum DexDataWorkerState {
    Running,
    Closed,
}

pub struct DexDataWorker {
    pub token: Token,
    pub stream: TcpStream,
    state: DexDataWorkerState,
}

impl DexDataWorker {
    pub fn new(token: Token, stream: TcpStream) -> DexDataWorker {
        DexDataWorker {
            token: token,
            stream: stream,
            state: DexDataWorkerState::Running,
        }
    }
    
    pub fn handle_readable(&mut self, event_loop: &mut EventLoop<DexDataServer>) {
        
        let request = RPCRequest::from_socket(&mut self.stream).unwrap();

        if request.name == "CloseConnection" {
            self.state = DexDataWorkerState::Closed;
        }
        
        println!("RPC name: {}", request.name);

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




