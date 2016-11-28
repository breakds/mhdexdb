#[macro_use]
extern crate log;

extern crate http_muncher;
extern crate mio;
extern crate rustc_serialize;
extern crate slab;

// Data Types
pub mod data;

// Server
pub mod server;

// Unit Tests
mod tests {
    mod utils;
}
