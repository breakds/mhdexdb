use std::io;
use std::io::Write;
use std::fmt;

use mio::{Poll, PollOpt, Token, Ready};
use mio::tcp::{TcpStream, Shutdown};

use http_muncher::{Parser, ParserHandler};

use server::{create_json_response, TestPayload, DexDataServer};
use server::rpc::RPCRequest;

enum DexDataWorkerState {
    Idle,
    Closed,
    Pending,  // Waiting to continue a read/write
}

pub struct DexDataWorker {
    pub token: Token,
    pub stream: TcpStream,
    state: DexDataWorkerState,
    // request: Box<RPCRequest>,
}

impl DexDataWorker {
    pub fn new(token: Token, stream: TcpStream) -> DexDataWorker {
        DexDataWorker {
            token: token,
            stream: stream,
            state: DexDataWorkerState::Idle,
        }
    }

    pub fn handle_readable(&mut self, poll: &mut Poll) {

        // if self.state == DexDataWorkerState::Idle {
        //     self.request = Box::new(RPCRequest::new());
        // }
        
        match RPCRequest::consume(&mut self.stream) {
            Ok(request) => {
                if request.name == "CloseConnection" {
                    self.state = DexDataWorkerState::Closed;
                }

                println!("RPC name: {}", request.name);
                println!("Handled client with token {:?}.", self.token);
            },

            Err(_) => {
                ()
            }
        }
        
        match self.state {
            DexDataWorkerState::Closed => (),
            _ => poll.reregister(&self.stream, self.token, Ready::writable(),
                                 PollOpt::edge() | PollOpt::oneshot()).unwrap(),
        }
    }

    pub fn handle_writable(&mut self, poll: &mut Poll) {
        println!("Prepare to write with token {:?}", self.token);

        let tmp = TestPayload {
            a: 31,
            b: 37,
        };

        let response = create_json_response(&tmp);

        self.stream.write(response.as_bytes()).unwrap();

        poll.reregister(&self.stream, self.token, Ready::readable(),
                        PollOpt::edge() | PollOpt::oneshot()).unwrap();
    }

    pub fn is_closed(&self) -> bool {
        match self.state {
            DexDataWorkerState::Closed => true,
            _ => false,
        }
    }
}
