use clap::Parser;
use std::env;

mod helper;

#[derive(Parser, Debug)]
struct Args {
    port: u16,
}

fn main() {
    let args = Args::parse();
    let port = args.port;
    let os = env::consts::OS; 

    helper::handle(os, port);
    
}