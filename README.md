# pelecan
A framework for extending Mumble servers with Rust (based on [murmur_grpc](https://github.com/chili-b/murmur_grpc)).

### Required Software
* [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* bash

### Usage

* [Server configuration instructions](server_configuration.md)
* [Module creation instructions](writing_modules.md) (coming soon)

#### Overview

Run `setup.sh` to create the required directories. The `servers` directory is where configuration files
for connections to Mumble servers (physical and virtual) are to be placed. The `modules` directory is where
the source code and configuration files for rust modules are to be placed. After everything is set up, run
`compile.sh` which will produce a binary at `bin/pelecan`.
