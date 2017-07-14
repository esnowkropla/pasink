extern crate pasink;

use std::env;
use pasink::{server, client};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "server" {
        server(args[2].as_str());
    } else if args[1] == "client" {
        client(args[2].as_str());
    }
}
