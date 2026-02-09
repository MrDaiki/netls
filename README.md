# netls (network ls)

## Description

Little library experimentation of rust capabilities to implement newtorking (for education and training purposes).

This project is a client-server implementation of a command to list remote files. The server is multi-client using threads. Part of how server and client communicate is based on [FTP protocol](https://datatracker.ietf.org/doc/html/rfc959).

No authentication or security mechanism is build (and it will probably never). This crate should thus NEVER be in real world application.

## Building

You need to have [cargo](https://rust-lang.org/) installed to build the binaries for client and server. When it is installed, just run : 
```bash
cargo build --release
```

this will produce two binaries in `target/release` : 
* server_main
* client_main

## Usage

you run the correct executable using : 
```
<executable_name> <config_path>
```

Client and Server each need a configuration json file as 1st command argument (See example json files provided at project root).

Client, after a successful connection to the server, will prompt available commands and await for user input :
```
LIST <filepath>
EXIT
```

More command may be added to the project later.

## Licensing

MIT license (see `LICENSE`)