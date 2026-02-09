use std::{
    io::Write,
    net::{SocketAddr, TcpListener, TcpStream},
};

use crate::{
    config::ClientConfig,
    message::message::{AskFileTree, EnableConnect, Message, MessageKind},
};

fn get_client_instruction() -> Option<MessageKind> {
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut read = String::new();
    match std::io::stdin().read_line(&mut read) {
        Ok(_) => {
            if read.len() < 4 {
                return None;
            }

            let command = &read[0..4];

            match command {
                "LIST" => {
                    // we check if LIST is followed by a space
                    if read.len() < 6 {
                        return None;
                    }

                    let command_path = &read[5..];
                    let command = AskFileTree {
                        path: String::from(command_path),
                    };
                    return Some(MessageKind::AskFileTree(command));
                }
                "EXIT" => {
                    return Some(MessageKind::Disconnect);
                }
                _ => {
                    return None;
                }
            }
        }
        Err(_) => todo!(),
    }
}

fn handle_message(stream: &mut TcpStream, adress: SocketAddr) {
    loop {
        println!("Connected to : {}", adress);
        println!("Awaiting request : ");
        println!("LIST <filepath>");
        println!("EXIT");

        match get_client_instruction() {
            Some(message_kind) => match message_kind {
                MessageKind::Disconnect => {
                    message_kind.write_to_stream(stream);
                    break;
                }
                MessageKind::AskFileTree(_) => {
                    message_kind.write_to_stream(stream);
                    let filetree_response = MessageKind::read_from_stream(stream);
                    match filetree_response {
                        Some(data) => match data {
                            MessageKind::ResponseFileTree(response_file_tree) => {
                                for file in response_file_tree.paths {
                                    println!("- {}", file);
                                }
                                println!("Press ENTER");
                            }
                            MessageKind::ResponseFileTreeError => {
                                println!("Response Error : Unable to access distant remote file");
                                println!("Press ENTER");
                            }
                            _ => {
                                eprintln!(
                                    "Incorrect request in response, ResponseFileTree or ResponseFileTreeError was expected"
                                );
                                println!("Press ENTER");
                            }
                        },
                        None => {}
                    }
                }
                _ => {}
            },
            None => {
                println!("Error : Invalid input, press ENTER");
            }
        }

        let _ = std::io::stdin().read_line(&mut String::new());
        print!("\x1B[2J\x1B[1;1H");
    }
}

pub fn main_loop(config: ClientConfig) {
    let server_adress = config.get_server_adress();
    let (client_adress, client_port) = config.get_client_adress();
    // Connection to the server
    match TcpStream::connect(server_adress) {
        Ok(mut server_stream) => {
            let self_stream = TcpListener::bind(format!("{}:{}", client_adress, client_port));

            match self_stream {
                Ok(self_stream) => {
                    MessageKind::EnableConnect(EnableConnect {
                        ip: client_adress,
                        port: client_port,
                    })
                    .write_to_stream(&mut server_stream);
                    let stream = self_stream.accept();
                    match stream {
                        Ok((mut client_stream, addr)) => {
                            server_stream.shutdown(std::net::Shutdown::Both).unwrap(); // We close the original server stream
                            let response = MessageKind::read_from_stream(&mut client_stream);
                            match response {
                                Some(message) => {
                                    match message {
                                        MessageKind::EnableConnectResponse(
                                            enable_connect_response,
                                        ) => {
                                            if !enable_connect_response.result {
                                                eprintln!(
                                                    "Error : false EnableConnectResponse sent by the server"
                                                );
                                                let disconnect: MessageKind =
                                                    MessageKind::Disconnect;
                                                disconnect.write_to_stream(&mut client_stream);
                                                return;
                                            }
                                            handle_message(&mut client_stream, addr);
                                        }
                                        _ => {
                                            // Unexpected message received : We disconnect
                                            eprintln!(
                                                "Error : EnableConnectResponse was expected but another message was received instead"
                                            );
                                            let disconnect = MessageKind::Disconnect;
                                            disconnect.write_to_stream(&mut client_stream);
                                        }
                                    }
                                }
                                None => todo!(),
                            }
                        }
                        Err(err) => {
                            eprintln!("Error opening server socket : {}", err);
                        }
                    }
                }
                Err(_) => todo!(),
            }
            // sending connexion adress after server is initialised
        }
        Err(err) => {
            panic!("Error : unable to connect to the server : {}", err);
        }
    }
}
