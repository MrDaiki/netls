use std::{
    io::{Read, Write},
    net::{Ipv4Addr, TcpStream},
    u8,
};

/*
    Message struct going to network must implement this trait
*/
pub trait Message {
    fn read_from_stream(stream: &mut TcpStream) -> Option<Self>
    where
        Self: Sized;
    fn write_to_stream(self, stream: &mut TcpStream); // Write the message and CONSUME it
}

pub struct EnableConnect {
    pub ip: Ipv4Addr, // on network, it is stored as 4 successive u8
    pub port: u16,
}

impl Message for EnableConnect {
    fn read_from_stream(stream: &mut TcpStream) -> Option<Self>
    where
        Self: Sized,
    {
        // we read 6 bytes for the buffer : 4 for ip adress, 2 for port
        let mut buffer = [0u8; 6];
        match stream.read_exact(&mut buffer) {
            Ok(_) => {
                let adress = Ipv4Addr::new(buffer[0], buffer[1], buffer[2], buffer[3]);
                let port = u16::from_le_bytes([buffer[4], buffer[5]]);
                return Some(EnableConnect {
                    ip: adress,
                    port: port,
                });
            }
            Err(err) => {
                eprintln!("Error handling EnableConnect : {}", err);
                None
            }
        }
    }

    fn write_to_stream(self, stream: &mut TcpStream) {
        let ip_buffer = self.ip.octets();
        let port_buffer = u16::to_le_bytes(self.port);
        let mut full_buffer = [0u8; 6];

        full_buffer[0] = ip_buffer[0];
        full_buffer[1] = ip_buffer[1];
        full_buffer[2] = ip_buffer[2];
        full_buffer[3] = ip_buffer[3];
        full_buffer[4] = port_buffer[0];
        full_buffer[5] = port_buffer[1];

        match stream.write_all(&full_buffer) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error sending EnableConnect : {}", err);
            }
        }
    }
}

/*
    Structure representing Connection enabling response : return 1 if received correctly, zero otherwise
*/
pub struct EnableConnectResponse {
    pub result: bool,
}

impl Message for EnableConnectResponse {
    fn read_from_stream(stream: &mut TcpStream) -> Option<Self>
    where
        Self: Sized,
    {
        let mut buffer = [0u8];
        match stream.read_exact(&mut buffer) {
            Ok(()) => {
                let mut res = false;

                if buffer[0] > 0 {
                    res = true;
                }

                return Some(EnableConnectResponse { result: res });
            }
            Err(err) => {
                eprintln!("Error receiving EnableConnectResponse : {}", err);
                return None;
            }
        }
    }

    fn write_to_stream(self, stream: &mut TcpStream) {
        let content = if self.result { 255 as u8 } else { 0u8 };
        match stream.write_all(&[content]) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Error sending EnableConnectResponse : {}", err);
            }
        }
    }
}

pub struct AskFileTree {
    pub path: String, // payload should look like : | path.len() as u16 | path |
}

fn write_to_stream_string(string: String, stream: &mut TcpStream) -> Result<(), String> {
    let len = string.len() as u16;
    let bytes = string.as_bytes();

    let mut len_buffer = u16::to_le_bytes(len);
    match stream.write_all(&mut len_buffer) {
        Ok(()) => match stream.write_all(bytes) {
            Ok(()) => {
                return Ok(());
            }
            Err(err) => {
                return Err(format!("Error serialising string : {}", err));
            }
        },
        Err(err) => {
            return Err(format!("Error serialising string length : {}", err));
        }
    }
}

fn read_from_stream_string(stream: &mut TcpStream) -> Result<String, String> {
    let mut strsizebuffer = [0u8; 2];
    match stream.read_exact(&mut strsizebuffer) {
        Ok(_) => {
            let strsize = u16::from_le_bytes(strsizebuffer);
            let mut strbuffer = vec![0u8; strsize as usize];

            match stream.read_exact(&mut strbuffer) {
                Ok(_) => {
                    let final_str = String::from_utf8(strbuffer).unwrap();
                    return Ok(final_str);
                }
                Err(err) => {
                    return Err(format!("Error receiving string : {}", err));
                }
            }
        }
        Err(err) => {
            return Err(format!("Error receiving string length : {}", err));
        }
    }
}

impl Message for AskFileTree {
    fn read_from_stream(stream: &mut TcpStream) -> Option<Self>
    where
        Self: Sized,
    {
        match read_from_stream_string(stream) {
            Ok(str) => {
                return Some(AskFileTree { path: str });
            }
            Err(err) => {
                eprintln!("{}", err);
                return None;
            }
        }
    }

    fn write_to_stream(self, stream: &mut TcpStream) {
        match write_to_stream_string(self.path, stream) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("Error sending path request : {}", err);
            }
        }
    }
}

pub struct ResponseFileTree {
    // we can send a maximun of maxu16 number of paths
    // path size must be with a maximum of maxu16 size
    pub paths: Vec<String>, //payload should look like | paths.len() as u16 | paths[0].len() | paths[0]| ...
}

impl Message for ResponseFileTree {
    fn read_from_stream(stream: &mut TcpStream) -> Option<Self>
    where
        Self: Sized,
    {
        let mut size_buffer = [0u8; 2];

        match stream.read_exact(&mut size_buffer) {
            Ok(_) => {
                let size = u16::from_le_bytes(size_buffer);
                let mut ret = Vec::new();

                for _ in 0..size {
                    match read_from_stream_string(stream) {
                        Ok(str) => {
                            ret.push(str);
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                            return None;
                        }
                    }
                }

                return Some(ResponseFileTree { paths: ret });
            }
            Err(err) => {
                eprintln!("Error receiving paths number : {}", err);
                return None;
            }
        }
    }

    fn write_to_stream(self, stream: &mut TcpStream) {
        let paths_size_buffer = u16::to_le_bytes(self.paths.len() as u16);
        match stream.write_all(&paths_size_buffer) {
            Ok(()) => {
                for str in self.paths.into_iter() {
                    match write_to_stream_string(str, stream) {
                        Ok(()) => {}
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error Serializing paths length : {}", err);
            }
        }
    }
}

/* 
Message wrapping to determine how to what kind of message is received
each type correspond to a single u8 message type going from 1 increasing
*/
pub enum MessageKind {
                                                    // Network message code
    EnableConnect(EnableConnect),                   // 1
    EnableConnectResponse(EnableConnectResponse),   // 2
    AskFileTree(AskFileTree),                       // 3
    ResponseFileTree(ResponseFileTree),             // 4
    ResponseFileTreeError,                          // 5
    Disconnect,                                     // 6
}

impl Message for MessageKind {
    fn read_from_stream(stream: &mut TcpStream) -> Option<Self>
    where
        Self: Sized,
    {
        let mut message_type_buffer = [0u8];
        match stream.read_exact(&mut message_type_buffer) {
            Ok(_) => {
                match message_type_buffer[0] {
                    1 => {
                        let content = EnableConnect::read_from_stream(stream);
                        match content {
                            Some(content) => {
                                return Some(MessageKind::EnableConnect(content));
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                    2 => {
                        let content = EnableConnectResponse::read_from_stream(stream);
                        match content {
                            Some(content) => {
                                return Some(MessageKind::EnableConnectResponse(content));
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                    3 => {
                        let content = AskFileTree::read_from_stream(stream);
                        match content {
                            Some(content) => {
                                return Some(MessageKind::AskFileTree(content));
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                    4 => {
                        let content = ResponseFileTree::read_from_stream(stream);
                        match content {
                            Some(content) => {
                                return Some(MessageKind::ResponseFileTree(content));
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                    5 => {
                        return Some(MessageKind::ResponseFileTreeError);
                    }
                    6 => {
                        return Some(MessageKind::Disconnect);
                    }
                    _ => {
                        //error : not expected
                        eprintln!(
                            "Error : unknown message type code {}",
                            message_type_buffer[0]
                        );
                        return None;
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading message type buffer : {}", err);
                return None;
            }
        }
    }

    fn write_to_stream(self, stream: &mut TcpStream) {
        match self {
            MessageKind::EnableConnect(payload) => match stream.write_all(&[1 as u8]) {
                Ok(()) => {
                    payload.write_to_stream(stream);
                }
                Err(err) => {
                    eprintln!("Error sending EnableConnect : {}", err);
                }
            },
            MessageKind::EnableConnectResponse(payload) => match stream.write_all(&[2 as u8]) {
                Ok(()) => {
                    payload.write_to_stream(stream);
                }
                Err(err) => {
                    eprintln!("Error sending EnableConnectResponse : {}", err);
                }
            },
            MessageKind::AskFileTree(payload) => match stream.write_all(&[3 as u8]) {
                Ok(()) => {
                    payload.write_to_stream(stream);
                }
                Err(err) => {
                    eprintln!("Error sending AskFileTree : {}", err);
                }
            },
            MessageKind::ResponseFileTree(payload) => match stream.write_all(&[4 as u8]) {
                Ok(()) => {
                    payload.write_to_stream(stream);
                }
                Err(err) => {
                    eprintln!("Error sending ResponseFileTree : {}", err);
                }
            },
            MessageKind::ResponseFileTreeError => match stream.write_all(&[5 as u8]) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error sending ResponseFileTreeError : {}", err);
                }
            },
            MessageKind::Disconnect => match stream.write_all(&[6 as u8]) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error sending Disconnect : {}", err);
                }
            },
        }
    }
}

// fn read_message_kind(stream: &mut TcpStream) -> Result<MessageKind, String> {
//     let mut buf = [0u8];
//     let result = stream.read_exact(&mut buf);
//     match result {
//         Ok(()) => {
//             let code = buf[0];
//             match code {
//                 0 => Ok(MessageKind::EnableConnect),
//                 1 => Ok(MessageKind::AskFile),
//                 2 => Ok(MessageKind::AskFileTree),
//                 _ => Err("Request code doesn't exist".to_string()),
//             }
//         }
//         Err(err) => {
//             let errstring = format!("{}", &err);
//             return Err(errstring);
//         }
//     }
// }
