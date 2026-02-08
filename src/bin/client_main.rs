use std::{env, net::Ipv4Addr};

use netls::client;

fn main() {
    let server_adress = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8000 as u16;

    let args: Vec<String> = env::args().collect();
    let client_port: u16;

    if args.len() < 2 {
        println!("No client port provided as first argument, setting it to 50000");
        client_port = 50000;
    } else {
        match args[1].parse::<u16>() {
            Ok(arg) => {
                client_port = arg;
            }
            Err(err) => {
                eprintln!(
                    "Error : unable to format first argument (client port) : {}",
                    err
                );
                println!("Setting client port to 50000");
                client_port = 50000;
            }
        }
    }

    client::main_loop(server_adress, port, client_port);
}
