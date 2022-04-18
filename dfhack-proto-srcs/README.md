# dfhack_proto_srcs

[![Crates.io](https://img.shields.io/crates/v/dfhack-proto-srcs)](https://crates.io/crates/dfhack-proto-srcs)
[![docs.rs](https://img.shields.io/docsrs/dfhack-proto-srcs)](https://docs.rs/dfhack-proto-srcs)
![Crates.io](https://img.shields.io/crates/l/dfhack-proto-srcs)

This subcomponent of [dfhack_remote](https://docs.rs/dfhack-remote/) contains the protobuf files
extracted from [DFHack](https://github.com/DFHack/dfhack).

It is licensed under ZLib, just like DFHack.

## Regenerating the protos

To regenerate the protos in the source directory, run `cargo build` with the `DFHACK_DOWNLOAD` set to `1`.

To target a different `DFHACK` version, set the `DFHACK_ZIP` environment variable to an url.
For example `https://github.com/DFHack/dfhack/archive/refs/heads/develop.zip`.
