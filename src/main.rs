use clap::Parser;
use webserver::start_server;

#[derive(Parser, Debug)]
struct Cli {
    host: String,
    port: String,
    num_of_threads: usize,
}

fn main() {
    let args = Cli::parse();
    start_server(args.host, args.port, args.num_of_threads);
}
