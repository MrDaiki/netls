use std::env;

use netls::config::ServerConfig;
use netls::server;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error : incorrect number of arguments");
        return;
    } else {
        let config = ServerConfig::from_path(&args[1]);
        server::main_loop(config);
    }
}
