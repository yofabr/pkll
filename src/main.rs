use clap::Parser;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix;

mod helper;
mod platform;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    port: u16,
}

fn main() {
    let args = Args::parse();
    let port = args.port;

    platform::handle(port);
}
