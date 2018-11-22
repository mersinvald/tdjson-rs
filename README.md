## TDLIB Json Client for Rust

### Preparing bindgen

Please follow the steps described in the [tdlib-sys](https://github.com/mersinvald/tdjson-sys) repo to setup FFI bindings generation.

### Usage

Add `tdjson` to your `Cargo.toml` dependency list
```toml
tdjson = "0.2"
```

And let the Cargo do it's magic!
```bash
cargo build
```