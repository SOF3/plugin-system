# plugin-system

A Rust project plugin management system using Cargo

## What is this?
The plugin system can be deployed in user-end software with a "core" component and optional "plugins" that add features to the core.

Traditionally, when the user installs the software, the user directly downloads the core software, then downloads optional .dll plugin files that are dynamically linked to the core.

An improved variant is where the core software provides an interface to download .dll plugins, saving the user from the trouble of dragging and dropping files.

This plugin-system further improves the mechanism by completely removing dynamic linking. When the software is downloaded, the user just installs the Rust toolchain plus this plugin system. Then the user edits a file that declares the plugins required. Then, when the software starts, a temporary crate is generated in a directory called "stage" (`$STAGE_DIR`), and `cargo run` is automatically run inside that child process. This approach has several advantages:
- All binaries are compiled locally, reducing the chance of malware distribution.
- Libraries are statically linked, so each library is only downloaded and compiled once (unless they have incompatible versions).
- Libraries are statically linked, so multiple plugins can interoperate flawlessly.
- \[Insert all advantages of open-source software here\]

But there are some drawbacks:
- In the first time or when the user changes the plugin list, recompilation takes a long time.
- Distributed plugins must be open-source.
- \[Insert all arguments against open-source software here\]

## Structure
- `plugin-runtime-codegen`: Complex macro definitions for `plugin-runtime`.
- `plugin-runtime`: Loaded by the core and plugins in the runtime.
- `plugin-system`: Loaded by the wrapper crate.
- `example/usage`: Example wrapper crate.
- `example/core`: Example core.
- `example/plugins/foo`: Example plugin, not directly used by `example/usage` but depended on by `example/plugins/bar`.
- `example/plugins/bar`: Example plugin, depends on `example/plugins/foo`.

When testing the example, set the `PS_LOCAL_RUNTIME` environment variable (to anything) so that it depends on the `plugin-runtime` crate in this repo rather than the one from the crates.io registry. Otherwise, compilation may failure due to multiple versions of `plugin-runtime` being used.
