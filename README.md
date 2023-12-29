# IoT_Platform
### Todo before v0.1:
- [X] Add config files for clients
- [X] Add optional capabilities (capability 'turn on' should only be available if device is in state 'off')
- [X] Add custom capabilities, using strings instead of an enum
- [X] Add ability to hand client callback functions to be called when certain capabilities are triggered
- [X] Get client working on a Raspberry Pi
- [ ] Write a proper README

Required to run (will be added to a DockerFile later):
```
sudo apt install xorg-dev libxcb-shape0-dev libxcb-xfixes0-dev clang avahi-daemon libavahi-client-dev protobuf-compiler
```

### Possible future additions
- Using [Clap](https://docs.rs/clap/latest/clap/) for command line arguments
