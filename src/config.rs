use std::{fs, net::Ipv4Addr};

use serde::{Deserialize};
use serde_json;



#[derive(Deserialize)]
pub struct ClientConfig {
    server_adress : String,
    server_port : usize,
    client_adress: String,
    client_port : usize
}

fn string_to_ipv4addr(string: &String) -> Ipv4Addr {
    let sub:Vec<&str> = string.split(".").collect();

    if sub.len() != 4 {
        panic!("Error : Incorrect adress pattern");
    }

    let sub_u8:Vec<u8> = sub.iter().map(|e| {
        match e.parse::<u8>() {
            Ok(v) => v,
            Err(err) => panic!("Error : incorrect integer format for ip : {}",err),
        }
    }).collect();



    return Ipv4Addr::new(sub_u8[0],sub_u8[1],sub_u8[2],sub_u8[3]);

}

impl ClientConfig {
    pub fn from_path(filepath: &str) -> Self {
        match fs::read_to_string(filepath) {
            Ok(jsonstring) => {
                match serde_json::from_str::<ClientConfig>(&jsonstring) {
                    Ok(config) => {
                        return config;
                    },
                    Err(err) => {
                        panic!("Error : unable to deserialize configuration : {}",err);
                    },
                }
            },
            Err(err) => {
                panic!("Error : unable to load configuration : {}",err);
            },
        }
    }

    pub fn get_server_adress(&self) -> String {
        return format!("{}:{}", self.server_adress,self.server_port);
    }

    pub fn get_client_adress(&self) -> (Ipv4Addr, u16) {
        let ip = string_to_ipv4addr(&self.client_adress);
        let port = self.client_port as u16;
        return (ip,port);
    }

}

#[derive(Deserialize)]
pub struct ServerConfig {
    server_adress : String,
    server_port : usize
}

impl ServerConfig {
    pub fn from_path(filepath: &str) -> Self {
        match fs::read_to_string(filepath) {
            Ok(jsonstring) => {
                match serde_json::from_str::<ServerConfig>(&jsonstring) {
                    Ok(config) => {
                        return config;
                    },
                    Err(err) => {
                        panic!("Error : unable to deserialize configuration : {}", err)
                    },
                }
            },
            Err(err) => {
                panic!("Error : unable to load configuration : {}",err);
            },
        }
    }

    pub fn server_adress(&self) -> String {
        return format!("{}:{}",self.server_adress,self.server_port);
    }
}
