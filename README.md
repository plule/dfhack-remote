# dfhack_remote

[![Github](https://img.shields.io/badge/github-plule%2Fdfhack--remote-8da0cb?style=flat-square)](https://github.com/plule/dfhack-remote)
[![Crates.io](https://img.shields.io/crates/v/dfhack-remote?style=flat-square)](https://crates.io/crates/dfhack-remote)
[![docs.rs](https://img.shields.io/docsrs/dfhack-remote?style=flat-square)](https://docs.rs/dfhack-remote)
![Crates.io](https://img.shields.io/crates/l/dfhack-remote?style=flat-square)


`dfhack_remote` is a library allowing users to interact with Dwarf Fortress using a remote API.

[Dwarf Fortress] is a video game by Bay 12 Games.
[DFHack] is a fan made mod for Dwarf Fortress adding many
features to the game.
One of these feature is a [remote API] enabling
the remote control of certain Dwarf Fortress features.
This crates is a client for this remote API, enabling rust tool developers to
interact with Dwarf Fortress.

## Examples

Displaying some information about the current world.

```no_run
let mut client = dfhack_remote::connect().unwrap();
let world_info = client.core().get_world_info().unwrap();

println!(
    "Welcome to {} ({})",
    world_info.world_name.last_name(),
    world_info.world_name.english_name()
);
```

Pausing or unpausing the game
``` no_run
let mut client = dfhack_remote::connect().unwrap();
let status = client.remote_fortress_reader().get_pause_state().unwrap();
client.remote_fortress_reader().set_pause_state(!status).unwrap();
```

The generated API is mostly a direct translation from the raw RPC,
and as such is quite verbose.

It includes some minor syntaxic sugar such as omitting `EmptyMessage` inputs.

## The DFHack API

The DFHack [remote API] relies on protobuf for serializing messages.
This means that all the input and output of each message is relying on protobuf code generation to create
a type-safe experience.

`dfhack_remote` is using the crates [protobuf] and [protobuf-codegen-pure] for the protobuf code generation.

However, DFHack is not using `gRPC` for representing the remote calls, but a specific protocol.
The definitions of RPC of DFHack is described with comments inserted in the `.proto` files. In order
to bind all the RPC, this crates is generating all the API entrypoints directly from the [DFHack] source code.

### Building for a different DFHack version

The DFHack source used for code generation can be controlled at build time using the `DFHACK_ZIP_URL`
environment variable. For example, setting this environment variable to `https://github.com/DFHack/dfhack/archive/refs/heads/develop.zip`
at build time would target the latest changes included in DFHack.

In order to trigger the download and source regeneration, `DFHACK_REGEN` and `DFHACK_DOWNLOAD` must also be set to `1`.


## Crate structure

This crate main entrypoint is the [connect] function.

The code is split into three crates:

- `dfhack_proto_srcs` downloads and extract the proto from the DFHack source code at build time.
- `dfhack_proto` builds the protobuf messages and plugin structs containing the RPC
- `dfhack_remote` is the main entrypoint. It implements the actual protocol and exposes the features.

For ease of use, `dfhack_remote` reexports all the generated code from `dfhack_proto`.

Internally, the `message` module handles the serialization/deserialization logic that sits on top of protobuf,
and the `channel` module handles the exchange flow.

## Testing

Most of the tests require the ability to communicate with Dwarf Fortress.
This can be enabled by setting the environment variable `DF_EXE` to the Dwarf Fortress executable.
Once this is setup, you can run these tests with the `test-with-df` feature enabled:

```txt
cargo test --features test-with-df
```

This will run the first save of this Dwarf Fortress installation. You should prepare a dedicated save with a pocket world for that purpose.

## Release

The versioning is done through [cargo release]. `cargo release --workspace --execute`, to release current version
and switch to next patch, or `cargo release [level] --workspace --execute` (minor, major).


[Dwarf Fortress]: http://www.bay12games.com/dwarves/
[DFHack]: https://docs.dfhack.org/en/stable/
[remote API]: https://docs.dfhack.org/en/stable/docs/Remote.html
[protobuf]: https://crates.io/crates/protobuf
[protobuf-codegen-pure]: https://crates.io/crates/protobuf-codegen-pure
[cargo release]: https://github.com/crate-ci/cargo-release