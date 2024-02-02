# Backend
This project contains the server code, used to host the backend of the IOT System. This project also 
contains the code to the CLI version of the frontend.

# Building
You can build the server using the command:
```Cargo build -r --bin server```

You can build the CUI frontend using the command:
```Cargo build -r --bin frontend```

# Running
The default port for the server is [port #2302](https://findports.com/article/halo), you can change this
easily in the server.rs file. In the future a commandline argument will be made available to change this at
runtime, rather than at compile time.

If you are running a frontend, that requires JSON instead of GRPC for requests (for example a web frontend),
you can use the commandline argument --json-frontend. This will start a JSON proxy, that will accept JSON
requests and forward them as GRPC requests to the Server. The response will then be send back as JSON.

Note: do not use the client contained within this project. It is legacy code and is broken. :disappointed: 
