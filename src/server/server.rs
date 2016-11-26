use std::io;
use mio::{Poll, Events, Ready, PollOpt, Token};
use mio::tcp::{TcpListener, TcpStream};
use slab::Slab;

use server::DexDataWorker;

const SERVER_TOKEN: Token = Token(131071);
const DEFAULT_TRANSACTION_POOL_SIZE: usize = 1024;

pub struct DexDataServer {
    pub socket: TcpListener,

    poll: Poll,

    // Slab<T> is a map between token and T, where token is generated
    // from a token pool managed by a free list.
    pool: Slab<DexDataWorker, Token>,
}

impl DexDataServer {
    pub fn new(address: &str, max_pool_size: usize) -> DexDataServer {
        let socket_address = address.parse().expect(
            "Failed to parse the address.");

        let server_socket = TcpListener::bind(&socket_address).expect(
            "Failed to bind to the server socket address.");
        let poll = Poll::new().unwrap();
        poll.register(&server_socket, SERVER_TOKEN,
                      Ready::readable(),
                      PollOpt::edge()).or_else(|e| {
                          println!("Failed to register the server socket, {}", e);
                          Err(e)
                      });

        DexDataServer {
            socket: server_socket,
            poll: poll,
            pool: Slab::with_capacity(max_pool_size)
        }
    }

    pub fn new_simple(address: &str) -> DexDataServer {
        Self::new(address, DEFAULT_TRANSACTION_POOL_SIZE)
    }

    pub fn run(&mut self, events_capacity: usize) {
        let mut events = Events::with_capacity(events_capacity);
        
        // The main event loop of the server. Every time it polls for
        // newly arrived events and handles them respectively. See the
        // loop for details about each type of the events.
        loop {
            self.poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {

                    SERVER_TOKEN => {
                        let client_socket = match self.socket.accept() {
                            Err(e) => {
                                println!("Server socket accept error: {}", e);
                                return;
                            },

                            // Ok((None, _)) => unreachable!("Server socket accept returns None"),
                            Ok((socket, _ /* unused address */)) => socket
                        };
                        
                        // We need to get a token to assign to the new
                        // (client) socket. Our slab-based pool can generate
                        // an unused one from the underlying free list.

                        let token = match self.pool.vacant_entry() {
                            Some(entry) => {
                                let new_token = entry.index();
                                entry.insert(DexDataWorker::new(
                                    new_token, client_socket)).index()
                            },
                            None => {
                                println!("Failed to create a new DexDataWorker.");
                                return;
                            }
                        };
                        
                        self.poll.register(&self.pool[token].stream,
                                           token,
                                           Ready::readable(),
                                           PollOpt::edge() | PollOpt::oneshot()).expect(
                            "Failed to register a new client socket.");
                        
                        println!("Inserting client socket {:?} to {:?}",
                                 self.pool[token].stream, token);
                    },

                    token => {                        

                        if event.kind().is_readable() {
                            let mut worker = self.pool.get_mut(token).unwrap();
                            worker.handle_readable(&mut self.poll);
                        }

                        if event.kind().is_writable() {
                            let mut worker = self.pool.get_mut(token).unwrap();
                            worker.handle_writable(&mut self.poll);            
                        }
                        
                        if self.pool.get(token).unwrap().is_closed() {
                            println!("Remove client {:?}.", token);
                            self.pool.remove(token);
                        }
                    }
                }
            }
        }
    }
}
