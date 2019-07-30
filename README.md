# plugin-system

A Rust project plugin management system using Cargo

## Structure
- `plugin-runtime-codegen`: Complex macro definitions for `plugin-runtime`.
- `plugin-runtime`: Loaded by the core and plugins in the runtime.
- `plugin-system`: Loaded by the wrapper crate.
- `example/usage`: Example wrapper crate.
- `example/core`: Example core.
- `example/plugins/foo`: Example plugin, not directly used by `example/usage` but depended on by `example/plugins/bar`.
- `example/plugins/bar`: Example plugin, depends on `example/plugins/foo`.

When testing the example, set the `PS_LOCAL_RUNTIME` environment variable (to anything) so that it depends on the `plugin-runtime` crate in this repo rather than the one from the crates.io registry. Otherwise, compilation may failure due to multiple versions of `plugin-runtime` being used.
