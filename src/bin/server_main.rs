use std::net::Ipv4Addr;

use netls::server;

fn main() {
    let server_adress = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8000 as u16;

    server::main_loop(server_adress, port);
}
