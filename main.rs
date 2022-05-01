use std::io;
use std::time;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;

use std::str;
use std::env;

pub mod client;
pub mod server;
use crate::client::client::run_client;
use crate::server::server::run_server;


fn main() {
    let args: Vec<String> = env::args().collect();
    let server_type = &args[1];

    println!("{}",server_type);
    if server_type == "server" {
        run_server().unwrap();
    }
    if server_type == "client" {
        run_client().unwrap();
    }

    println!("Done!?");
}
