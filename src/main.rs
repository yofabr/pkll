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

    match os {
        "windows" => helper::handler_for_windows(port),
        "macos" => helper::handler_for_unix(port),
        "linux" => helper::handler_for_unix(port),
        _ => {

        }
    }
    
}