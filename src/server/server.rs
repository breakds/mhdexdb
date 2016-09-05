use std::io;
use mio::{EventLoop, EventSet, Handler, PollOpt, Token, TryRead, TryWrite};
use mio::tcp::{TcpListener, TcpStream};
use mio::util::Slab;

use server::DexDataWorker;

const SERVER_TOKEN: Token = Token(1);
const DEFAULT_TRANSACTION_POOL_SIZE: usize = 1024;

pub struct DexDataServer {
    pub socket: TcpListener,

    // Slab<T> is a map between token and T, where token is generated
    // from a token pool managed by a free list.
    pool: Slab<DexDataWorker>,
}

impl DexDataServer {
    pub fn new(address: &str, max_pool_size: usize) -> DexDataServer {
        let socket_address = address.parse().expect(
            "Failed to parse the address.");
        
        DexDataServer {
            socket: TcpListener::bind(&socket_address).expect(
                "Failed to bind to the server socket address."),
            pool: Slab::new_starting_at(
                Token(SERVER_TOKEN.as_usize() + 1),
                max_pool_size)
        }
    }

    pub fn new_simple(address: &str) -> DexDataServer {
        Self::new(address, DEFAULT_TRANSACTION_POOL_SIZE)
    }

    pub fn register(&self, event_loop: &mut EventLoop<DexDataServer>)
                    -> io::Result<()> {
        event_loop.register(&self.socket,
                            SERVER_TOKEN,
                            EventSet::readable(),
                            PollOpt::edge()).or_else(|e| {
                                println!("Failed to register the server socket, {}", e);
                                Err(e)
                            })
    }
}

impl Handler for DexDataServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop:  &mut EventLoop<DexDataServer>,
             token: Token, interest: EventSet) {
        // Here we are implementing the ready() function of Handler,
        // provided by mio.
        //
        // This function is invoked when never an event whose type we
        // are interested in occurs.
        //
        // The argurment token indicates which socket/transaction is
        // notifying such event.
        //
        // The argument interest is a bit set indicating the type of
        // the event, such as readable, writable, etc.

        if interest.is_readable() {
            match token {
                SERVER_TOKEN => {
                    let client_socket = match self.socket.accept() {
                        Err(e) => {
                            println!("Server socket accept error: {}", e);
                            return;
                        },

                        Ok(None) => unreachable!("Server socket accept returns None"),
                        Ok(Some((socket, _ /* unused address */))) => socket
                    };

                    // We need to get a token to assign to the new
                    // (client) socket. Our slab-based pool can generate
                    // an unused one from the underlying free list.
                    let token = self.pool.insert_with(|token| {
                        DexDataWorker::new(token, client_socket)
                    }).expect(
                        "Failed to insert socket to pool. The pool might be full.");

                    event_loop.register(&self.pool[token].stream, token, EventSet::readable(),
                                        PollOpt::edge() | PollOpt::oneshot()).expect(
                        "Failed to register a new client socket.");
                    
                    println!("Inserting client socket {:?} to {:?}",
                             self.pool[token].stream, token);
                },
                token => {
                    {
                        let mut client = self.pool.get_mut(token).unwrap();
                        client.handle_readable(event_loop);
                    }
                    
                    if self.pool.get(token).unwrap().is_closed() {
                        println!("Remove client {:?}.", token);
                        self.pool.remove(token);
                    }
                }
            }
        }
         

        if interest.is_writable() {
            let mut worker = self.pool.get_mut(token).unwrap();
            worker.handle_writable(event_loop);
        }
        
    }
}
