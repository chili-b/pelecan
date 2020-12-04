<h1> <img align="left" width="100" height="100" src="pelecan.png">
  pelecan 
</h1>

A framework for extending Mumble servers with Rust.
<sub><sup>(based on [murmur_grpc](https://github.com/chili-b/murmur_grpc))</sup></sub>

### Build Requirements
* [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
  * make sure you have rustfmt as well (can be installed with `rustup component add rustfmt`)
* bash (to run the `compile.sh` script)

Pelecan outputs a single mostly statically linked binary once you've compiled it with your given modules.

### Usage

* [Server configuration instructions](server_configuration.md)
* [Module creation instructions](writing_modules.md) (WIP)

### Overview

Run `setup.sh` to create the required directories. The `servers` directory is where configuration files
for connections to Mumble servers (physical and virtual) are to be placed. The `modules` directory is where
the source code and configuration files for rust modules are to be placed. After everything is set up, run
`compile.sh` which will produce a binary at `bin/pelecan`.
