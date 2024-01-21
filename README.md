# Nik's Open-source Smart Home Platform
NOSHP is a platform for creators to build upon, which supplies the basic building blocks of a smart home system.

It focuses on:
- Being easy to setup for novice users
- Easy access to all internal systems
- An easy way to setup new custom devices

Please view the introduction (needs hyperlink) and example device (needs hyperlink) to get started

## Installation
- Docker containers will be made available at a later date
- Building from source is easy using using cargo [Rust's cargo build system](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Binaries are available from the [releases page](https://github.com/niknik3610/IoT_Platform/releases)

Required packages:
```
sudo apt install protobuf-compiler
```

## Project Layout
    ├─ backend/           Rust Files, used in client, server and frontend backend
      ├─ proto/           Protobuf files
      │  ├─ frontend/     Protobuf files between server and frontend
      │  ├─ iot/          Protobuf files between server and devices
      ├─ src/             Application source code
      │  ├─ client/       Client source code 
      │  ├─ frontend/     Frontend source code
      │  └─ server/       Server source code
    
## Todo before v0.1:
- [X] Add config files for clients
- [X] Add optional capabilities (capability 'turn on' should only be available if device is in state 'off')
- [X] Add custom capabilities, using strings instead of an enum
- [X] Add ability to hand client callback functions to be called when certain capabilities are triggered
- [X] Get client working on a Raspberry Pi
- [ ] Get network service discovery working between linux devices
- [ ] Write proper run scripts for web-frontend/backend
- [X] Write a proper README


## Possible future additions
Nothing here ATM

