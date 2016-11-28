use std::io::{ErrorKind};
use std::result::Result;
use std::string::String;
use mio::{Poll, Event, Events, Ready, PollOpt, Token};
use mio::tcp::{TcpListener, TcpStream};
use slab::Slab;

use server::DexDataWorker;

// We need to use the server token to distinguish the events for
// server and the events for children connections.
//
// Since we are using a slab to store the children connections, they
// will naturally be given tokens from 0, 1, 2 ... So we need to make
// sure that we pick a token for the server that will be different
// from the others (a very big integer would suffice).
const SERVER_TOKEN: Token = Token(1310719999);
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
            format!("Failed to parse the address {}.",
                    address).as_str());
        
        let server_socket = TcpListener::bind(&socket_address).expect(
            "Failed to bind to the server socket address.");

        let poll = Poll::new().unwrap();

        // Register for listening to the readable event of the server
        // TcpListener.
        poll.register(&server_socket, SERVER_TOKEN,
                      Ready::readable(),
                      PollOpt::edge()).unwrap();
        
        DexDataServer {
            socket: server_socket,
            poll: poll,
            pool: Slab::with_capacity(max_pool_size)
        }
    }

    pub fn new_simple(address: &str) -> DexDataServer {
        Self::new(address, DEFAULT_TRANSACTION_POOL_SIZE)
    }

    pub fn handle_event(&mut self, event: Event)
                        -> Result<(), String> {
        debug!("[Token {:?}] Handle event: {:?}", event.token(), event);
        
        if event.kind().is_error() {
            return Err(format!("Error event for [Token {:?}]", event.token()));
        }

        if event.kind().is_hup() {
            return Err(format!("HUP event for [Token {:?}]", event.token()));
        }
        
        match event.token() {
            SERVER_TOKEN => {
                let new_socket = try!(match self.socket.accept() {
                    Ok((socket, _)) => Ok(socket),
                    Err(e) => match e.kind() {
                        ErrorKind::WouldBlock =>
                            Err(format!("WouldBlock on Accept.")),
                        _ => Err(format!("Failed to accept: {:?}", e)),
                    }
                });

                let token = try!(match self.pool.vacant_entry() {
                    Some(entry) => {
                        let new_token = entry.index();
                        Ok(entry.insert(DexDataWorker::new(
                            new_token, new_socket)).index())
                    },
                    None => Err("No vacant in server slab.")
                });

                match self.poll.register(&self.pool[token].stream,
                                         token, Ready::readable(),
                                         PollOpt::edge() |
                                         PollOpt::oneshot()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to register: {:?}", e))
                }
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
                    self.pool.remove(token);
                }

                Ok(())
            },
        }

        // match event.token() {
            
        //     SERVER_TOKEN => {
        //         let client_socket = match self.socket.accept() {
        //             Err(e) => {
        //                 println!("Server socket accept error: {}", e);
        //                 return;
        //             },

        //             // Ok((None, _)) => unreachable!("Server socket accept returns None"),
        //             Ok((socket, _ /* unused address */)) => socket
        //         };
                
        //         // We need to get a token to assign to the new
        //         // (client) socket. Our slab-based pool can generate
        //         // an unused one from the underlying free list.

        //         let token = match self.pool.vacant_entry() {
        //             Some(entry) => {
        //                 let new_token = entry.index();
        //                 entry.insert(DexDataWorker::new(
        //                     new_token, client_socket)).index()
        //             },
        //             None => {
        //                 println!("Failed to create a new DexDataWorker.");
        //                 return;
        //             }
        //         };
                
        //         self.poll.register(&self.pool[token].stream,
        //                            token,
        //                            Ready::readable(),
        //                            PollOpt::edge() | PollOpt::oneshot()).expect(
        //             "Failed to register a new client socket.");
                
        //         println!("Inserting client socket {:?} to {:?}",
        //                  self.pool[token].stream, token);
        //     },

        //     token => {                        

        //         if event.kind().is_readable() {
        //             let mut worker = self.pool.get_mut(token).unwrap();
        //             worker.handle_readable(&mut self.poll);
        //         }

        //         if event.kind().is_writable() {
        //             let mut worker = self.pool.get_mut(token).unwrap();
        //             worker.handle_writable(&mut self.poll);            
        //         }
                
        //         if self.pool.get(token).unwrap().is_closed() {
        //             println!("Remove client {:?}.", token);
        //             self.pool.remove(token);
        //         }
        //     }
        // }
    }

    // The run function starts the main loop and will not stop until
    // the whole program termintates. This is usually the last method
    // being called in main().
    pub fn run(&mut self, events_capacity: usize) {
        // The container for polling events. 
        let mut events = Events::with_capacity(events_capacity);
        
        // The main event loop of the server. Every time it polls for
        // newly arrived events and handles them respectively. See the
        // loop for details about each type of the events.
        loop {
            // Poll the events and store them in `events`.
            self.poll.poll(&mut events, None).unwrap();

            // Iterate over the events and decide how to handle them
            // based on their token (token determines whether it is
            // for the server TcpListener or the other TcpStreams.
            for event in events.iter() {
                match self.handle_event(event) {
                    Err(error_message) =>
                        warn!("Unsuccessful event handling: {:?}.",
                              error_message),
                    Ok(_) => (),
                };
            }
        }  // main loop
    }
}
