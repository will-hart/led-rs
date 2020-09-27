# LEd-rs

This is a simple parser for the LEd file format written in Rust. See
https://deepnight.net/tools/led-2d-level-editor/.

It is written to support the app version `0.2.1`, JSON version 1.

No guarantees about compatibilty or ongoing support are made :D

## Limitations

- `defs` are not parsed
- Array type `entityInstance.fieldInstances` are not parsed

## Usage

The library isn't currently on `crates.io`. Add to your `cargo.toml`

```toml
led-rs = { git = "https://github.com/will-hart/led-rs" }
```

See the `examples` folder for some example usage or run the examples with

```bash
cargo run --example parse_sample
```

## LICENSE

MIT / Apache
