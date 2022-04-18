# dfhack_proto

[![Crates.io](https://img.shields.io/crates/v/dfhack-proto)](https://crates.io/crates/dfhack-proto)
[![docs.rs](https://img.shields.io/docsrs/dfhack-proto)](https://docs.rs/dfhack-proto)
![Crates.io](https://img.shields.io/crates/l/dfhack-proto)

This subcomponent of [dfhack_remote](https://docs.rs/dfhack-remote/) contains all the generated code
for interacting with DFHack remote API.

It contains two main modules:

 - [messages] exposes the protobuf messages. This is the standard generated protobuf.
 - [plugins] exposes the plugins and their RPC. DFHack is not using gRPC and this is a custom implementation

Internally these two modules are created under the `generated` module.

All the plugins are built from a struct implementing [ProtocolTrait]. This trait
should implement the actual data exchange.

The code is regenerated under the condition that the environment variable `DFHACK_REGEN` is set.

This crates generates all its code directly in a source subfolder. It would likely
be cleaner to use the $OUT_DIR, or macro to do this job.
