# Client Library for Nik's Open-source Smart Home Platform 
This Rust library is meant as a resource to easily write code to interact with the [NOSHP server](https://github.com/niknik3610/IoT_Platform). This is mainly meant for Smart-Home devices.  

### Important Links
- [Github](https://github.com/niknik3610/IoT_Platform)
- [Crates.io](https://crates.io/crates/NOSHP-Client)
- [Documenation](https://docs.rs/NOSHP-Client/0.1.2/NOSHP_Client/)
- [Example Client](https://github.com/niknik3610/Example-Iot-Device)

### Building 
Add the library to your project using the crates.io link above. Then create your device, or view the documentation for an example device.

Once you're done, run the command:

```cargo build -r```

Then install any dependencies you are missing.

### Running
To run the client library you will need to set the IP of your server. How you get this is up to you. I personally recommend setting a static IP for your server, that way you won't need to change this everytime this changes. However if you do not want to do this, the server IP is printed at server startup. 

**Remember, the server IP needs to be prefixed with "HTTP://", otherwise it will not work**

### Notes
- At the moment you will need to have the protobuf compiler installed (instructions can be found [here](https://grpc.io/docs/protoc-installation/)).
