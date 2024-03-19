# Nik's Open-source Smart Home Platform
NOSHP is a framework for creators to build their own smart home devices, which supplies the basic building blocks of a smart home system.

It focuses on:
- Being easy to setup for novice users
- Easy access to all internal systems
- An easy way to setup new custom devices

Please view the introduction (needs hyperlink) and example device (needs hyperlink) to get started

## Building and Running
- The Rust compiler will be required to build and run the server. This should be installed using [Rustup](https://rustup.rs/).
- The node package manager (NPM) will be required for running the frontend, installation instructions can be found [here](https://nodejs.org/en/download/package-manager).
- Finally, to program, build and run the client software, you will need some rudementary Rust knowledge, along with the Rust compiler. For more information and an example program that should compile with no modifications, view the documentation for the [client library](https://crates.io/crates/NOSHP-Client).
- This system does not have many system dependencies, however one thing you will need is the protobuf compiler. Installation instructions can be found [here](https://grpc.io/docs/protoc-installation/), it is usually included in Linux package managers.

Once you have met these requirements, you can probably build and run all parts of the software (you may still be missing dependencies such as GCC, required to build any Rust program). Here are instructions to run every part of the program:
- To build the server, you can use the command, while in the 'backend' directory: `cargo build -r --bin server`. This will build the server in release mode. You can then find the compiled binary in the 'backend/target/release' directory, or you can run it using `cargo run -r --bin server` (note, that to use command line arguments from here you will need to seperate them with a '--', for example `cargo run -r --bin server -- --json-frontend`).
- Building the CLI frontend is very similar. While in the 'backend' directory run the command `cargo build -r --bin frontend`. The binary will then be again be located in the 'backend/target/release' directory. Note, that to run this, you will need to include the server's ip as a command line argument. For example: `./frontend -s 192.168.1.1:2302`.
- To build and run the web frontend, just execute the 'serve_frontend.sh' script (this will not execute on Windows and might need to be modified). Select yes to any questions asked while it is running. Once the server is running it should be available from the browser at 'localhost:5173'.
- Building a client is very simple. Simply run `cargo add NOSHP-Client tokio` in a Rust project, this should add the client library and all its dependencies to your current Rust project. From there follow instructions in the client library documentation to build a client. 
- **NOTE: YOU SHOULD PROBABLY NOT DO THIS.** Building the client library from scratch is a little more complicated. Note that you do **not** need to do this, unless attempting to edit the actual library code. While in the client_library directory, execute the 'cp_proto_here.sh' script. This will move all the protobuf files into the current directory. From there you can build the library using `cargo build -r`.


## Project Layout
    ├── backend            Contains server and CLI frontend code
    │   ├── src
    ├── client_library     Contains the code for the Client Library
    │   └── src
    ├── proto              Contains all protobuf API defintions
    │   ├── frontend      
    │   └── iot
    ├── testing            Contains all programs and data used during testing
    │   ├── ddos_noshp_server
    │   ├── spawn_noshp_clients
    │   └── test_data
    └── vue_frontend       Contains all web-frontend code
        ├── public
        └── src
