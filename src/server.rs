
use std::{
    fs::read_dir,
    net::{Ipv4Addr, Shutdown, TcpListener, TcpStream},
    path::Path,
    thread,
};

use crate::message::message::{EnableConnectResponse, Message, MessageKind, ResponseFileTree};

/*
    Inner handling function when connection is established with client
*/
fn handle_message(stream: &mut TcpStream, adress: String) {
    loop {
        let message = MessageKind::read_from_stream(stream);
        match message {
            Some(message) => match message {
                MessageKind::EnableConnect(_) => {
                    eprintln!(
                        "[{}]Error : unexpected message at this time, connexion already established",
                        adress
                    );
                }
                MessageKind::EnableConnectResponse(_) => {
                    eprintln!(
                        "[{}]Error : server should never receive a EnableConnectResponse",
                        adress
                    );
                }
                MessageKind::ResponseFileTree(_) => {
                    eprintln!(
                        "[{}]Error : server should never receive a ResponseFileTree",
                        adress
                    );
                }
                MessageKind::ResponseFileTreeError => {
                    eprintln!(
                        "[{}]Error : server should never receive a ResponseFileTreeError",
                        adress
                    );
                }
                MessageKind::AskFileTree(ask_file_tree) => {
                    let path = ask_file_tree.path.trim();
                    let path = Path::new(path);
                    match read_dir(path) {
                        Ok(dir) => {
                            let mut res = Vec::new();
                            for entry in dir {
                                match entry {
                                    Ok(path) => match path.file_name().into_string() {
                                        Ok(path) => {
                                            res.push(path);
                                        }
                                        Err(err) => {
                                            eprintln!("[{}] Error on osString : {:?}", adress, err);
                                        }
                                    },
                                    Err(err) => {
                                        eprintln!("[{}] Error on dir entry : {}", adress, err);
                                    }
                                }
                            }
                            let content = ResponseFileTree { paths: res };
                            MessageKind::ResponseFileTree(content).write_to_stream(stream);
                        }
                        Err(err) => {
                            eprintln!("[{}] Error reading directory : {}", adress, err);
                            MessageKind::ResponseFileTreeError.write_to_stream(stream);
                        }
                    }
                }
                MessageKind::Disconnect => {
                    println!("[{}] Disconnecting from client", adress);
                    break;
                }
            },
            None => {
                break;
            }
        }
    }
}

fn handle_connect(mut stream: TcpStream) {
    let message = MessageKind::read_from_stream(&mut stream);
    match message {
        Some(message) => match message {
            MessageKind::EnableConnect(enable_connect) => {
                let adress = format!("{}:{}", enable_connect.ip, enable_connect.port);
                let connectionsocket = TcpStream::connect(&adress);
                match connectionsocket {
                    Ok(mut client_stream) => {
                        println!("[{}] Connected to Client", adress);
                        stream.shutdown(Shutdown::Both).unwrap(); // we close the temp stream since we are connected to the client
                        let resp = EnableConnectResponse { result: true };
                        MessageKind::EnableConnectResponse(resp)
                            .write_to_stream(&mut client_stream);
                        handle_message(&mut client_stream, adress);
                    }
                    Err(err) => {
                        eprintln!("Error : unable to open connexion to '{}' : {}", adress, err);
                    }
                }
            }
            _ => {}
        },
        None => {}
    }
}

pub fn main_loop(ip: Ipv4Addr, port: u16) {
    let listener = match TcpListener::bind(format!("{}:{}", ip, port)) {
        Ok(listener) => listener,
        Err(err) => panic!("Error starting server : {}", err),
    };

    println!("Server socket sucessfully binded, awaiting connections");

    for acc in listener.incoming() {
        match acc {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connect(stream);
                });
            }
            Err(err) => {
                eprintln!("Error receiving message : {}", err);
            }
        }
    }
}
