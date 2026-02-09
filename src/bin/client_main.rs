use std::env;

use netls::client;
use netls::config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error : incorrect number of arguments");
        return;
    } else {
        let config = config::ClientConfig::from_path(args[1].as_str());

        client::main_loop(config);
    }
}
