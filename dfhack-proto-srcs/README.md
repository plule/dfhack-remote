# dfhack_proto_srcs

This subcomponent of [dfhack_remote](https://docs.rs/dfhack-remote/) contains the protobuf files
extracted from [DFHack](https://github.com/DFHack/dfhack).

It is licensed under ZLib, just like DFHack.

## Regenerating the protos

To regenerate the protos in the source directory, run `cargo build` with the `DFHACK_DOWNLOAD` set to `1`.

To target a different `DFHACK` version, set the `DFHACK_ZIP` environment variable to an url.
For example `https://github.com/DFHack/dfhack/archive/refs/heads/develop.zip`.
