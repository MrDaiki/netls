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

For the moment, the server and client are set to be launched on localhost.

No configuration for the server is avaible in the current version.

Client can be launched with a dedicated port as an argument (it is usefull to test multi-client capabilities).
Client, after a successful connection to the server, will prompt available commands and await for user input :
```
LIST <filepath>
EXIT
```

More command may be added to the project later.

## Licensing

MIT license (see `LICENSE`)